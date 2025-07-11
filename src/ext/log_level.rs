//! Conversions for [`LogLevel`].

use crate::xterm::LogLevel;

use log::Level;
use std::convert::TryFrom;

impl From<Level> for LogLevel {
    fn from(level: Level) -> LogLevel {
        match level {
            Level::Trace | Level::Debug => LogLevel::Debug,
            Level::Info => LogLevel::Info,
            Level::Warn => LogLevel::Warn,
            Level::Error => LogLevel::Error,
        }
    }
}

impl From<Option<Level>> for LogLevel {
    fn from(level: Option<Level>) -> LogLevel {
        level.map_or(LogLevel::Off, Into::into)
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
/// Type indicating that a [`LogLevel`] to [`Level`] conversion failed because
/// the [`LogLevel`] was [`Off`](LogLevel::Off).
pub struct LogLevelIsOff;

impl TryFrom<LogLevel> for Level {
    type Error = LogLevelIsOff;

    #[allow(clippy::match_wildcard_for_single_variants)]
    fn try_from(level: LogLevel) -> Result<Level, LogLevelIsOff> {
        match level {
            LogLevel::Debug => Ok(Level::Debug),
            LogLevel::Info => Ok(Level::Info),
            LogLevel::Warn => Ok(Level::Warn),
            LogLevel::Error => Ok(Level::Error),
            LogLevel::Off => Err(LogLevelIsOff),
            _ => unreachable!(),
        }
    }
}
