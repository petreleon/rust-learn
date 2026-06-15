use futures::future::{BoxFuture, FutureExt};

use super::*;
use crate::domain::teacher_applications::audit::TeacherApplicationAuditEventType;

struct FakeStore {
    platform_ids: Result<Vec<i32>, TeacherApplicationNotificationError>,
    organization_ids: Result<Vec<i32>, TeacherApplicationNotificationError>,
    send_fail_for: Option<i32>,
    sent: Vec<i32>,
}

impl Default for FakeStore {
    fn default() -> Self {
        Self {
            platform_ids: Ok(Vec::new()),
            organization_ids: Ok(Vec::new()),
            send_fail_for: None,
            sent: Vec::new(),
        }
    }
}

impl TeacherApplicationNotificationStore for FakeStore {
    fn platform_reviewer_ids(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<i32>, TeacherApplicationNotificationError>> {
        async move { self.platform_ids.clone() }.boxed()
    }

    fn organization_viewer_ids(
        &mut self,
        _: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, TeacherApplicationNotificationError>> {
        async move { self.organization_ids.clone() }.boxed()
    }

    fn send(
        &mut self,
        recipient_user_id: i32,
        _: TeacherApplicationNotificationCommand,
    ) -> BoxFuture<'_, Result<(), TeacherApplicationNotificationError>> {
        if self.send_fail_for == Some(recipient_user_id) {
            async move {
                Err(TeacherApplicationNotificationError::Delivery(
                    "send failed".to_string(),
                ))
            }
            .boxed()
        } else {
            self.sent.push(recipient_user_id);
            async move { Ok(()) }.boxed()
        }
    }
}

#[tokio::test]
async fn dedupes_applicant_platform_and_organization_recipients() {
    let mut store = FakeStore {
        platform_ids: Ok(vec![7, 9]),
        organization_ids: Ok(vec![9, 11]),
        ..Default::default()
    };

    let outcome = notify_application_event(&mut store, command())
        .await
        .unwrap();

    assert_eq!(store.sent, vec![7, 9, 11]);
    assert_eq!(outcome.recipient_count, 3);
    assert_eq!(outcome.sent_count, 3);
    assert_eq!(outcome.failed_count, 0);
}

#[tokio::test]
async fn continues_when_lookup_and_send_failures_happen() {
    let mut store = FakeStore {
        platform_ids: Err(TeacherApplicationNotificationError::Database(
            "lookup failed".to_string(),
        )),
        organization_ids: Ok(vec![9, 11]),
        send_fail_for: Some(11),
        ..Default::default()
    };

    let outcome = notify_application_event(&mut store, command())
        .await
        .unwrap();

    assert_eq!(store.sent, vec![7, 9]);
    assert_eq!(outcome.recipient_count, 3);
    assert_eq!(outcome.sent_count, 2);
    assert_eq!(outcome.failed_count, 2);
}

fn command() -> TeacherApplicationNotificationCommand {
    TeacherApplicationNotificationCommand {
        applicant_user_id: 7,
        application_id: 101,
        event_type: TeacherApplicationAuditEventType::Submitted,
        organization_sponsor_id: Some(44),
        reason: None,
        requested_organization_id: Some(44),
        requested_scope: "organization".to_string(),
        status: "submitted".to_string(),
    }
}
