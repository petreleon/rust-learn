use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PollFailureLogLevel {
    Info,
    Warn,
    Suppress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PollFailureLogDecision {
    pub(super) level: PollFailureLogLevel,
    pub(super) consecutive_failures: u64,
    pub(super) suppressed_failure_count: u64,
    pub(super) outage_seconds: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PollFailureRecovery {
    pub(super) consecutive_failures: u64,
    pub(super) suppressed_failure_count: u64,
    pub(super) outage_seconds: u64,
}

#[derive(Debug, Default)]
pub(super) struct PollFailureLogState {
    first_failure_at: Option<Instant>,
    last_warning_at: Option<Instant>,
    consecutive_failures: u64,
    suppressed_failure_count: u64,
}

pub(super) fn should_log_indexer_poll(
    credited_count: usize,
    last_idle_log: Option<Instant>,
    idle_log_interval: Duration,
    now: Instant,
) -> bool {
    if credited_count > 0 {
        return true;
    }

    last_idle_log
        .map(|logged_at| now.duration_since(logged_at) >= idle_log_interval)
        .unwrap_or(true)
}

impl PollFailureLogState {
    pub(super) fn record_failure(
        &mut self,
        now: Instant,
        warn_interval: Duration,
    ) -> PollFailureLogDecision {
        let first_failure_at = *self.first_failure_at.get_or_insert(now);
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        let outage = now.saturating_duration_since(first_failure_at);

        let warn_due = outage >= warn_interval
            && self
                .last_warning_at
                .map(|last_warning_at| {
                    now.saturating_duration_since(last_warning_at) >= warn_interval
                })
                .unwrap_or(true);

        if warn_due {
            let suppressed_failure_count = self.suppressed_failure_count;
            self.suppressed_failure_count = 0;
            self.last_warning_at = Some(now);
            return PollFailureLogDecision {
                level: PollFailureLogLevel::Warn,
                consecutive_failures: self.consecutive_failures,
                suppressed_failure_count,
                outage_seconds: outage.as_secs(),
            };
        }

        if self.consecutive_failures == 1 {
            return PollFailureLogDecision {
                level: PollFailureLogLevel::Info,
                consecutive_failures: self.consecutive_failures,
                suppressed_failure_count: 0,
                outage_seconds: outage.as_secs(),
            };
        }

        self.suppressed_failure_count = self.suppressed_failure_count.saturating_add(1);
        PollFailureLogDecision {
            level: PollFailureLogLevel::Suppress,
            consecutive_failures: self.consecutive_failures,
            suppressed_failure_count: self.suppressed_failure_count,
            outage_seconds: outage.as_secs(),
        }
    }

    pub(super) fn record_success(&mut self, now: Instant) -> Option<PollFailureRecovery> {
        let first_failure_at = self.first_failure_at?;
        let recovery = PollFailureRecovery {
            consecutive_failures: self.consecutive_failures,
            suppressed_failure_count: self.suppressed_failure_count,
            outage_seconds: now.saturating_duration_since(first_failure_at).as_secs(),
        };
        *self = Self::default();
        Some(recovery)
    }
}
