//! Configuration for a `term`-type logger and corresponding factory for an
//! `Async` drain.
//!
//! Corresponds to a logger created with `slog_term::TermDecorator`.

use Error;
pub use common::{Level, OpenMode, TermTarget as Target, Timestamp};

use std;

use slog::{Drain, Never};
use slog_async::{Async, AsyncGuard};
use slog_term::{CompactFormat, Decorator, FullFormat, TermDecorator};

use chrono::{Local, Utc};


#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
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

    /// The color settings.
    #[serde(default)]
    pub color: Color,
}

impl ::Config for Config {
    fn ty(&self) -> &'static str {
        "term"
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


/// The color-settings for the `TermDecorator`.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    /// Automatically enable colors depending on the terminal being used.
    Auto,

    /// Disables colors.
    Disable,

    /// Force-enables colors.
    Force,
}

impl Default for Color {
    fn default() -> Self {
        Color::Auto
    }
}


/// Factory for an `Async` drain of type `term`.
pub struct Factory;

impl ::Factory for Factory {
    type Config = Config;
    type Target = (Async, AsyncGuard);

    fn build(&self, cfg: &Config) -> Result<Self::Target, Error> {
        build(cfg)
    }
}

fn build(cfg: &Config) -> Result<(Async, AsyncGuard), Error> {
    let builder = match cfg.target {
        Target::Stdout => TermDecorator::new().stdout(),
        Target::Stderr => TermDecorator::new().stderr(),
    };

    let builder = match cfg.color {
        Color::Auto => builder,
        Color::Disable => builder.force_plain(),
        Color::Force => builder.force_color(),
    };

    build_1(cfg, builder.build())
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
                .filter_level((&cfg.level).into())
                .fuse();

            build_2(cfg, format)
        },
        Format::Compact => {
            let format = CompactFormat::new(decorator);

            let format = match cfg.timestamp {
                Timestamp::Rfc3339Utc => format.use_custom_timestamp(timestamp_iso8601_utc),
                Timestamp::Rfc3339Local => format.use_custom_timestamp(timestamp_iso8601_local),
            };

            let format = format.build().filter_level((&cfg.level).into()).fuse();

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
