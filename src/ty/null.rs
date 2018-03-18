//! Configuration for a `null`-type logger and corresponding factory for an
//! `Async` drain.
//!
//! Corresponds to a logger created with `slog_term::Discard`.
//! This type of logger will not emit any output.

use Error;

use slog::Discard;
use slog_async::{Async, AsyncGuard};


/// Configuration for a logger of type `null`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Config;

impl ::Config for Config {
    fn ty(&self) -> &'static str {
        "null"
    }
}


/// Factory for an `Async` drain of type `null`.
pub struct Factory;

impl ::Factory for Factory {
    type Config = Config;
    type Target = (Async, AsyncGuard);

    fn build(&self, _cfg: &Config) -> Result<Self::Target, Error> {
        Ok(Async::new(Discard).build_with_guard())
    }
}
