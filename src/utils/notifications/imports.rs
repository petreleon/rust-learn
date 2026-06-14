use crate::application::notifications::notification_inbox::NOTIFICATION_LIST_LIMIT;
use crate::db::DbPool;
use crate::models::notification::{NewNotification, Notification};
use anyhow::Result;
use diesel_async::AsyncPgConnection;

#[derive(Clone)]
pub struct NotificationsState {
    pool: DbPool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationMessage {
    pub title: &'static str,
    pub body: String,
}

fn compact_text(input: impl AsRef<str>, max_chars: usize) -> String {
    let input = input.as_ref().trim();
    if input.chars().count() <= max_chars {
        return input.to_string();
    }

    let mut compacted = input
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    compacted.push_str("...");
    compacted
}

pub fn enrollment_notification(
    course_id: i32,
    course_title: impl AsRef<str>,
) -> NotificationMessage {
    NotificationMessage {
        title: "course:enrolled",
        body: format!(
            "You were enrolled in course #{}: {}",
            course_id,
            compact_text(course_title, 120)
        ),
    }
}

pub fn content_published_notification(
    course_id: i32,
    content_id: i32,
    content_type: impl AsRef<str>,
) -> NotificationMessage {
    NotificationMessage {
        title: "content:published",
        body: format!(
            "New {} content was published for course #{} as content #{}.",
            compact_text(content_type, 80),
            course_id,
            content_id
        ),
    }
}

pub fn role_assignment_notification(
    scope: impl AsRef<str>,
    scope_id: Option<i32>,
    role_name: impl AsRef<str>,
) -> NotificationMessage {
    let scope = compact_text(scope, 80);
    let scope_label = match scope_id {
        Some(id) => format!("{} #{}", scope, id),
        None => scope,
    };

    NotificationMessage {
        title: "role:assigned",
        body: format!(
            "You were assigned the {} role for {}.",
            compact_text(role_name, 80),
            scope_label
        ),
    }
}

pub fn worker_failure_notification(
    job_id: i64,
    object: impl AsRef<str>,
    attempts: i32,
    error: impl AsRef<str>,
) -> NotificationMessage {
    NotificationMessage {
        title: "worker:job_failed",
        body: format!(
            "Upload job #{} failed permanently after {} attempts for {}. Error: {}",
            job_id,
            attempts,
            compact_text(object, 120),
            compact_text(error, 160)
        ),
    }
}

pub fn reward_event_notification(
    amount: impl AsRef<str>,
    event_type: impl AsRef<str>,
    transaction_id: Option<i64>,
) -> NotificationMessage {
    let transaction_label = transaction_id
        .map(|id| format!(" Transaction #{} was recorded.", id))
        .unwrap_or_default();

    NotificationMessage {
        title: "reward:recorded",
        body: format!(
            "Reward event {} recorded for {} LearnToken.{}",
            compact_text(event_type, 80),
            compact_text(amount, 80),
            transaction_label
        ),
    }
}

pub fn reward_wallet_credit_notification(
    course_id: i32,
    course_title: impl AsRef<str>,
    amount: impl AsRef<str>,
    wallet_id: i32,
    transaction_id: i64,
) -> NotificationMessage {
    NotificationMessage {
        title: "reward:wallet_credited",
        body: format!(
            "Reward for course #{} ({}) was credited: {} LearnToken to wallet #{}. Transaction #{} was recorded.",
            course_id,
            compact_text(course_title, 120),
            compact_text(amount, 80),
            wallet_id,
            transaction_id
        ),
    }
}
