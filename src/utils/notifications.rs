use crate::db::DbPool;
use crate::models::notification::{NewNotification, Notification};
use anyhow::Result;

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

pub fn teacher_application_notification(
    application_id: i64,
    event_type: impl AsRef<str>,
    status: impl AsRef<str>,
    requested_scope: impl AsRef<str>,
    reason: Option<impl AsRef<str>>,
) -> NotificationMessage {
    let reason_label = reason
        .map(|value| format!(" Reason: {}", compact_text(value, 160)))
        .unwrap_or_default();

    NotificationMessage {
        title: "teacher_application:updated",
        body: format!(
            "Teacher application #{} {} with status {} for {} scope.{}",
            application_id,
            compact_text(event_type, 80),
            compact_text(status, 80),
            compact_text(requested_scope, 80),
            reason_label
        ),
    }
}

impl NotificationsState {
    /// Create a new NotificationsState from an existing DB pool.
    pub fn new(pool: DbPool) -> Self {
        NotificationsState { pool }
    }

    /// Send (add) a notification for a user.
    pub async fn send_notification(
        &self,
        user_id: i32,
        title: impl AsRef<str>,
        body: impl AsRef<str>,
    ) -> Result<i64> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        let new = NewNotification {
            user_id: Some(user_id),
            title: title.as_ref(),
            body: body.as_ref(),
        };
        let inserted_id = Notification::create(new, &mut conn).await?;
        Ok(inserted_id)
    }

    pub async fn send_event_notification(
        &self,
        user_id: i32,
        message: NotificationMessage,
    ) -> Result<i64> {
        self.send_notification(user_id, message.title, message.body)
            .await
    }

    pub async fn send_enrollment_notification(
        &self,
        user_id: i32,
        course_id: i32,
        course_title: impl AsRef<str>,
    ) -> Result<i64> {
        self.send_event_notification(user_id, enrollment_notification(course_id, course_title))
            .await
    }

    pub async fn send_content_published_notification(
        &self,
        user_id: i32,
        course_id: i32,
        content_id: i32,
        content_type: impl AsRef<str>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            content_published_notification(course_id, content_id, content_type),
        )
        .await
    }

    pub async fn send_role_assignment_notification(
        &self,
        user_id: i32,
        scope: impl AsRef<str>,
        scope_id: Option<i32>,
        role_name: impl AsRef<str>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            role_assignment_notification(scope, scope_id, role_name),
        )
        .await
    }

    pub async fn send_worker_failure_notification(
        &self,
        user_id: i32,
        job_id: i64,
        object: impl AsRef<str>,
        attempts: i32,
        error: impl AsRef<str>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            worker_failure_notification(job_id, object, attempts, error),
        )
        .await
    }

    pub async fn send_reward_event_notification(
        &self,
        user_id: i32,
        amount: impl AsRef<str>,
        event_type: impl AsRef<str>,
        transaction_id: Option<i64>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            reward_event_notification(amount, event_type, transaction_id),
        )
        .await
    }

    pub async fn send_teacher_application_notification(
        &self,
        user_id: i32,
        application_id: i64,
        event_type: impl AsRef<str>,
        status: impl AsRef<str>,
        requested_scope: impl AsRef<str>,
        reason: Option<impl AsRef<str>>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            teacher_application_notification(
                application_id,
                event_type,
                status,
                requested_scope,
                reason,
            ),
        )
        .await
    }

    /// Get notifications for a user ordered by created_at desc.
    pub async fn get_notifications(&self, user_id: i32) -> Result<Vec<Notification>> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        let rows = Notification::find_by_user_id(user_id, &mut conn).await?;
        Ok(rows)
    }

    /// Mark a notification read by its id.
    pub async fn mark_read(&self, user_id: i32, notification_id: i64) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        Notification::mark_as_read(user_id, notification_id, &mut conn).await?;
        Ok(())
    }

    /// Clear notifications for a user (delete).
    pub async fn clear(&self, user_id: i32) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        Notification::delete_by_user_id(user_id, &mut conn).await?;
        Ok(())
    }
}

impl From<DbPool> for NotificationsState {
    fn from(pool: DbPool) -> Self {
        NotificationsState::new(pool)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        content_published_notification, enrollment_notification, reward_event_notification,
        role_assignment_notification, teacher_application_notification,
        worker_failure_notification,
    };

    #[test]
    fn builds_requested_event_notification_messages() {
        assert_eq!(
            enrollment_notification(7, "Rust 101").title,
            "course:enrolled"
        );
        assert_eq!(
            content_published_notification(7, 9, "video").title,
            "content:published"
        );
        assert_eq!(
            role_assignment_notification("course", Some(7), "STUDENT").title,
            "role:assigned"
        );
        assert_eq!(
            worker_failure_notification(10, "object.mp4", 5, "ffmpeg failed").title,
            "worker:job_failed"
        );
        assert_eq!(
            reward_event_notification("42", "token_transfer", Some(99)).title,
            "reward:recorded"
        );
        assert_eq!(
            teacher_application_notification(
                55,
                "approved",
                "approved",
                "course",
                Some("approved by central administration")
            )
            .title,
            "teacher_application:updated"
        );
    }
}
