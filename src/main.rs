use clap::{Parser, ValueEnum};
use copypasta::{ClipboardContext, ClipboardProvider};

use carbon14::HochTable;
use hex;
use hex::FromHexError;
use iocore::plant::{PathRelative};
use iocore::{rsvfilematch, absolute_path};
use iocore::Exception;
use serde_yaml;

#[derive(Debug)]
pub enum Error {
    HexDecodeError(FromHexError),
    IOException(iocore::Exception),
}

impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Self {
        Error::HexDecodeError(e)
    }
}

impl From<Exception> for Error {
    fn from(e: Exception) -> Self {
        Error::IOException(e)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOException(e) => write!(f, "I/O Core Exception: {}", e),
            Error::HexDecodeError(e) => write!(f, "Hex Decode Exception: {}", e),
        }
    }
}

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

    #[clap(short, long, value_enum)]
    pub format: Option<Format>,
}

pub fn main() -> Result<(), Error> {
    let args = Argv::parse();
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
        ) {
            Ok(table) => {
                match serde_yaml::to_string(&table) {
                    Ok(display) => {
                        println!("{}", display);
                        let msg = format!(
                            "failed to copy checksum data to clipboard for file: {}",
                            path.display()
                        );
                        let mut ctx = ClipboardContext::new().expect(&msg);
                        ctx.set_contents(display.clone()).expect(&msg);
                    }
                    Err(e) => eprintln!("{}", e),
                };
                return true;
            }
            Err(e) => {
                eprintln!("\x1b[1;38;5;160m{}\x1b[0m", e);
                return false;
            }
        }
    })?;
    Ok(())
}
