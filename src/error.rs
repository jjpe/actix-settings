use ioe;
use std::env::VarError;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::num::ParseIntError;
use std::str::ParseBoolError;
use toml::de::Error as TomlError;

macro_rules! InvalidValue {
    (expected: $expected:expr, got: $got:expr,) => {
        crate::Error::InvalidValue {
            expected: $expected,
            got: $got.to_string(),
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    EnvVarError(VarError),
    FileExists(PathBuf),
    InvalidValue {
        expected: &'static str,
        got: String,
        file:   &'static str,
        line:   u32,
        column: u32,
    },
    IoError(ioe::IoError),
    ParseBoolError(ParseBoolError),
    ParseIntError(ParseIntError),
    ParseAddressError(String),
    TomlError(TomlError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self { Self::IoError(ioe::IoError::from(err)) }
}

impl From<ioe::IoError> for Error {
    fn from(err: ioe::IoError) -> Self { Self::IoError(err) }
}

impl From<ParseBoolError> for Error {
    fn from(err: ParseBoolError) -> Self { Self::ParseBoolError(err) }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self { Self::ParseIntError(err) }
}

impl From<TomlError> for Error {
    fn from(err: TomlError) -> Self { Self::TomlError(err) }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self { Self::EnvVarError(err) }
}
