//! Logger capability abstraction
//!
//! Provides a minimal logging capability trait to prepare for effect isolation.
//! This keeps the core pure while allowing injection of structured logging at
//! boundaries. Initial implementation offers a `NoOpLogger` (default) and a
//! simple `StdoutLogger` for immediate use. Future work can extend this with
//! structured/leveled logging without changing call sites.
//!
//! Design goals:
//! - Zero-cost when using `NoOpLogger` (calls optimized away in release)
//! - Simple trait object safe interface
//! - Explicit levels; minimal set to start (Info, Warn, Error, Debug, Trace)
//! - Avoid pulling external deps until justified

/// Log level enumeration (expandable if needed)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Logger capability trait. Implementors should be lightweight & thread-safe.
pub trait Logger: Send + Sync {
    fn log(&self, level: LogLevel, message: &str);

    fn trace(&self, message: &str) { self.log(LogLevel::Trace, message); }
    fn debug(&self, message: &str) { self.log(LogLevel::Debug, message); }
    fn info(&self, message: &str) { self.log(LogLevel::Info, message); }
    fn warn(&self, message: &str) { self.log(LogLevel::Warn, message); }
    fn error(&self, message: &str) { self.log(LogLevel::Error, message); }
}

/// No-op logger (default for pure contexts)
#[derive(Debug, Default, Clone, Copy)]
pub struct NoOpLogger;

impl Logger for NoOpLogger {
    #[inline]
    fn log(&self, _level: LogLevel, _message: &str) {
        // Intentionally no-op
    }
}

/// Simple stdout logger (minimal, unstructured). For early integration only.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{level:?}] {message}");
    }
}

/// Filtering logger that only emits messages at or above a minimum level
#[derive(Debug, Clone, Copy)]
pub struct FilteringLogger {
    pub min: LogLevel,
}

impl FilteringLogger {
    #[must_use]
    pub const fn new(min: LogLevel) -> Self { Self { min } }
    #[inline]
    const fn enabled(&self, level: LogLevel) -> bool { (level as u8) >= (self.min as u8) }
}

impl Logger for FilteringLogger {
    fn log(&self, level: LogLevel, message: &str) {
        if self.enabled(level) {
            println!("[{level:?}] {message}");
        }
    }
}

// Predefined filtering logger singletons
pub static LOGGER_TRACE: FilteringLogger = FilteringLogger { min: LogLevel::Trace };
pub static LOGGER_DEBUG: FilteringLogger = FilteringLogger { min: LogLevel::Debug };
pub static LOGGER_INFO: FilteringLogger = FilteringLogger { min: LogLevel::Info };
pub static LOGGER_WARN: FilteringLogger = FilteringLogger { min: LogLevel::Warn };
pub static LOGGER_ERROR: FilteringLogger = FilteringLogger { min: LogLevel::Error };

/// Logger handle type alias for ergonomic passing (cheap clone expected via &dyn usage)
pub type DynLogger = &'static dyn Logger;

/// Global default logger (NoOp) - optional convenience. Avoid in core; prefer explicit.
/// Kept simple; can be replaced with OnceCell if needed later.
pub static DEFAULT_LOGGER: NoOpLogger = NoOpLogger;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_op_logger_does_not_panic() {
        let logger = NoOpLogger;
        logger.info("test message");
    }

    #[test]
    fn stdout_logger_prints() {
        let logger = StdoutLogger;
        logger.warn("warn message");
        logger.error("error message");
    }
}
