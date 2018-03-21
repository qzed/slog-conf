//! Configuration for a `json`-type logger and corresponding factory for an
//! `Async` drain.
//!
//! Corresponds to a logger created with `slog_json::Json`.

use Error;
pub use common::{Level, OpenMode, Target, Timestamp};
use common::OptionalTag;

use std;

use slog::{self, Drain, FnValue, PushFnValue, PushFnValueSerializer, Record};
use slog_async::{Async, AsyncGuard};
use slog_json::{Json, JsonBuilder};

use chrono::{Local, Utc};


/// Configuration for a logger of type `json`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Config {
    /// The target to which the logger should write.
    #[serde(default)]
    pub target: Target,

    /// The format in which a record should be displayed.
    #[serde(default)]
    pub format: Format,

    /// The minimal logging level the logger should output.
    #[serde(default)]
    pub level: Level,

    /// The timestamp format.
    #[serde(default)]
    pub timestamp: Timestamp,

    /// If set to `true`, start each entry on a new line.
    #[serde(default = "default::newlines")]
    pub newlines: bool,

    /// If set to `true`, emit pretty-formatted json.
    #[serde(default = "default::pretty")]
    pub pretty: bool,
}

impl ::Config for Config {
    fn ty(&self) -> &'static str {
        "json"
    }
}


/// The json-format in which a record should be displayed.
///
/// This controls which key-value pairs are being emitted.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    /// Basic key-value pairs.
    ///
    /// Provides the log-level (`level`), the timestamp (`ts`), and the message
    /// (`msg`).
    Basic,

    /// Basic key-value pairs with tag.
    ///
    /// Provides the log-level (`level`), the timestamp (`ts`), the message
    /// (`msg`), and an optional tag (`tag`).
    Tagged,

    /// A basic winston-style format.
    ///
    /// Provides the log-level (`level`), the timestamp (`timestamp`), the
    /// message (`message`), and an optional tag (`label`).
    Winston,

    /// Bunyan-style format.
    ///
    /// Contains all required bunyan core fields. Have a look at the [bunyan
    /// documentation](https://github.com/trentm/node-bunyan#core-fields) for
    /// details.
    #[cfg(feature = "json-bunyan")]
    Bunyan,
}

impl Default for Format {
    fn default() -> Self {
        Format::Basic
    }
}


/// Factory for an `Async` drain of type `json`.
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
        Target::Stdout => build_1(cfg, Json::new(std::io::stdout())),
        Target::Stderr => build_1(cfg, Json::new(std::io::stderr())),
        Target::File { ref path, mode } => {
            let mut opt = std::fs::OpenOptions::new();

            match mode {
                OpenMode::Append => opt.create(true).write(true).append(true),
                OpenMode::Truncate => opt.create(true).write(true).truncate(true),
                OpenMode::New => opt.create_new(true).write(true),
            };

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            build_1(cfg, Json::new(opt.open(path)?))
        },
    }
}

