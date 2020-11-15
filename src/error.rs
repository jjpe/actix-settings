use ioe;
use std::env::VarError;
use std::io::{self, Error as IoError};
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


impl From<AtError> for IoError {
    fn from(err: AtError) -> Self {
        match err {
            AtError::EnvVarError(var_error) => {
                let msg = format!("Env var error: {}", var_error);
                IoError::new(io::ErrorKind::InvalidInput, msg)
            },
            AtError::FileExists(path_buf) => {
                let msg = format!("File exists: {}", path_buf.display());
                IoError::new(io::ErrorKind::AlreadyExists, msg)
            },
            AtError::InvalidValue { expected, ref got, file, line, column } => {
                let msg = format!("Expected {}, got {}  (@ {}:{}:{})",
                                  expected, got, file, line, column);
                IoError::new(io::ErrorKind::InvalidInput, msg)
            },
            AtError::IoError(io_error) => io_error.into(),
            AtError::ParseBoolError(parse_bool_error) => {
                let msg = format!("Failed to parse boolean: {}", parse_bool_error);
                IoError::new(io::ErrorKind::InvalidInput, msg)
            },
            AtError::ParseIntError(parse_int_error) => {
                let msg = format!("Failed to parse integer: {}", parse_int_error);
                IoError::new(io::ErrorKind::InvalidInput, msg)
            },
            AtError::ParseAddressError(string) => {
                let msg = format!("Failed to parse address: {}", string);
                IoError::new(io::ErrorKind::InvalidInput, msg)
            },
            AtError::TomlError(toml_error) => {
                let msg = format!("TOML error: {}", toml_error);
                IoError::new(io::ErrorKind::InvalidInput, msg)
            },
        }
    }
}
