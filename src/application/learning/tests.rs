use chrono::{DateTime, Utc};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentReadError,
};
use crate::application::learning::list_assessment_attempts::list_user_assessment_attempts;
use crate::application::learning::list_course_assessments::list_published_course_assessments;
use crate::application::learning::ports::AssessmentReadStore;

#[derive(Default)]
struct FakeAssessmentReadStore {
    assessments: Vec<AssessmentOutput>,
    attempts: Vec<AssessmentAttemptOutput>,
    requested_course_id: Option<i32>,
    requested_attempts: Option<(i32, i32)>,
}

impl AssessmentReadStore for FakeAssessmentReadStore {
    fn list_published_for_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentOutput>, AssessmentReadError>> {
        self.requested_course_id = Some(course_id);
        let assessments = self.assessments.clone();
        async move { Ok(assessments) }.boxed()
    }

    fn list_attempts_for_user(
        &mut self,
        assessment_id: i32,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentAttemptOutput>, AssessmentReadError>> {
        self.requested_attempts = Some((assessment_id, user_id));
        let attempts = self.attempts.clone();
        async move { Ok(attempts) }.boxed()
    }
}

#[tokio::test]
async fn list_published_course_assessments_uses_assessment_read_port() {
    let now = Utc::now();
    let expected = AssessmentOutput {
        id: 11,
        course_id: 7,
        title: "Ownership quiz".to_string(),
        description: None,
        passing_score: 70,
        max_attempts: 3,
        published: true,
        created_at: now,
        updated_at: now,
    };
    let mut store = FakeAssessmentReadStore {
        assessments: vec![expected.clone()],
        ..Default::default()
    };

    let result = list_published_course_assessments(&mut store, 7)
        .await
        .expect("assessments should load");

    assert_eq!(result, vec![expected]);
    assert_eq!(store.requested_course_id, Some(7));
}

#[tokio::test]
async fn list_user_assessment_attempts_uses_assessment_read_port() {
    let now: DateTime<Utc> = Utc::now();
    let expected = AssessmentAttemptOutput {
        id: 21,
        assessment_id: 11,
        user_id: 5,
        score: Some(9),
        passed: Some(true),
        started_at: now,
        completed_at: Some(now),
    };
    let mut store = FakeAssessmentReadStore {
        attempts: vec![expected.clone()],
        ..Default::default()
    };

    let result = list_user_assessment_attempts(&mut store, 11, 5)
        .await
        .expect("attempts should load");

    assert_eq!(result, vec![expected]);
    assert_eq!(store.requested_attempts, Some((11, 5)));
}
