/// A library to process Server.toml files

#[macro_use] mod error;
mod core;

use actix_http::{KeepAlive as ActixKeepAlive, Request, Response};
use actix_service::{IntoServiceFactory, ServiceFactory};
use actix_web::{Error as WebError, HttpServer};
use actix_web::dev::{AppConfig, MessageBody, Service};
use crate::error::{Error, Result};
use crate::core::*;
use serde_derive::Deserialize;
use std::env::{self, VarError};
use std::io::{Read, Write};
use std::fmt::Debug;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
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
    pub client_timeout: Timeout,
    #[serde(rename = "client-shutdown")]
    pub client_shutdown: Timeout,
    #[serde(rename = "shutdown-timeout")]
    pub shutdown_timeout: Timeout,
    pub ssl: Ssl,
}

impl Settings {
    const DEFAULT_TOML_FILE_TEMPLATE: &'static str = r#"
# For more info, see: https://docs.rs/actix-web/3.1.0/actix_web/struct.HttpServer.html.

hosts = [
    ["0.0.0.0", 9000]      # This should work for both development and deployment...
    #                      # ... but other entries are possible, as well.
]
mode = "development"       # Either "development" or "production".
enable-compression = true  # Toggle compression middleware.
enable-log = true          # Toggle logging middleware.

# The number of workers that the server should start.
# By default the number of available logical cpu cores is used.
# Takes a string value: Either "default", or an integer N > 0 e.g. "6".
num-workers = "default"

# The maximum number of pending connections.  This refers to the number of clients
# that can be waiting to be served.  Exceeding this number results in the client
# getting an error when attempting to connect.  It should only affect servers under
# significant load.  Generally set in the 64-2048 range.  The default value is 2048.
# Takes a string value: Either "default", or an integer N > 0 e.g. "6".
backlog = "default"

# Sets the maximum per-worker number of concurrent connections.  All socket listeners
# will stop accepting connections when this limit is reached for each worker.
# By default max connections is set to a 25k.
# Takes a string value: Either "default", or an integer N > 0 e.g. "6".
max-connections = "default"

# Sets the maximum per-worker concurrent connection establish process.  All listeners
# will stop accepting connections when this limit is reached. It can be used to limit
# the global TLS CPU usage.  By default max connections is set to a 256.
# Takes a string value: Either "default", or an integer N > 0 e.g. "6".
max-connection-rate = "default"

# Set server keep-alive setting.  By default keep alive is set to 5 seconds.
# Takes a string value: Either "default", "disabled", "os",
# or a string of the format "N seconds" where N is an integer > 0 e.g. "6 seconds".
keep-alive = "default"

# Set server client timeout in milliseconds for first request.  Defines a timeout
# for reading client request header. If a client does not transmit the entire set of
# headers within this time, the request is terminated with the 408 (Request Time-out)
# error.  To disable timeout, set the value to 0.
# By default client timeout is set to 5000 milliseconds.
# Takes a string value: Either "default", or a string of the format "N milliseconds"
# where N is an integer > 0 e.g. "6 milliseconds".
client-timeout = "default"

# Set server connection shutdown timeout in milliseconds.  Defines a timeout for
# shutdown connection. If a shutdown procedure does not complete within this time,
# the request is dropped.  To disable timeout set value to 0.
# By default client timeout is set to 5000 milliseconds.
# Takes a string value: Either "default", or a string of the format "N milliseconds"
# where N is an integer > 0 e.g. "6 milliseconds".
client-shutdown = "default"

# Timeout for graceful workers shutdown. After receiving a stop signal, workers have
# this much time to finish serving requests. Workers still alive after the timeout
# are force dropped.  By default shutdown timeout sets to 30 seconds.
# Takes a string value: Either "default", or a string of the format "N seconds"
# where N is an integer > 0 e.g. "6 seconds".
shutdown-timeout = "default"

[ssl] # SSL is disabled by default because the certs don't exist
enabled = false
certificate = "path/to/cert/cert.pem"
private-key = "path/to/cert/key.pem"
"#;

