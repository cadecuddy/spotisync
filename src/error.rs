use serde_json::Error as SerdeJsonError;
use std::{
    fmt::{self, Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    ParseError(SerdeJsonError),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ConfigError::IoError(ref err) => write!(f, "IO error: {}", err),
            ConfigError::ParseError(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

impl From<SerdeJsonError> for ConfigError {
    fn from(err: SerdeJsonError) -> Self {
        ConfigError::ParseError(err)
    }
}
