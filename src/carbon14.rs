// $$'""""'$$$\                  $$\\                       $$$  $$\  $$\\\
// $' .$$$. `$\                  $$\\                        $$\ $$\  $$\\\
// $  $$$$$$$$\.$$$$$$. $$$$$$$. $$$$$$$. .$$$$$$. $$$$$$$.  $$\ $$$$$$$\\\
// $  $$$$$$$$\$$'  `$$ $$'\\`$$ $$'  `$$ $$'  `$$ $$'  `$$  $$\\ \\\\$$\\\
// $. `$$$' .$\$$.  .$$ $$\      $$.  .$$ $$.  .$$ $$\   $$  $$\\     $$\\\
// $$.     .$$\`$$$$$$$ $$\      $$$$$$$'\`$$$$$$' $$\   $$ $$$$\     $$\\\
// $$$$$$$$$$$\ \\\\\\\ \\\\     \\\\\\\\\ \\\\\\\ \\\\  \\ \\\\\     \\\\\
// \\\\\\\\\\\\\ \\\\\\\ \\\\     \\\\\\\\\ \\\\\\\ \\\\  \\ \\\\\     \\\\\
// https://en.wikipedia.org/wiki/Radiocarbon_dating
use std::io::{stdout, Write};

use carbon14::{clipboard_lines, stdin_lines, Error, HochTable};
use clap::Parser;
use iocore::{walk_dir, Exception, Path};
use serde::Serialize;
use serde_yaml;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    targets: Vec<String>,

    #[clap(short = 'x', long)]
    pub hexonly: bool,

    #[clap(short, long)]
    pub clipboard: bool,

    #[clap(short, long)]
    pub output: bool,

    #[clap(short = 'f', long, requires = "output")]
    pub output_file: Option<Path>,

    #[clap(short, long, requires = "output_file")]
    pub defer_write: bool,

    #[clap(short, long)]
    pub log_err: bool,
}
impl Cli {
    pub fn writer(&self) -> FWriter {
        FWriter::new(self.output_file.clone(), self.defer_write, self.log_err)
    }
    pub fn objects(&self) -> Result<Vec<String>, Error> {
        let targets: Vec<String> = self.targets.iter().filter(|s| !s.is_empty()).map(|s| Path::from(s)).filter(|p|p.exists()).map(|p|p.to_string()).collect();
        let objects = if targets.len() > 0 {
            targets
        } else {
            stdin_lines().or(clipboard_lines()).unwrap_or(Vec::new())
        };
        if objects.is_empty() {
            Err(Error::Error(format!("no targets, try --help")))
        } else {
            Ok(objects)
        }
    }
}
struct Carbon14 {
    pub cli: Cli,
}
impl Carbon14 {
    pub fn new() -> Carbon14 {
        let cli = Cli::parse();
        Carbon14 { cli }
    }
    pub fn scan(&mut self) -> Result<FWriter, Error> {
        let mut writer = self.cli.writer();
        for target in self.cli.objects()? {
            let target = Path::from(&target);
            if target.exists() {
                let target = target.canonicalize()?.relative_to_cwd();
                if target.is_file() {
                    match target.read_bytes() {
                        Ok(bytes) => {
                            let meta = (!self.cli.hexonly)
                                .then_some(Some(target.to_string()))
                                .unwrap_or(None);

                            let table = HochTable::new(meta.clone()).cs(bytes);
                            writer.append(&table).and(Ok(())).unwrap_or(());
                        },
                        Err(e) => {
                            eprintln!("error reading {}: {}", &target, e);
                        },
                    }
                } else if target.is_dir() {
                    walk_dir(
                        &target,
                        &mut |path: &Path| {
                            if path.is_file() {
                                match path.read_bytes() {
                                    Ok(bytes) => {
                                        let meta = (!self.cli.hexonly)
                                            .then_some(Some(path.to_string()))
                                            .unwrap_or(None);

                                        let table = HochTable::new(meta.clone()).cs(bytes);
                                        writer.append(&table).and(Ok(())).unwrap_or(());
                                    },
                                    Err(e) => {
                                        eprintln!("error reading {}: {}", &path, e);
                                    },
                                }
                            }
                            true
                        },
                        &mut |path, exc| {
                            eprintln!("error reading {}: {}", path, exc);
                            None
                        },
                        None,
                        None,
                    )?;
                } else {
                    continue;
                }
            } else {
                let target = target.to_string();
                let meta = Some(target.clone());
                let table = HochTable::new(meta).cs(target.as_bytes().to_vec());
                writer.append(&table)?;
            }
        }
        Ok(writer)
    }
    pub fn launch() -> Result<(), Error> {
        let mut c14 = Carbon14::new();
        let mut writer = c14.scan()?;
        writer.finish()
    }
}
pub fn main() {
    if let Err(des) = Carbon14::launch() {
        eprintln!("{}", des);
        std::process::exit(0o11);
    }
}

#[derive(Debug, Clone)]
pub struct FWriter {
    path: Option<Path>,
    defer_write: bool,
    log_err: bool,
    buffer: Vec<u8>,
}

impl FWriter {
    pub fn new(path: Option<Path>, defer_write: bool, log_err: bool) -> FWriter {
        FWriter {
            path,
            defer_write,
            log_err,
            buffer: Vec::new(),
        }
    }
    pub fn append(&mut self, data: impl Serialize) -> Result<FWriter, Error> {
        match self.encode(data) {
            Ok(bytes) =>
                if self.defer_write {
                    self.buffer.extend_from_slice(&bytes);
                } else {
                    if let Err(y) =
                        self.path.clone().map(|path| path.append(&bytes)).unwrap_or_else(|| {
                            stdout().write(&bytes).map_err(|e| Exception::from(e))
                        })
                    {
                        self.handle(y)?;
                    }
                },
            Err(y) => self.handle(y)?,
        }
        Ok(self.clone())
    }
    pub fn encode(&self, data: impl Serialize) -> Result<Vec<u8>, Error> {
        let mut bytes = b"\n---\n".to_vec();
        let y = serde_yaml::to_string(&data).map_err(|e| {
            Error::Error(format!("encoding yaml destined to {}: {}", self.output(), e))
        })?.trim().to_string();
        bytes.extend_from_slice(y.as_bytes());
        bytes.extend_from_slice("#\t∎".as_bytes());
        Ok(bytes)
    }
    pub fn handle(&self, e: impl Into<Error>) -> Result<(), Error> {
        let e: Error = e.into();
        if self.log_err {
            eprintln!("error: {}", e);
            Ok(())
        } else {
            Err(e)
        }
    }
    pub fn finish(&mut self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            if self.defer_write {
                Err(Error::Error(format!("writing data to {}: empty buffer", self.output())))
            } else {
                Ok(())
            }
        } else {
            let buffer = self.buffer.clone();
            self.path
                .clone()
                .map(|path| path.write(&buffer).map(|_| buffer.len()))
                .unwrap_or_else(|| stdout().write(&buffer).map_err(|e| Exception::from(e)))?;
            Ok(())
        }
    }
    pub fn path(&self) -> Option<Path> {
        self.path.clone()
    }
    fn output(&self) -> String {
        self.path.clone().map(|path| path.to_string()).unwrap_or("stdout".to_string())
    }
}

// pub trait HochSchreiber {
//     fn append(&mut self, data: impl Serialize) -> Result<FWriter, Error>;
//     fn encode(&self, data: impl Serialize) -> Result<Vec<u8>, Error>;
//     fn handle(&self, e: impl Into<Error>) -> Result<(), Error>;
//     fn finish(&mut self) -> Result<(), Error>;
// }