    /// Write the TOML config file template to a new file, to be
    /// located at `filepath`.  Return a `Error::FileExists(_)`
    /// error if a file already exists at that location.
    pub fn write_toml_file<P>(filepath: P) -> Result<()>
    where P: AsRef<Path> {
        let filepath = filepath.as_ref();
        let contents = Self::DEFAULT_TOML_FILE_TEMPLATE.trim();
        if filepath.exists() {
            return Err(Error::FileExists(filepath.to_path_buf()));
        }
        let mut file = File::create(filepath)?;
        file.write_all(contents.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    /// Parse an instance of `Self` from a `TOML` file located at `filepath`.
    /// If the `TOML` file doesn't exist, it is generated from a template,
    /// after which the newly generated file is read in and parsed.
    pub fn parse_toml<P>(filepath: P) -> Result<Self>
    where P: AsRef<Path> {
        let filepath = filepath.as_ref();
        if !filepath.exists() { Self::write_toml_file(filepath)?; }
        let mut f = File::open(filepath)?;
        let mut contents = String::with_capacity(f.metadata()?.len() as usize);
        f.read_to_string(&mut contents)?;
        Ok(toml::from_str::<Settings>(&contents)?)
    }

    pub fn override_field<F, V>(field: &mut F, value: V) -> Result<()>
    where F: Parse,
          V: AsRef<str>
    {
        *field = F::parse(value.as_ref())?;
        Ok(())
    }

    pub fn override_field_with_env_var<F, N>(
        field: &mut F,
        var_name: N,
    ) -> Result<()>
    where F: Parse,
          N: AsRef<str> {
        match env::var(var_name.as_ref()) {
            Err(VarError::NotPresent) => Ok((/*NOP*/)),
            Err(var_error) => Err(Error::from(var_error)),
            Ok(value) => Self::override_field(field, value),
        }
    }

}



pub trait ApplySettings {
    #[must_use]
    fn apply_settings(self, settings: &Settings) -> Self;
}

impl<F, I, S, B> ApplySettings for HttpServer<F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S>,
    S: ServiceFactory<Config = AppConfig, Request = Request>,
    S::Error: Into<WebError> + 'static,
    S::InitError: Debug,
    S::Response: Into<Response<B>> + 'static,
    <S::Service as Service>::Future: 'static,
    B: MessageBody + 'static
{
    fn apply_settings(mut self, settings: &Settings) -> Self {
        if settings.ssl.enabled {
            // for Address { host, port } in &settings.hosts {
            //     self = self.bind(format!("{}:{}", host, port))
            //         .unwrap(/*TODO*/);
            // }
            todo!("[ApplySettings] SSL support has not been implemented yet.");
        } else {
            for Address { host, port } in &settings.hosts {
                self = self.bind(format!("{}:{}", host, port))
                    .unwrap(/*TODO*/);
            }
        }
        self = match settings.num_workers {
            NumWorkers::Default   => self,
            NumWorkers::Manual(n) => self.workers(n),
        };
        self = match settings.backlog {
            Backlog::Default   => self,
            Backlog::Manual(n) => self.backlog(n as i32),
        };
        self = match settings.max_connections {
            MaxConnections::Default   => self,
            MaxConnections::Manual(n) => self.max_connections(n),
        };
        self = match settings.max_connection_rate {
            MaxConnectionRate::Default   => self,
            MaxConnectionRate::Manual(n) => self.max_connection_rate(n),
        };
        self = match settings.keep_alive {
            KeepAlive::Default    => self,
            KeepAlive::Disabled   => self.keep_alive(ActixKeepAlive::Disabled),
            KeepAlive::Os         => self.keep_alive(ActixKeepAlive::Os),
            KeepAlive::Seconds(n) => self.keep_alive(n),
        };
        self = match settings.client_timeout {
            Timeout::Default         => self,
            Timeout::Milliseconds(n) => self.client_timeout(n as u64),
            Timeout::Seconds(n)      => self.client_timeout(n as u64 * 1000),
        };
        self = match settings.client_shutdown {
            Timeout::Default         => self,
            Timeout::Milliseconds(n) => self.client_shutdown(n as u64),
            Timeout::Seconds(n)      => self.client_shutdown(n as u64 * 1000),
        };
        self = match settings.shutdown_timeout {
            Timeout::Default         => self,
            Timeout::Milliseconds(_) => self.shutdown_timeout(1),
            Timeout::Seconds(n)      => self.shutdown_timeout(n as u64),
        };
        self
    }
}



#[cfg(test)]
mod tests {
    use actix_web::App;
    use crate::core::Address;
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn apply_settings() -> Result<()> {
        let settings = Settings::parse_toml("Server.toml")?;
        let _ = HttpServer::new(|| { App::new() }).apply_settings(&settings);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn override_field__hosts() -> Result<()> {
        let mut settings = Settings::parse_toml("Server.toml")?;
        assert_eq!(settings.hosts, vec![
            Address { host: "0.0.0.0".into(),   port: 9000 },
        ]);
        Settings::override_field(&mut settings.hosts, r#"[
            ["0.0.0.0", 1234],
        ]"#)?;
        assert_eq!(settings.hosts, vec![
            Address { host: "0.0.0.0".into(),   port: 1234 },
        ]);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn override_field_with_env_var__hosts() -> Result<()> {
        let mut settings = Settings::parse_toml("Server.toml")?;
        assert_eq!(settings.hosts, vec![
            Address { host: "0.0.0.0".into(),   port: 9000 },
        ]);
        std::env::set_var("APPLY_TOML_AND_ENV_VARS__HOSTS", r#"[
            ["0.0.0.0",   1234],
            ["localhost", 2345]
        ]"#);
        Settings::override_field_with_env_var(
            &mut settings.hosts, "APPLY_TOML_AND_ENV_VARS__HOSTS"
        )?;
        assert_eq!(settings.hosts, vec![
            Address { host: "0.0.0.0".into(),   port: 1234 },
            Address { host: "localhost".into(), port: 2345 },
        ]);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn override_field__ssl_enabled() -> Result<()> {
        let mut settings = Settings::parse_toml("Server.toml")?;
        assert!(!settings.ssl.enabled);
        Settings::override_field(&mut settings.ssl.enabled, "true")?;
        assert!(settings.ssl.enabled);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn override_field_with_env_var__ssl_enabled() -> Result<()> {
        let mut settings = Settings::parse_toml("Server.toml")?;
        assert!(!settings.ssl.enabled);
        std::env::set_var("APPLY_TOML_AND_ENV_VARS__SSL_ENABLED", "true");
        Settings::override_field_with_env_var(
            &mut settings.ssl.enabled, "APPLY_TOML_AND_ENV_VARS__SSL_ENABLED"
        )?;
        assert!(settings.ssl.enabled);
        Ok(())
    }

}
