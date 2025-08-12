//! Clock abstraction for time-related functionality
//!
//! Provides a `Clock` capability trait enabling deterministic tests and effect isolation.
//! This replaces ad-hoc `SystemTime::now()` / `Instant::now()` usage inside core logic.
//!
//! Alternatives (minority reports) summarized in `analysis/ADT-alternatives.md` (capabilities section forthcoming):
//! * Direct system calls (rejected: impure, hard to test)
//! * Global static clock (rejected: hidden dependency, complicates concurrency)

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Trait for clock functionality – enables dependency injection for
/// deterministic testing and effect isolation.
///
/// Implementations should be cheap to clone/copy and free of global mutable
/// state. Prefer passing a `&dyn Clock` (or concrete impl) explicitly through
/// orchestration layers instead of calling `SystemTime::now()` / `Instant::now()`
/// directly inside pure logic.
pub trait Clock: Send + Sync {
    /// Get current system time
    fn system_time(&self) -> SystemTime;

    /// Get current instant for elapsed time measurements
    fn instant_now(&self) -> Instant;

    /// Get current timestamp as seconds since UNIX epoch
    fn timestamp_secs(&self) -> u64 {
        self.system_time()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }
}

/// Real clock implementation for production use
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn system_time(&self) -> SystemTime {
        SystemTime::now()
    }

    fn instant_now(&self) -> Instant {
        Instant::now()
    }
}

/// Test clock implementation for deterministic testing
#[derive(Debug, Clone)]
pub struct TestClock {
    fixed_time: SystemTime,
    fixed_instant: Instant,
}

impl TestClock {
    /// Create a test clock with fixed time
    #[must_use]
    pub fn new(fixed_time: SystemTime) -> Self {
        Self {
            fixed_time,
            fixed_instant: Instant::now(), // This will be overridden anyway in tests
        }
    }

    /// Create a test clock with current time (for initialization)
    #[must_use]
    pub fn now() -> Self {
        Self::new(SystemTime::now())
    }
}

impl Clock for TestClock {
    fn system_time(&self) -> SystemTime {
        self.fixed_time
    }

    fn instant_now(&self) -> Instant {
        self.fixed_instant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_clock() {
        let clock = SystemClock;
        let time1 = clock.system_time();
        let time2 = clock.system_time();

        // Time should advance (though this could theoretically fail on very fast systems)
        assert!(time2 >= time1);
    }

    #[test]
    fn test_test_clock_deterministic() {
        let fixed_time = UNIX_EPOCH + Duration::from_secs(1_000_000);
        let clock = TestClock::new(fixed_time);

        // Test clock should always return the same time
        assert_eq!(clock.system_time(), fixed_time);
        assert_eq!(clock.system_time(), fixed_time);
        assert_eq!(clock.timestamp_secs(), 1_000_000);
    }
}
