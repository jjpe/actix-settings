use ioe;
use std::env::VarError;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::num::ParseIntError;
use std::str::ParseBoolError;
use toml::de::Error as TomlError;

pub type AtResult<T> = std::result::Result<T, AtError>;

#[derive(Clone, Debug)]
pub enum AtError {
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

macro_rules! InvalidValue {
    (expected: $expected:expr, got: $got:expr,) => {
        crate::AtError::InvalidValue {
            expected: $expected,
            got: $got.to_string(),
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

impl From<IoError> for AtError {
    fn from(err: IoError) -> Self { Self::IoError(ioe::IoError::from(err)) }
}

impl From<ioe::IoError> for AtError {
    fn from(err: ioe::IoError) -> Self { Self::IoError(err) }
}

impl From<ParseBoolError> for AtError {
    fn from(err: ParseBoolError) -> Self { Self::ParseBoolError(err) }
}

impl From<ParseIntError> for AtError {
    fn from(err: ParseIntError) -> Self { Self::ParseIntError(err) }
}

impl From<TomlError> for AtError {
    fn from(err: TomlError) -> Self { Self::TomlError(err) }
}

impl From<VarError> for AtError {
    fn from(err: VarError) -> Self { Self::EnvVarError(err) }
}
