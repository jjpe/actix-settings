/// A library to process Server.toml files

mod error;

use crate::error::ServerResult;
use regex::Regex;
use serde_derive::Deserialize;
use std::io::Read;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct ServerToml {
    pub hosts: Vec<Address>,
    pub mode: Mode,
    #[serde(rename = "enable-compression")]
    pub enable_compression: bool,
    #[serde(rename = "enable-log")]
    pub enable_log: bool,
    #[serde(rename = "num-workers")]
    pub num_workers: NumWorkers,
    pub backlog: Backlog,
    #[serde(rename = "max-connections")]
    pub max_connections: MaxConnections,
    #[serde(rename = "max-connection-rate")]
    pub max_connection_rate: MaxConnectionRate,
    #[serde(rename = "keep-alive")]
    pub keep_alive: KeepAlive,
    #[serde(rename = "client-timeout")]
    pub client_timeout: ClientTimeout,
    #[serde(rename = "client-shutdown")]
    pub client_shutdown: ClientShutdown,
    #[serde(rename = "shutdown-timeout")]
    pub shutdown_timeout: ShutdownTimeout,
    pub ssl: Ssl,
}

impl ServerToml {
    pub fn parse<P: AsRef<Path>>(filename: P) -> ServerResult<Self> {
        const KILOBYTE: usize = 1024;
        const CAPACITY: usize = 10 * KILOBYTE;
        let mut file: File = OpenOptions::new()
            .read(true)
            .write(false)
            .open(filename)?;
        let mut contents = String::with_capacity(CAPACITY);
        file.read_to_string(&mut contents)?;
        Ok(toml::from_str::<ServerToml>(&contents)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Address {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Mode {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "production")]
    Production
}

#[derive(Debug, Clone)]
pub enum NumWorkers {
    Default,
    Manual(usize),
}

impl<'de> serde::Deserialize<'de> for NumWorkers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::de;
        use std::fmt;

        struct NumWorkersVisitor;

        impl<'de> de::Visitor<'de> for NumWorkersVisitor {
            type Value = NumWorkers;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\" or a string containing an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match value {
                    "default" => Ok(NumWorkers::Default),
                    string => match string.parse::<usize>() {
                        Ok(val) => Ok(NumWorkers::Manual(val)),
                        Err(_) => Err(de::Error::invalid_value(
                            de::Unexpected::Str(string),
                            &"a positive integer"
                        ))
                    },
                }
            }
        }

        deserializer.deserialize_string(NumWorkersVisitor)
    }
}


#[derive(Debug, Clone)]
pub enum Backlog {
    Default,
    Manual(usize),
}

impl<'de> serde::Deserialize<'de> for Backlog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::de;
        use std::fmt;

        struct BacklogVisitor;

        impl<'de> de::Visitor<'de> for BacklogVisitor {
            type Value = Backlog;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\" or a string containing an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match value {
                    "default" => Ok(Backlog::Default),
                    string => match string.parse::<usize>() {
                        Ok(val) => Ok(Backlog::Manual(val)),
                        Err(_) => Err(de::Error::invalid_value(
                            de::Unexpected::Str(string),
                            &"an integer > 0"
                        ))
                    },
                }
            }
        }

        deserializer.deserialize_string(BacklogVisitor)
    }
}

#[derive(Debug, Clone)]
pub enum MaxConnections {
    Default,
    Manual(usize),
}

impl<'de> serde::Deserialize<'de> for MaxConnections {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::de;
        use std::fmt;

        struct MaxConnectionsVisitor;

        impl<'de> de::Visitor<'de> for MaxConnectionsVisitor {
            type Value = MaxConnections;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\" or a string containing an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match value {
                    "default" => Ok(MaxConnections::Default),
                    string => match string.parse::<usize>() {
                        Ok(val) => Ok(MaxConnections::Manual(val)),
                        Err(_) => Err(de::Error::invalid_value(
                            de::Unexpected::Str(string),
                            &"an integer > 0"
                        ))
                    },
                }
            }
        }

        deserializer.deserialize_string(MaxConnectionsVisitor)
    }
}

