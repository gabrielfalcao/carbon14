use clap::{Parser, ValueEnum};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::Write;
use carbon14::HochTable;
use carbon14::Error;
use iocore::plant::PathRelative;
use iocore::{absolute_path, rsvfilematch, open_write};
use serde_yaml;
use std::borrow::BorrowMut;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    Plain,
    Csv,
    Toml,
    Json,
    Yaml,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Argv {
    pub files: Vec<String>,

    #[clap(short = 'x', long)]
    pub hexonly: bool,

    #[clap(short, long)]
    pub clipboard: bool,

    #[clap(short, long, value_enum)]
    pub format: Option<Format>,

    #[clap(short, long)]
    pub output_file: Option<String>,
}

pub fn main() -> Result<(), Error> {
    let args = Argv::parse();
    let clipboard = args.clipboard;
    let hexonly = args.hexonly;
    let output_file = args.output_file.clone();
    let mut table_list = Vec::<HochTable>::new();
    let tables: &mut Vec<HochTable> = table_list.borrow_mut();
    rsvfilematch(args.files, move |path| {
        let data = match std::fs::read(&path) {
            Ok(data) => data.to_vec(),
            Err(_) => return false,
        };
        match HochTable::new(
            format!(
                "{}",
                path.relative_wherewith(&absolute_path(".").unwrap())
                    .display()
            ),
            &data,
            hexonly,
        ) {
            Ok(table) => {
                if args.output_file == None {
                match serde_yaml::to_string(&table) {
                    Ok(display) => {
                        println!("{}", display);
                        if clipboard {
                            let msg = format!(
                                "failed to copy checksum data to clipboard for file: {}",
                                path.display()
                            );
                            let mut ctx = ClipboardContext::new().expect(&msg);
                            ctx.set_contents(display.clone()).expect(&msg);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                };
                }
                tables.push(table);
                return true;
            }
            Err(e) => {
                eprintln!("\x1b[1;38;5;160m{}\x1b[0m", e);
                return false;
            }
        }
    })?;
    if let Some(output_file) = output_file {
        open_write(&output_file)?.write(serde_yaml::to_string(&table_list)?.as_bytes())?;
    }
    Ok(())
}
