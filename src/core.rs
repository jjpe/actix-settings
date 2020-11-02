/// Core types and impls

use crate::error::Error;
use regex::Regex;
use serde::de;
use serde_derive::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

pub trait Parse: Sized {
    type Error;

    fn parse(string: &str) -> Result<Self, Error>;
}

impl Parse for bool {
    type Error = Error;

    fn parse(string: &str) -> Result<Self, Error> {
        Self::from_str(string).map_err(Error::from)
    }
}

macro_rules! impl_parse_for_int_type {
    ($($int_type:ty),+ $(,)?) => {
        $(
            impl Parse for $int_type {
                type Error = Error;
                fn parse(string: &str) -> Result<Self, Error> {
                    Self::from_str(string).map_err(Error::from)
                }
            }
        )+
    }
}
impl_parse_for_int_type![i8, i16, i32, i64, i128,    u8, u16, u32, u64, u128];

impl Parse for PathBuf {
    type Error = Error;

    fn parse(string: &str) -> Result<Self, Error> {
        Ok(PathBuf::from(string))
    }
}



#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Address {
    pub host: String,
    pub port: u16,
}

lazy_static::lazy_static! {
    pub static ref ADDR_REGEX: Regex = Regex::new(r#"(?x)
        \[           # opening square bracket
        (\s)*            # optional whitespace
            "(?P<host>[^"]+)"   # host name  (string)
            ,                   # separating comma
            (\s)*               # optional whitespace
            (?P<port>\d+)       # port number  (integer)
        (\s)*            # optional whitespace
        \]           # closing square bracket
    "#).expect("Failed to compile regex: ADDR_REGEX");
    pub static ref ADDR_LIST_REGEX: Regex = Regex::new(r#"(?x)
        \[       # opening square bracket (list)
        (\s)*        # optional whitespace
            (?P<elements>(
                \[".*", (\s)* \d+\] # element
                (,)?                # element separator
                (\s)*               # optional whitespace
            )*)
        (\s)*        # optional whitespace
        \]       # closing square bracket (list)
    "#).expect("Failed to compile regex: ADDRS_REGEX");
}

impl Parse for Address {
    type Error = Error;

    fn parse(string: &str) -> Result<Self, Error> {
        let mut items = string.trim()
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(",");
        let parse_error = || Error::ParseAddressError(string.to_string());
        if !ADDR_REGEX.is_match(string) { return Err(parse_error()); }
        Ok(Self {
            host: items.next().ok_or(parse_error())?.trim().to_string(),
            port: items.next().ok_or(parse_error())?.trim().parse()?
        })
    }
}

impl Parse for Vec<Address> {
    type Error = Error;

    fn parse(string: &str) -> Result<Self, Error> {
        let parse_error = || Error::ParseAddressError(string.to_string());
        if !ADDR_LIST_REGEX.is_match(string) { return Err(parse_error()); }
        let mut addrs = vec![];
        for list_caps in ADDR_LIST_REGEX.captures_iter(string) {
            let elements = &list_caps["elements"].trim();
            for elt_caps in ADDR_REGEX.captures_iter(elements) {
                addrs.push(Address {
                    host: elt_caps["host"].to_string(),
                    port: elt_caps["port"].parse()?
                });
            }
        }
        Ok(addrs)
    }
}


#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum Mode {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "production")]
    Production
}

impl Parse for Mode {
    type Error = Error;

