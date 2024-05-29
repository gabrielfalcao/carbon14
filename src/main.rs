// $$'""""'$$$                   $$                         $$$  $$   $$
// $' .$$$. `$                   $$                          $$  $$   $$
// $  $$$$$$$$ .$$$$$$. $$$$$$$. $$$$$$$. .$$$$$$. $$$$$$$.  $$  $$$$$$$
// $  $$$$$$$$ $$'  `$$ $$'  `$$ $$'  `$$ $$'  `$$ $$'  `$$  $$       $$
// $. `$$$' .$ $$.  .$$ $$       $$.  .$$ $$.  .$$ $$    $$  $$       $$
// $$.     .$$ `$$$$$$$ $$       $$$$$$$' `$$$$$$' $$    $$ $$$$      $$
// $$$$$$$$$$$
use carbon14::Error;
use carbon14::HochTable;
use clap::{Parser};
use iocore::{walk_nodes, Path};
use serde::Serialize;
use serde_yaml;
use std::borrow::BorrowMut;
use std::io::{stdin, IsTerminal, Read, BufRead};
use carbon14::{clipboard_lines, stdin_lines};

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
    pub fn writer(&self, log_err: bool) -> Option<FWriter> {
        let defer_write = self.defer_write.clone();
        self.output_file.map(|p| FWriter::new(p, defer_write, log_err))
    }
    pub fn objects(&self) -> Vec<String> {
        if self.targets.len() > 0 {
            self.targets.iter().filter(|s| !s.is_empty()).map(|s|s.clone()).collect()
        } else {
            stdin_lines().or(clipboard_lines()).unwrap_or(Vec::new())
        }
    }
    pub fn stdin(&self) -> Option<Vec<String>> {
        let mut lines = Vec::new();
        for line in stdin().lock().lines() {
            match line {
                Ok(line) => lines.push(line),
                Err(y) => {
                    eprintln!("error: {}", y);
                    break;
                }
            }
        }
        (lines.len() > 0).then_some(lines)
    }
    pub fn clipboard(&self) -> Option<Vec<String>> {
        match ClipboardContext::new() {
            Ok(mc) => match c.get_contents() {
                Ok(t) => return Some(
                    t.lines()
                        .filter(|t| !t.trim().is_empty())
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>(),
                ),
                Err(y) => {
                    eprintln!("error: {}", y);
                }
            },
            Err(y) => {
                eprintln!("error: {}", y);
            }
        }
        None
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
        Carbon14 {
            tables,
            cli,
        }
    }
    pub fn scan(&mut self) -> Vec<HochTable> {
        let mut tables = Vec::<HochTable>::new();
        let mut
        for target in c14.cli.objects() {
            if Path::from(target).exists() {
                let meta = (!args.hexonly).then_some(path.to_string());
                walk_nodes(
                    vec![target.to_string()],
                    &mut |p| {
                        let table = HochTable::new(meta).cs(path.read_bytes());
                        tables.push(table);
                        writer.append(&table).unwrap_or(());
                        p.is_dir()
                    },
                    &mut |_path, _exc| None,
                    None,
                )?;
            } else {
                let meta = Some(target.clone());
                let table = HochTable::new(meta).cs(path.read_bytes());
                tables.push(table);
                writer.append(&table).unwrap_or(());
                p.is_dir()
            }
        }
    }
    pub fn launch() -> Result<(), Error> {
        let mut c14 = Carbon14::new();
        for target in c14.cli.objects() {
            if Path::from(target).exists() {
                let meta = (!args.hexonly).then_some(path.to_string());
                walk_nodes(
                    vec![target.to_string()],
                    &mut |p| {
                        let table = HochTable::new(meta).cs(path.read_bytes());
                        tables.push(table);
                        writer.append(&table).unwrap_or(());
                        p.is_dir()
                    },
                    &mut |_path, _exc| None,
                    None,
                )?;
            } else {
                let meta = Some(target.clone());
                let table = HochTable::new(meta).cs(path.read_bytes());
                tables.push(table);
                writer.append(&table).unwrap_or(());
                p.is_dir()
            }
        }
        if let Some(output_file) = output_file {
            open_write(&output_file)?.write(serde_yaml::to_string(&table_list)?.as_bytes())?;
        }
        Ok(())
    }
}
pub fn main() -> Result<(), Error> {
    Carbon14::launch()
}

pub trait HochSchreiber {
    fn append(&mut self, data: impl Serialize) -> Result<FWriter, Error> ;
    fn encode(&self, data: impl Serialize) -> Result<Vec<u8>, Error> ;
    fn handle(&self, e: impl Into<Error>) -> Result<(), Error> ;
    fn finish(&mut self) -> Result<(), Error> ;
}

#[derive(Debug, Clone)]
struct FWriter {
    path: Path,
    defer_write: bool,
    log_err: bool,
    buffer: Vec<u8>,
}

impl FWriter {
    pub fn new(path: Path, defer_write: bool, log_err: bool) -> FWriter {
        FWriter {
            path,
            defer_write,
            log_err,
            buffer: Vec::new(),
        }
    }
    pub fn append(&mut self, data: impl Serialize) -> Result<FWriter, Error> {
        match self.encode(data) {
            Ok(bytes) => {
                if self.defer_write {
                    self.buffer.extend_from_slice(&bytes);
                } else {
                    if let Err(y) = self.path.append(&bytes).map(|_| ()) {
                        self.handle(y)?;
                    }
                }
            }
            Err(y) => self.handle(y)?,
        }
        Ok(self.clone())
    }
    fn encode(&self, data: impl Serialize) -> Result<Vec<u8>, Error> {
        let mut bytes = b"\n---\n".to_vec();
        let y = serde_yaml::to_string(&data).map_err(|e| {
            Error::Error(format!("encoding yaml destined to {}: {}", &self.path, e))
        })?;
        bytes.extend_from_slice(y.as_bytes());
        Ok(bytes)
    }
    fn handle(&self, e: impl Into<Error>) -> Result<(), Error> {
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
            Err(Error::Error(format!(
                "writing data into {}: empty buffer",
                &self.path
            )))
        } else {
            self.path.write(&self.buffer)?;
            Ok(())
        }
    }
}