#[derive(Debug, Clone)]
pub enum MaxConnectionRate {
    Default,
    Manual(usize),
}

impl<'de> serde::Deserialize<'de> for MaxConnectionRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::de;
        use std::fmt;

        struct MaxConnectionRateVisitor;

        impl<'de> de::Visitor<'de> for MaxConnectionRateVisitor {
            type Value = MaxConnectionRate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\" or a string containing an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match value {
                    "default" => Ok(MaxConnectionRate::Default),
                    string => match string.parse::<usize>() {
                        Ok(val) => Ok(MaxConnectionRate::Manual(val)),
                        Err(_) => Err(de::Error::invalid_value(
                            de::Unexpected::Str(string),
                            &"an integer > 0"
                        ))
                    },
                }
            }
        }

        deserializer.deserialize_string(MaxConnectionRateVisitor)
    }
}

#[derive(Debug, Clone)]
pub enum KeepAlive {
    Default,
    Disabled,
    Os,
    Seconds(usize),
}

impl<'de> serde::Deserialize<'de> for KeepAlive {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::de;
        use std::fmt;

        struct KeepAliveVisitor;

        impl<'de> de::Visitor<'de> for KeepAliveVisitor {
            type Value = KeepAlive;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\", \"disabled\", \"os\", or a string of the format \"N seconds\" where N is an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
            where E: de::Error {
                lazy_static::lazy_static! {
                    pub static ref FMT: Regex = Regex::new(
                        r"^\d+ seconds$"
                    ).expect("Failed to compile regex: FMT");
                    pub static ref DIGITS: Regex = Regex::new(
                        r"^\d+"
                    ).expect("Failed to compile regex: FMT");
                }
                let invalid_value = |string| Err(de::Error::invalid_value(
                    de::Unexpected::Str(string),
                    &"a string of the format \"N seconds\" where N is an integer > 0"
                ));
                let digits_in = |m: regex::Match| &string[m.start() .. m.end()];
                match string {
                    "default"   => Ok(KeepAlive::Default),
                    "disabled"  => Ok(KeepAlive::Disabled),
                    "OS" | "os" => Ok(KeepAlive::Os),
                    string if !FMT.is_match(&string) => invalid_value(string),
                    string => match DIGITS.find(&string) {
                        None => invalid_value(string),
                        Some(mat) => match digits_in(mat).parse() {
                            Ok(val) => Ok(KeepAlive::Seconds(val)),
                            Err(_) => invalid_value(string),
                        },
                    },
                }
            }
        }

        deserializer.deserialize_string(KeepAliveVisitor)
    }
}


#[derive(Debug, Clone)]
pub enum ClientTimeout {
    Default,
    Milliseconds(usize),
}

impl<'de> serde::Deserialize<'de> for ClientTimeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::de;
        use std::fmt;

        struct ClientTimeoutVisitor;

        impl<'de> de::Visitor<'de> for ClientTimeoutVisitor {
            type Value = ClientTimeout;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\", \"disabled\", \"os\", or a string of the format \"N seconds\" where N is an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
            where E: de::Error {
                lazy_static::lazy_static! {
                    pub static ref FMT: Regex = Regex::new(
                        r"^\d+ milliseconds$"
                    ).expect("Failed to compile regex: FMT");
                    pub static ref DIGITS: Regex = Regex::new(
                        r"^\d+"
                    ).expect("Failed to compile regex: FMT");
                }
                let invalid_value = |string| Err(de::Error::invalid_value(
                    de::Unexpected::Str(string),
                    &"a string of the format \"N seconds\" where N is an integer > 0"
                ));
                let digits_in = |m: regex::Match| &string[m.start() .. m.end()];
                match string {
                    "default"   => Ok(ClientTimeout::Default),
                    string if !FMT.is_match(&string) => invalid_value(string),
                    string => match DIGITS.find(&string) {
                        None => invalid_value(string),
                        Some(mat) => match digits_in(mat).parse() {
                            Ok(val) => Ok(ClientTimeout::Milliseconds(val)),
                            Err(_) => invalid_value(string),
                        },
                    },
                }
            }
        }

        deserializer.deserialize_string(ClientTimeoutVisitor)
    }
}


