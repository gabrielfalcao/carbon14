// $$'""""'$$$                   $$                         $$$  $$   $$
// $' .$$$. `$                   $$                          $$  $$   $$
// $  $$$$$$$$ .$$$$$$. $$$$$$$. $$$$$$$. .$$$$$$. $$$$$$$.  $$  $$$$$$$
// $  $$$$$$$$ $$'  `$$ $$'  `$$ $$'  `$$ $$'  `$$ $$'  `$$  $$       $$
// $. `$$$' .$ $$.  .$$ $$       $$.  .$$ $$.  .$$ $$    $$  $$       $$
// $$.     .$$ `$$$$$$$ $$       $$$$$$$' `$$$$$$' $$    $$ $$$$      $$
// $$$$$$$$$$$
use std::io::{stdout, Write};

use carbon14::{clipboard_lines, stdin_lines, Error, HochTable};
use clap::Parser;
use iocore::{walk_nodes, Exception, Path};
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
}
impl Cli {
    pub fn writer(&self) -> FWriter {
        let defer_write = self.defer_write.clone();
        let log_err = defer_write;
        FWriter::new(self.output_file.clone(), defer_write, log_err)
    }
    pub fn objects(&self) -> Result<Vec<String>, Error> {
        let objects = if self.targets.len() > 0 {
            self.targets.iter().filter(|s| !s.is_empty()).map(|s| s.clone()).collect()
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
    pub tables: Vec<HochTable>,
    pub cli: Cli,
}
impl Carbon14 {
    pub fn new() -> Carbon14 {
        let tables = Vec::<HochTable>::new();
        let cli = Cli::parse();
        Carbon14 { tables, cli }
    }
    pub fn scan(&mut self) -> Result<FWriter, Error> {
        let mut writer = self.cli.writer();
        for target in self.cli.objects()? {
            if Path::from(&target).exists() {
                let meta = (!self.cli.hexonly)
                    .then_some(writer.path().map(|p| p.to_string()))
                    .unwrap_or(None);
                walk_nodes(
                    vec![target.to_string()],
                    &mut |p| {
                        match Path::from(&target).read_bytes() {
                            Ok(bytes) => {
                                let table = HochTable::new(meta.clone()).cs(bytes);
                                self.tables.push(table.clone());
                                writer.append(&table).and(Ok(())).unwrap_or(());
                            },
                            Err(e) => {
                                eprintln!("error reading {}: {}", &target, e);
                            },
                        }
                        p.is_dir()
                    },
                    &mut |path, exc| {
                        eprintln!("error reading {}: {}", path, exc);
                        None
                    },
                    None,
                )?;
            } else {
                let meta = Some(target.clone());
                let table = HochTable::new(meta).cs(target.as_bytes().to_vec());
                self.tables.push(table.clone());
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
        })?;
        bytes.extend_from_slice(y.as_bytes());
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
            if !self.defer_write {
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