    fn parse(string: &str) -> std::result::Result<Self, Self::Error> {
        match string {
            "development" => Ok(Self::Development),
            "production"  => Ok(Self::Production),
            _ => Err(InvalidValue! {
                expected: "\"development\" | \"production\".",
                got: string,
            })
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumWorkers {
    Default,
    Manual(usize),
}

impl Parse for NumWorkers {
    type Error = Error;

    fn parse(string: &str) -> std::result::Result<Self, Self::Error> {
        match string {
            "default" => Ok(NumWorkers::Default),
            string => match string.parse::<usize>() {
                Ok(val) => Ok(NumWorkers::Manual(val)),
                Err(_) => Err(InvalidValue! {
                    expected: "a positive integer",
                    got: string,
                }),
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for NumWorkers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct NumWorkersVisitor;

        impl<'de> de::Visitor<'de> for NumWorkersVisitor {
            type Value = NumWorkers;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\" or a string containing an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match NumWorkers::parse(value) {
                    Ok(num_workers) => Ok(num_workers),
                    Err(Error::InvalidValue { expected, got, .. }) =>
                        Err(de::Error::invalid_value(
                            de::Unexpected::Str(&got),
                            &expected
                        )),
                    Err(_) => unreachable!(),
                }
            }
        }

        deserializer.deserialize_string(NumWorkersVisitor)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Backlog {
    Default,
    Manual(usize),
}

impl Parse for Backlog {
    type Error = Error;

    fn parse(string: &str) -> std::result::Result<Self, Self::Error> {
        match string {
            "default" => Ok(Backlog::Default),
            string => match string.parse::<usize>() {
                Ok(val) => Ok(Backlog::Manual(val)),
                Err(_) => Err(InvalidValue! {
                    expected: "an integer > 0",
                    got: string,
                }),
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for Backlog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct BacklogVisitor;

        impl<'de> de::Visitor<'de> for BacklogVisitor {
            type Value = Backlog;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\" or a string containing an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match Backlog::parse(value) {
                    Ok(backlog) => Ok(backlog),
                    Err(Error::InvalidValue { expected, got, .. }) =>
                        Err(de::Error::invalid_value(
                            de::Unexpected::Str(&got),
                            &expected
                        )),
                    Err(_) => unreachable!(),
                }
            }
        }

        deserializer.deserialize_string(BacklogVisitor)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaxConnections {
    Default,
    Manual(usize),
}

impl Parse for MaxConnections {
    type Error = Error;

    fn parse(string: &str) -> std::result::Result<Self, Self::Error> {
        match string {
            "default" => Ok(MaxConnections::Default),
            string => match string.parse::<usize>() {
                Ok(val) => Ok(MaxConnections::Manual(val)),
                Err(_) => Err(InvalidValue! {
                    expected: "an integer > 0",
                    got: string,
                }),
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for MaxConnections {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MaxConnectionsVisitor;

        impl<'de> de::Visitor<'de> for MaxConnectionsVisitor {
            type Value = MaxConnections;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\" or a string containing an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match MaxConnections::parse(value) {
                    Ok(max_connections) => Ok(max_connections),
                    Err(Error::InvalidValue { expected, got, .. }) =>
                        Err(de::Error::invalid_value(
                            de::Unexpected::Str(&got),
                            &expected
                        )),
                    Err(_) => unreachable!(),
                }
            }
        }

        deserializer.deserialize_string(MaxConnectionsVisitor)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaxConnectionRate {
    Default,
    Manual(usize),
}

impl Parse for MaxConnectionRate {
    type Error = Error;

    fn parse(string: &str) -> std::result::Result<Self, Self::Error> {
        match string {
            "default" => Ok(MaxConnectionRate::Default),
            string => match string.parse::<usize>() {
                Ok(val) => Ok(MaxConnectionRate::Manual(val)),
                Err(_) => Err(InvalidValue! {
                    expected: "an integer > 0",
                    got: string,
                }),
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for MaxConnectionRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MaxConnectionRateVisitor;

        impl<'de> de::Visitor<'de> for MaxConnectionRateVisitor {
            type Value = MaxConnectionRate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\" or a string containing an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match MaxConnectionRate::parse(value) {
                    Ok(max_connection_rate) => Ok(max_connection_rate),
                    Err(Error::InvalidValue { expected, got, .. }) =>
                        Err(de::Error::invalid_value(
                            de::Unexpected::Str(&got),
                            &expected
                        )),
                    Err(_) => unreachable!(),
                }
            }
        }

        deserializer.deserialize_string(MaxConnectionRateVisitor)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeepAlive {
    Default,
    Disabled,
    Os,
    Seconds(usize),
}

impl Parse for KeepAlive {
    type Error = Error;

    fn parse(string: &str) -> std::result::Result<Self, Self::Error> {
        lazy_static::lazy_static! {
            pub static ref FMT: Regex = Regex::new(r"^\d+ seconds$")
                .expect("Failed to compile regex: FMT");
            pub static ref DIGITS: Regex = Regex::new(r"^\d+")
                .expect("Failed to compile regex: FMT");
        }
        macro_rules! invalid_value {
            ($got:expr) => {
                Err(InvalidValue! {
                    expected: "a string of the format \"N seconds\" where N is an integer > 0",
                    got: $got,
                })
            }
        }
        let digits_in = |m: regex::Match| &string[m.start() .. m.end()];
        match string {
            "default"   => Ok(KeepAlive::Default),
            "disabled"  => Ok(KeepAlive::Disabled),
            "OS" | "os" => Ok(KeepAlive::Os),
            string if !FMT.is_match(&string) => invalid_value!(string),
            string => match DIGITS.find(&string) {
                None => invalid_value!(string),
                Some(mat) => match digits_in(mat).parse() {
                    Ok(val) => Ok(KeepAlive::Seconds(val)),
                    Err(_) => invalid_value!(string),
                },
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for KeepAlive {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct KeepAliveVisitor;

        impl<'de> de::Visitor<'de> for KeepAliveVisitor {
            type Value = KeepAlive;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\", \"disabled\", \"os\", or a string of the format \"N seconds\" where N is an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match KeepAlive::parse(value) {
                    Ok(keep_alive) => Ok(keep_alive),
                    Err(Error::InvalidValue { expected, got, .. }) =>
                        Err(de::Error::invalid_value(
                            de::Unexpected::Str(&got),
                            &expected
                        )),
                    Err(_) => unreachable!(),
                }
            }
        }

        deserializer.deserialize_string(KeepAliveVisitor)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timeout {
    Default,
    Milliseconds(usize),
    Seconds(usize),
}

impl Parse for Timeout {
    type Error = Error;

    fn parse(string: &str) -> std::result::Result<Self, Self::Error> {
        lazy_static::lazy_static! {
            pub static ref FMT: Regex = Regex::new(r"^\d+ (milliseconds|seconds)$")
                .expect("Failed to compile regex: FMT");
            pub static ref DIGITS: Regex = Regex::new(r"^\d+")
                .expect("Failed to compile regex: DIGITS");
            pub static ref UNIT: Regex = Regex::new(r"(milliseconds|seconds)$")
                .expect("Failed to compile regex: UNIT");
        }
        macro_rules! invalid_value {
            ($got:expr) => {
                Err(InvalidValue! {
                    expected: "a string of the format \"N seconds\" or \"N milliseconds\" where N is an integer > 0",
                    got: $got,
                })
            }
        }
        match string {
            "default"   => Ok(Timeout::Default),
            string if !FMT.is_match(&string) => invalid_value!(string),
            string => match (DIGITS.find(&string), UNIT.find(&string)) {
                (None, _) => invalid_value!(string),
                (_, None) => invalid_value!(string),
                (Some(dmatch), Some(umatch)) => {
                    let digits = &string[dmatch.start() .. dmatch.end()];
                    let   unit = &string[umatch.start() .. umatch.end()];
                    match (digits.parse(), unit) {
                        (Ok(v), "milliseconds") => Ok(Timeout::Milliseconds(v)),
                        (Ok(v),      "seconds") => Ok(Timeout::Seconds(v)),
                        _ => invalid_value!(string),
                    }
                }
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for Timeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct TimeoutVisitor;

        impl<'de> de::Visitor<'de> for TimeoutVisitor {
            type Value = Timeout;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let msg = "Either \"default\", \"disabled\", \"os\", or a string of the format \"N seconds\" where N is an integer > 0";
                formatter.write_str(msg)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: de::Error {
                match Timeout::parse(value) {
                    Ok(num_workers) => Ok(num_workers),
                    Err(Error::InvalidValue { expected, got, .. }) =>
                        Err(de::Error::invalid_value(
                            de::Unexpected::Str(&got),
                            &expected
                        )),
                    Err(_) => unreachable!(),
                }
            }
        }

        deserializer.deserialize_string(TimeoutVisitor)
    }
}


#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Ssl {
    pub enabled: bool,
    pub certificate: PathBuf,
    #[serde(rename = "private-key")]
    pub private_key: PathBuf,
}