#[derive(Debug, Clone)]
pub enum ClientShutdown {
    Default,
    Milliseconds(usize),
}

impl<'de> serde::Deserialize<'de> for ClientShutdown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::de;
        use std::fmt;

        struct ClientShutdownVisitor;

        impl<'de> de::Visitor<'de> for ClientShutdownVisitor {
            type Value = ClientShutdown;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\", \"disabled\", \"os\", or a string of the format \"N seconds\" where N is an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
            where E: de::Error {
                lazy_static::lazy_static! {
                    pub static ref FMT: Regex = Regex::new(
                        r"^\d+ milliseconds$"
                    ).expect("Failed to compile regex: FMT");
                    pub static ref DIGITS: Regex = Regex::new(
                        r"^\d+"
                    ).expect("Failed to compile regex: FMT");
                }
                let invalid_value = |string| Err(de::Error::invalid_value(
                    de::Unexpected::Str(string),
                    &"a string of the format \"N seconds\" where N is an integer > 0"
                ));
                let digits_in = |m: regex::Match| &string[m.start() .. m.end()];
                match string {
                    "default"   => Ok(ClientShutdown::Default),
                    string if !FMT.is_match(&string) => invalid_value(string),
                    string => match DIGITS.find(&string) {
                        None => invalid_value(string),
                        Some(mat) => match digits_in(mat).parse() {
                            Ok(val) => Ok(ClientShutdown::Milliseconds(val)),
                            Err(_) => invalid_value(string),
                        },
                    },
                }
            }
        }

        deserializer.deserialize_string(ClientShutdownVisitor)
    }
}

#[derive(Debug, Clone)]
pub enum ShutdownTimeout {
    Default,
    Seconds(usize),
}

impl<'de> serde::Deserialize<'de> for ShutdownTimeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::de;
        use std::fmt;

        struct ShutdownTimeoutVisitor;

        impl<'de> de::Visitor<'de> for ShutdownTimeoutVisitor {
            type Value = ShutdownTimeout;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\", \"disabled\", \"os\", or a string of the format \"N seconds\" where N is an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
            where E: de::Error {
                lazy_static::lazy_static! {
                    pub static ref FMT: Regex = Regex::new(
                        r"^\d+ seconds$"
                    ).expect("Failed to compile regex: FMT");
                    pub static ref DIGITS: Regex = Regex::new(
                        r"^\d+"
                    ).expect("Failed to compile regex: FMT");
                }
                let invalid_value = |string| Err(de::Error::invalid_value(
                    de::Unexpected::Str(string),
                    &"a string of the format \"N seconds\" where N is an integer > 0"
                ));
                let digits_in = |m: regex::Match| &string[m.start() .. m.end()];
                match string {
                    "default"   => Ok(ShutdownTimeout::Default),
                    string if !FMT.is_match(&string) => invalid_value(string),
                    string => match DIGITS.find(&string) {
                        None => invalid_value(string),
                        Some(mat) => match digits_in(mat).parse() {
                            Ok(val) => Ok(ShutdownTimeout::Seconds(val)),
                            Err(_) => invalid_value(string),
                        },
                    },
                }
            }
        }

        deserializer.deserialize_string(ShutdownTimeoutVisitor)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ssl {
    pub enabled: bool,
    pub certificate: PathBuf,
    #[serde(rename = "private-key")]
    pub private_key: PathBuf,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() -> ServerResult<()> {
        let toml = ServerToml::parse("Server.toml.template")?;

        println!("{:#?}", toml);

        // Ok(())
        todo!()
    }
}
