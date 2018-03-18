//! Configuration for a `plain`-type logger and corresponding factory for an
//! `Async` drain.
//!
//! Corresponds to a logger created with `slog_term::PlainDecorator`.

use Error;
pub use common::{Level, OpenMode, Target, Timestamp};

use std;

use slog::{Drain, Never};
use slog_async::{Async, AsyncGuard};
use slog_term::{CompactFormat, Decorator, FullFormat, PlainDecorator};

use chrono::{Local, Utc};


/// Configuration for a logger of type `plain`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Config {
    /// The target to which the logger should write.
    #[serde(default)]
    pub target: Target,

    /// The format in which the logger should display its information.
    #[serde(default)]
    pub format: Format,

    /// The minimal logging level the logger should output.
    #[serde(default)]
    pub level: Level,

    /// The timestamp format.
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl ::Config for Config {
    fn ty(&self) -> &'static str {
        "plain"
    }
}


/// The format in which the logger should display its information.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    /// Display all information in every written line. Corresponds to
    /// `slog_term::FullFormat`.
    Full,

    /// Display information in a compact, non-repetitive format. Corresponds to
    /// `slog_term::CompactFormat`.
    Compact,
}

impl Default for Format {
    fn default() -> Self {
        Format::Full
    }
}


/// Factory for an `Async` drain of type `plain`.
pub struct Factory;

impl ::Factory for Factory {
    type Config = Config;
    type Target = (Async, AsyncGuard);

    fn build(&self, cfg: &Config) -> Result<Self::Target, Error> {
        build(cfg)
    }
}

fn build(cfg: &Config) -> Result<(Async, AsyncGuard), Error> {
    match cfg.target {
        Target::Stdout => build_1(cfg, PlainDecorator::new(std::io::stdout())),
        Target::Stderr => build_1(cfg, PlainDecorator::new(std::io::stderr())),
        Target::File { ref path, mode } => {
            let mut opt = std::fs::OpenOptions::new();

            match mode {
                OpenMode::Append => opt.create(true).write(true).append(true),
                OpenMode::Truncate => opt.create(true).write(true).truncate(true),
                OpenMode::New => opt.create_new(true).write(true),
            };

            build_1(cfg, PlainDecorator::new(opt.open(path)?))
        },
    }
}

fn build_1<D>(cfg: &Config, decorator: D) -> Result<(Async, AsyncGuard), Error>
where
    D: Decorator + Send + 'static,
{
    match cfg.format {
        Format::Full => {
            let format = FullFormat::new(decorator);

            let format = match cfg.timestamp {
                Timestamp::Rfc3339Utc => format.use_custom_timestamp(timestamp_iso8601_utc),
                Timestamp::Rfc3339Local => format.use_custom_timestamp(timestamp_iso8601_local),
            };

            let format = format
                .use_original_order()
                .build()
                .filter_level(cfg.level.into())
                .fuse();

            build_2(cfg, format)
        },
        Format::Compact => {
            let format = CompactFormat::new(decorator);

            let format = match cfg.timestamp {
                Timestamp::Rfc3339Utc => format.use_custom_timestamp(timestamp_iso8601_utc),
                Timestamp::Rfc3339Local => format.use_custom_timestamp(timestamp_iso8601_local),
            };

            let format = format.build().filter_level(cfg.level.into()).fuse();

            build_2(cfg, format)
        },
    }
}

fn build_2<D>(_cfg: &Config, drain: D) -> Result<(Async, AsyncGuard), Error>
where
    D: Drain<Err = Never, Ok = ()> + Send + 'static,
{
    Ok(Async::new(drain).build_with_guard())
}

fn timestamp_iso8601_utc(w: &mut std::io::Write) -> std::io::Result<()> {
    write!(w, "{}", Utc::now().to_rfc3339())
}

fn timestamp_iso8601_local(w: &mut std::io::Write) -> std::io::Result<()> {
    write!(w, "{}", Local::now().to_rfc3339())
}
