use hex;
use hex::FromHexError;
use iocore::Exception;

#[derive(Debug)]
pub enum Error {
    HexDecodeError(FromHexError),
    IOError(std::io::Error),
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
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOException(e) => write!(f, "I/O Core Exception: {}", e),
            Error::IOError(e) => write!(f, "IO Error: {}", e),
            Error::HexDecodeError(e) => write!(f, "Hex Decode Exception: {}", e),
        }
    }
}
