# actix-toml

A Rust crate that allows for configuring `actix-web`'s (HttpServer)[https://docs.rs/actix-web/3.1.0/actix_web/struct.HttpServer.html] instance through a `TOML` file.

## Usage

Add this to your `Cargo.toml`:

``` toml
[dependencies]
actix-toml = "0.2.0"
actix-web  = "3.1.0" # Should already be present
env_logger = "0.8.1"
```

### Basic usage

Import these items into your crate:

``` rust
use actix_toml::{ApplySettings, AtResult, Settings};
```

They can be used like this:
``` rust
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Arc;
    use actix_web::http::ContentEncoding;

    let mut settings = Settings::parse_toml("Server.toml")
        .expect("Failed to parse `Settings` from Server.toml");
    // If the environment variable `$APPLICATION__HOSTS` is set,
    // have its value override the `settings.hosts` setting:
    Settings::override_field_with_env_var(
        &mut settings.hosts,
        "APPLICATION__HOSTS"
    )?;
    init_logger(&settings);
    let settings = Arc::new(settings); // Leverage `Arc` to not waste memory
    let settings2 = Arc::clone(&settings);
    HttpServer::new(move || {
        App::new()
            // Include this `.wrap()` call for compression settings to take effect:
            .wrap(Compress::new(if settings2.enable_compression {
                ContentEncoding::Deflate
            } else {
                ContentEncoding::Identity
            }))
            .wrap(Logger::default())
            .data(settings2.clone()) // <- Make `Settings` available to handlers
        // Define routes:
            .service(index) // <- Add routes here as normal
    })
        .apply_settings(&settings) // <- apply the `Settings` to actix's `HttpServer`
        .run()
        .await
}

/// Initialize the logging infrastructure
fn init_logger(settings: &Settings) {
    if !settings.enable_log { return }
    std::env::set_var("RUST_LOG", match settings.mode {
        Mode::Development => "actix_web=debug",
        Mode::Production  => "actix_web=info",
    });
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

```


### Custom Settings

There is a way to extend the available settings.  This can be used to combine
the settings provided by `actix-web` and those provided by application server
built using `actix`.

Have a look at the `override_extended_field_with_custom_type` test
in `src/lib.rs` to see how.


## Special Thanks

This crate was made possible by support from Accept B.V.
