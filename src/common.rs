//! Common configuration types.

use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

use slog;


/// The modes in which a log file can be opened.
/// 
/// The default mode is [`Append`](OpenMode::Append).
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpenMode {
    /// Appends to an already existing file or creates a new file if it does not
    /// exist.
    Append,

    /// Truncates an already existing file to 0 length before writing or creates
    /// a new file if it does not exist.
    Truncate,

    /// Creates a new file. No file is allowed to (already) exist at the target
    /// location.
    New,
}

impl Default for OpenMode {
    fn default() -> Self {
        OpenMode::Append
    }
}


/// The output-target for a terminal-based logger.
/// 
/// Defaults to [`Stdout`](TermTarget::Stdout).
/// 
/// See [`Target`](Target) for a target that can represent arbitrary files.
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TermTarget {
    /// The standard output stream.
    Stdout,

    /// The standard error stream.
    Stderr,
}

impl Default for TermTarget {
    fn default() -> Self {
        TermTarget::Stdout
    }
}


/// The output-target for a logger capable of writing to the terminal and files.
/// 
/// Defaults to [`Stdout`](Target::Stdout).
/// 
/// See [`TermTarget`](TermTarget) for a target that can only represent terminal output.
#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    /// The standard output stream.
    Stdout,

    /// The standard error stream.
    Stderr,

    /// A file.
    File {
        /// The path at which the file is located.
        path: PathBuf,

        /// The mode with which the file will be opened.
        mode: OpenMode,
    },
}

impl Default for Target {
    fn default() -> Self {
        Target::Stdout
    }
}

impl Serialize for Target {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        match *self {
            Target::Stdout => serializer.serialize_str("stdout"),
            Target::Stderr => serializer.serialize_str("stderr"),
            Target::File { ref path, ref mode } => {
                let mut state = serializer.serialize_struct("File", 1)?;
                state.serialize_field("path", path)?;
                state.serialize_field("mode", mode)?;
                state.end()
            },
        }
    }
}

impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Path,
            Mode,
            _Ignore,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`path`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(Field::Path),
                            "mode" => Ok(Field::Mode),
                            _ => Ok(Field::_Ignore),
                        }
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }


        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Target;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a version string like \"0.9.8\" or a \
                     detailed dependency like { version = \"0.9.8\" }",
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match s {
                    "stdout" => Ok(Target::Stdout),
                    "stderr" => Ok(Target::Stderr),
                    s => Err(serde::de::Error::unknown_variant(s, &["stdout", "stderr"])),
                }
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut path = None;
                let mut mode = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Path => {
                            if path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path = Some(map.next_value()?);
                        },
                        Field::Mode => {
                            if mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode = Some(map.next_value()?);
                        },
                        _ => {
                            let _ignore: serde::de::IgnoredAny = map.next_value()?;
                        },
                    }
                }

                let path = path.ok_or_else(|| serde::de::Error::missing_field("path"))?;
                let mode = mode.unwrap_or_default();
                Ok(Target::File { path, mode })
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let path: PathBuf = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                let mode: Option<OpenMode> = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                let mode = mode.unwrap_or_default();
                Ok(Target::File { path, mode })
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}


/// Logging level for filtering.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Level {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl Default for Level {
    fn default() -> Self {
        Level::Info
    }
}

impl From<slog::Level> for Level {
    fn from(level: slog::Level) -> Level {
        match level {
            slog::Level::Critical => Level::Critical,
            slog::Level::Error => Level::Error,
            slog::Level::Warning => Level::Warning,
            slog::Level::Info => Level::Info,
            slog::Level::Debug => Level::Debug,
            slog::Level::Trace => Level::Trace,
        }
    }
}

impl<'a> Into<slog::Level> for &'a Level {
    fn into(self) -> slog::Level {
        match *self {
            Level::Critical => slog::Level::Critical,
            Level::Error => slog::Level::Error,
            Level::Warning => slog::Level::Warning,
            Level::Info => slog::Level::Info,
            Level::Debug => slog::Level::Debug,
            Level::Trace => slog::Level::Trace,
        }
    }
}

impl Serialize for Level {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let level: slog::Level = self.into();
        let level = level.as_str().to_lowercase();
        serializer.serialize_str(&level)
    }
}

impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Level;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter
                    .write_str("one of `critical`, `error`, `warning`, `info`, `debug` or `trace`")
            }

            fn visit_str<E>(self, value: &str) -> Result<Level, E>
            where
                E: serde::de::Error,
            {
                use serde::de::Error;

                slog::Level::from_str(value)
                    .map(|level| level.into())
                    .map_err(|_| {
                        Error::unknown_variant(
                            value,
                            &["critical", "error", "warning", "info", "debug", "trace"],
                        )
                    })
            }

            fn visit_u64<E>(self, value: u64) -> Result<Level, E>
            where
                E: serde::de::Error,
            {
                use serde::de::Error;

                slog::Level::from_usize(value as usize)
                    .map(|level| level.into())
                    .ok_or_else(|| {
                        Error::invalid_value(serde::de::Unexpected::Unsigned(value), &self)
                    })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}


/// Timestamp format and timezone.
/// 
/// Defaults to [`Rfc3339Utc`](Timestamp::Rfc3339Utc).
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Timestamp {
    /// UTC time in RFC-3339 format.
    #[serde(rename = "rfc3339-utc")]
    Rfc3339Utc,

    /// Local time in RFC-3339 format.
    #[serde(rename = "rfc3339-local")]
    Rfc3339Local,
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp::Rfc3339Utc
    }
}
