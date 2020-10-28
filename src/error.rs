use ioe;
use std::io::Error as IoError;
use toml::de::Error as TomlError;

pub type ServerResult<T> = std::result::Result<T, ServerError>;

#[derive(Clone, Debug)]
pub enum ServerError {
    IoError(ioe::IoError),
    TomlError(TomlError),
}

impl From<IoError> for ServerError {
    fn from(err: IoError) -> Self { Self::IoError(ioe::IoError::from(err)) }
}

impl From<ioe::IoError> for ServerError {
    fn from(err: ioe::IoError) -> Self { Self::IoError(err) }
}

impl From<TomlError> for ServerError {
    fn from(err: TomlError) -> Self { Self::TomlError(err) }
}
