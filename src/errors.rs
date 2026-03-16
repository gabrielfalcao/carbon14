use hex;
use hex::FromHexError;
use iocore::Error as IOCoreError;
use serde_yaml;

#[derive(Debug)]
pub enum Error {
    YamlEncodeError(serde_yaml::Error),
    Error(String),
    HexDecodeError(FromHexError),
    IOError(std::io::Error),
    IOCoreError(IOCoreError),
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::YamlEncodeError(e)
    }
}
impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Self {
        Error::HexDecodeError(e)
    }
}

impl From<IOCoreError> for Error {
    fn from(e: IOCoreError) -> Self {
        Error::IOCoreError(e)
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
            Error::IOCoreError(e) => write!(f, "I/O Core Error: {}", e),
            Error::IOError(e) => write!(f, "IO Error: {}", e),
            Error::Error(e) => write!(f, "{}", e),
            Error::YamlEncodeError(e) => write!(f, "YamlEncode Error: {}", e),
            Error::HexDecodeError(e) => write!(f, "Hex Decode Error: {}", e),
        }
    }
}