fn build_1<W>(cfg: &Config, builder: JsonBuilder<W>) -> Result<(Async, AsyncGuard), Error>
where
    W: std::io::Write + Send + 'static,
{
    let builder = match cfg.format {
        Format::Basic => match cfg.timestamp {
            Timestamp::Rfc3339Utc => builder.add_key_value(o!(
                "msg" => PushFnValue(|r, s| s.emit(r.msg())),
                "level" => FnValue(|r| r.level().as_short_str()),
                "ts" => PushFnValue(timestamp_iso8601_utc),
            )),
            Timestamp::Rfc3339Local => builder.add_key_value(o!(
                "msg" => PushFnValue(|r, s| s.emit(r.msg())),
                "level" => FnValue(|r| r.level().as_short_str()),
                "ts" => PushFnValue(timestamp_iso8601_loc),
            )),
        },
        Format::Tagged => match cfg.timestamp {
            Timestamp::Rfc3339Utc => builder.add_key_value(o!(
                "msg" => PushFnValue(|r, s| s.emit(r.msg())),
                "tag" => OptionalTag,
                "level" => FnValue(|r| r.level().as_short_str()),
                "ts" => PushFnValue(timestamp_iso8601_utc),
            )),
            Timestamp::Rfc3339Local => builder.add_key_value(o!(
                "msg" => PushFnValue(|r, s| s.emit(r.msg())),
                "tag" => OptionalTag,
                "level" => FnValue(|r| r.level().as_short_str()),
                "ts" => PushFnValue(timestamp_iso8601_loc),
            )),
        },
        Format::Winston => match cfg.timestamp {
            Timestamp::Rfc3339Utc => builder.add_key_value(o!(
                "message" => PushFnValue(|r, s| s.emit(r.msg())),
                "label" => OptionalTag,
                "level" => FnValue(|r| r.level().as_short_str()),
                "timestamp" => PushFnValue(timestamp_iso8601_utc),
            )),
            Timestamp::Rfc3339Local => builder.add_key_value(o!(
                "message" => PushFnValue(|r, s| s.emit(r.msg())),
                "label" => OptionalTag,
                "level" => FnValue(|r| r.level().as_short_str()),
                "timestamp" => PushFnValue(timestamp_iso8601_loc),
            )),
        },
        #[cfg(feature = "json-bunyan")]
        Format::Bunyan => match cfg.timestamp {
            Timestamp::Rfc3339Utc => builder.add_key_value(o!(
                "msg" => PushFnValue(|r, s| s.emit(r.msg())),
                "level" => FnValue(|r| bunyan::level(r.level())),
                "pid" => bunyan::pid(),
                "name" => bunyan::name(),
                "hostname" => bunyan::hostname(),
                "time" => PushFnValue(timestamp_iso8601_utc),
                "v" => 0u8,
            )),
            Timestamp::Rfc3339Local => builder.add_key_value(o!(
                "msg" => PushFnValue(|r, s| s.emit(r.msg())),
                "level" => FnValue(|r| bunyan::level(r.level())),
                "pid" => bunyan::pid(),
                "name" => bunyan::name(),
                "hostname" => bunyan::hostname(),
                "time" => PushFnValue(timestamp_iso8601_loc),
                "v" => 0u8,
            )),
        },
    };

    let drain = builder
        .set_newlines(cfg.newlines)
        .set_pretty(cfg.pretty)
        .build();

    build_2(cfg, drain)
}

fn build_2<W>(cfg: &Config, drain: Json<W>) -> Result<(Async, AsyncGuard), Error>
where
    W: std::io::Write + Send + 'static,
{
    let drain = drain.filter_level(cfg.level.into());
    Ok(Async::new(drain.fuse()).build_with_guard())
}


fn timestamp_iso8601_utc<'c, 'd>(_: &'c Record<'d>, s: PushFnValueSerializer<'c>) -> slog::Result {
    s.emit(Utc::now().to_rfc3339())
}

fn timestamp_iso8601_loc<'c, 'd>(_: &'c Record<'d>, s: PushFnValueSerializer<'c>) -> slog::Result {
    s.emit(Local::now().to_rfc3339())
}


mod default {
    pub fn newlines() -> bool { true }
    pub fn pretty() -> bool { false }
}

mod bunyan {
    use std;
    use slog::Level;

    pub fn level(level: Level) -> u8 {
        match level {
            Level::Critical => 60,
            Level::Error => 50,
            Level::Warning => 40,
            Level::Info => 30,
            Level::Debug => 20,
            Level::Trace => 10,
        }
    }

    pub fn name() -> String {
        let path = std::env::current_exe().ok();
        let name = path.as_ref().and_then(|path| path.file_name());

        name.and_then(|name| name.to_str())
            .map(|name| name.to_owned())
            .unwrap_or_else(|| "<unknown>".into())
    }

    pub fn hostname() -> String {
        #[cfg(unix)]
        use libc::gethostname;

        #[cfg(windows)]
        use winapi::um::winsock2::gethostname;

        const MAXLEN: usize = 256;

        let mut buf = [0 as std::os::raw::c_char; MAXLEN];
        let err = unsafe { gethostname(buf.as_mut_ptr() as *mut _, MAXLEN as _) };

        let name = if err == 0 {
            unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()).to_str().ok() }
        } else {
            None
        };

        name.unwrap_or("<unknown>").to_owned()
    }

    pub fn pid() -> u64 {
        #[cfg(unix)]
        use libc::getpid;

        #[cfg(windows)]
        use winapi::um::processthreadsapi::GetCurrentProcessId as getpid;

        unsafe { getpid() as u64 }
    }
}
