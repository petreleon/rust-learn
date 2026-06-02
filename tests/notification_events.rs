use chrono::NaiveDate;
use diesel_async::AsyncPgConnection;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::user::User;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::notifications::NotificationsState;
use std::collections::HashSet;

fn unique_email(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}+{}-{}@example.com", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection) -> User {
    let email = unique_email("notification-events");
    create_user(
        conn,
        "Notification Events",
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
}

#[actix_web::test]
async fn event_notification_helpers_persist_requested_event_types() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let user = create_test_user(&mut conn).await;
    drop(conn);

    let notifications = NotificationsState::new(pool.clone());
    notifications
        .send_enrollment_notification(user.id(), 11, "Rust Foundations")
        .await
        .expect("enrollment notification should persist");
    notifications
        .send_content_published_notification(user.id(), 11, 22, "video")
        .await
        .expect("content notification should persist");
    notifications
        .send_role_assignment_notification(user.id(), "course", Some(11), "STUDENT")
        .await
        .expect("role notification should persist");
    notifications
        .send_worker_failure_notification(user.id(), 33, "courses/11/video.mp4", 5, "ffmpeg failed")
        .await
        .expect("worker failure notification should persist");
    notifications
        .send_reward_event_notification(user.id(), "42", "token_transfer", Some(44))
        .await
        .expect("reward notification should persist");
    notifications
        .send_teacher_application_notification(
            user.id(),
            55,
            "approved",
            "approved",
            "course",
            Some("approved by central administration"),
        )
        .await
        .expect("teacher application notification should persist");

    let rows = notifications
        .get_notifications(user.id())
        .await
        .expect("notifications should load");
    let titles: HashSet<_> = rows.iter().map(|row| row.title.as_str()).collect();

    assert!(titles.contains("course:enrolled"));
    assert!(titles.contains("content:published"));
    assert!(titles.contains("role:assigned"));
    assert!(titles.contains("worker:job_failed"));
    assert!(titles.contains("reward:recorded"));
    assert!(titles.contains("teacher_application:updated"));
}
