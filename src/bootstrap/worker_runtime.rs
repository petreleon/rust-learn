use chrono::{DateTime, Duration, Utc};
use std::io;
use std::path::Path;
use tokio::fs;

pub const DEFAULT_WORKER_HEARTBEAT_PATH: &str = "/tmp/worker_alive";

pub fn positive_usize_from_env_value(value: Option<&str>, default: usize) -> usize {
    value
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

pub fn positive_i64_from_env_value(value: Option<&str>, default: i64) -> i64 {
    value
        .and_then(|raw| raw.trim().parse::<i64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

pub fn positive_u64_from_env_value(value: Option<&str>, default: u64) -> u64 {
    value
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

pub fn retry_backoff_seconds(base_backoff_seconds: u64, current_attempts: i64) -> u64 {
    let exponent = current_attempts.max(0) as u32;
    base_backoff_seconds.saturating_mul(2u64.saturating_pow(exponent))
}

pub fn retry_available_at(
    now: DateTime<Utc>,
    base_backoff_seconds: u64,
    current_attempts: i64,
) -> DateTime<Utc> {
    let backoff = retry_backoff_seconds(base_backoff_seconds, current_attempts);
    now.checked_add_signed(Duration::seconds(backoff as i64))
        .unwrap_or(now)
}

pub async fn write_heartbeat(path: impl AsRef<Path>) -> io::Result<()> {
    fs::write(path, Utc::now().timestamp().to_string()).await
}

#[cfg(test)]
mod tests {
    use super::{
        positive_i64_from_env_value, positive_u64_from_env_value, positive_usize_from_env_value,
        retry_available_at, retry_backoff_seconds, write_heartbeat,
    };
    use chrono::{Duration, Utc};
    use std::fs;

    #[test]
    fn parses_positive_worker_numeric_config_values() {
        assert_eq!(positive_usize_from_env_value(Some(" 3 "), 1), 3);
        assert_eq!(positive_i64_from_env_value(Some("7"), 5), 7);
        assert_eq!(positive_u64_from_env_value(Some("90"), 60), 90);
    }

    #[test]
    fn falls_back_for_invalid_worker_numeric_config_values() {
        assert_eq!(positive_usize_from_env_value(None, 1), 1);
        assert_eq!(positive_usize_from_env_value(Some("0"), 1), 1);
        assert_eq!(positive_i64_from_env_value(Some("-1"), 5), 5);
        assert_eq!(positive_u64_from_env_value(Some("not-a-number"), 60), 60);
    }

    #[test]
    fn calculates_exponential_retry_backoff_from_current_attempts() {
        assert_eq!(retry_backoff_seconds(60, 0), 60);
        assert_eq!(retry_backoff_seconds(60, 1), 120);
        assert_eq!(retry_backoff_seconds(60, 2), 240);
        assert_eq!(retry_backoff_seconds(60, -1), 60);
    }

    #[test]
    fn calculates_retry_available_at_from_backoff() {
        let now = Utc::now();
        assert_eq!(retry_available_at(now, 30, 2), now + Duration::seconds(120));
    }

    #[tokio::test]
    async fn write_heartbeat_writes_current_epoch_timestamp() {
        let path = std::env::temp_dir().join(format!(
            "rust-learn-worker-heartbeat-{}-{}",
            std::process::id(),
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let before = Utc::now().timestamp();

        write_heartbeat(&path)
            .await
            .expect("heartbeat write should succeed");

        let contents = fs::read_to_string(&path).expect("heartbeat file should be readable");
        let timestamp = contents
            .parse::<i64>()
            .expect("heartbeat should be an epoch timestamp");
        let after = Utc::now().timestamp();

        assert!(timestamp >= before);
        assert!(timestamp <= after);

        let _ = fs::remove_file(path);
    }
}
