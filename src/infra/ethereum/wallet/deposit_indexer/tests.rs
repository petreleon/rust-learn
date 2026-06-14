use super::poll_logging::{should_log_indexer_poll, PollFailureLogLevel, PollFailureLogState};
use std::time::{Duration, Instant};

#[test]
fn logs_first_idle_indexer_poll() {
    assert!(should_log_indexer_poll(
        0,
        None,
        Duration::from_secs(60),
        Instant::now()
    ));
}

#[test]
fn throttles_idle_indexer_poll_until_interval_elapses() {
    let now = Instant::now();

    assert!(!should_log_indexer_poll(
        0,
        Some(now - Duration::from_secs(30)),
        Duration::from_secs(60),
        now
    ));
    assert!(should_log_indexer_poll(
        0,
        Some(now - Duration::from_secs(60)),
        Duration::from_secs(60),
        now
    ));
}

#[test]
fn logs_non_idle_indexer_poll_immediately() {
    let now = Instant::now();

    assert!(should_log_indexer_poll(
        1,
        Some(now),
        Duration::from_secs(60),
        now
    ));
}

#[test]
fn logs_first_indexer_poll_failure_as_retrying_info() {
    let now = Instant::now();
    let mut state = PollFailureLogState::default();

    let decision = state.record_failure(now, Duration::from_secs(60));

    assert_eq!(decision.level, PollFailureLogLevel::Info);
    assert_eq!(decision.consecutive_failures, 1);
    assert_eq!(decision.suppressed_failure_count, 0);
    assert_eq!(decision.outage_seconds, 0);
}

#[test]
fn suppresses_transient_indexer_poll_failures_before_warning_interval() {
    let now = Instant::now();
    let mut state = PollFailureLogState::default();
    state.record_failure(now, Duration::from_secs(60));

    let decision = state.record_failure(now + Duration::from_secs(30), Duration::from_secs(60));

    assert_eq!(decision.level, PollFailureLogLevel::Suppress);
    assert_eq!(decision.consecutive_failures, 2);
    assert_eq!(decision.suppressed_failure_count, 1);
    assert_eq!(decision.outage_seconds, 30);
}

#[test]
fn warns_for_sustained_indexer_poll_failures_at_configured_interval() {
    let now = Instant::now();
    let mut state = PollFailureLogState::default();
    state.record_failure(now, Duration::from_secs(60));
    state.record_failure(now + Duration::from_secs(30), Duration::from_secs(60));

    let decision = state.record_failure(now + Duration::from_secs(60), Duration::from_secs(60));

    assert_eq!(decision.level, PollFailureLogLevel::Warn);
    assert_eq!(decision.consecutive_failures, 3);
    assert_eq!(decision.suppressed_failure_count, 1);
    assert_eq!(decision.outage_seconds, 60);
}

#[test]
fn throttles_sustained_indexer_poll_failure_warnings() {
    let now = Instant::now();
    let mut state = PollFailureLogState::default();
    state.record_failure(now, Duration::from_secs(60));
    state.record_failure(now + Duration::from_secs(30), Duration::from_secs(60));
    state.record_failure(now + Duration::from_secs(60), Duration::from_secs(60));

    let suppressed = state.record_failure(now + Duration::from_secs(75), Duration::from_secs(60));
    let warned_again =
        state.record_failure(now + Duration::from_secs(120), Duration::from_secs(60));

    assert_eq!(suppressed.level, PollFailureLogLevel::Suppress);
    assert_eq!(suppressed.suppressed_failure_count, 1);
    assert_eq!(warned_again.level, PollFailureLogLevel::Warn);
    assert_eq!(warned_again.consecutive_failures, 5);
    assert_eq!(warned_again.suppressed_failure_count, 1);
    assert_eq!(warned_again.outage_seconds, 120);
}

#[test]
fn logs_indexer_poll_recovery_and_resets_failure_state() {
    let now = Instant::now();
    let mut state = PollFailureLogState::default();
    state.record_failure(now, Duration::from_secs(60));
    state.record_failure(now + Duration::from_secs(30), Duration::from_secs(60));

    let recovery = state
        .record_success(now + Duration::from_secs(45))
        .expect("failure state should record a recovery");

    assert_eq!(recovery.consecutive_failures, 2);
    assert_eq!(recovery.suppressed_failure_count, 1);
    assert_eq!(recovery.outage_seconds, 45);
    assert!(state
        .record_success(now + Duration::from_secs(46))
        .is_none());
}
