use chrono::{DateTime, Utc};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentReadError, LearnerAssessmentQuestionOutput,
};
use crate::application::learning::delete_course::{
    delete_course, CourseDeletionError, CourseDeletionOutcome, CourseDeletionStore,
};
use crate::application::learning::list_assessment_attempts::list_user_assessment_attempts;
use crate::application::learning::list_course_assessments::list_published_course_assessments;
use crate::application::learning::list_course_organizations::{
    list_course_organizations, CourseOrganizationOutput, CourseOrganizationReadError,
    CourseOrganizationStore,
};
use crate::application::learning::ports::AssessmentReadStore;

#[derive(Default)]
struct FakeAssessmentReadStore {
    assessments: Vec<AssessmentOutput>,
    attempts: Vec<AssessmentAttemptOutput>,
    questions: Vec<LearnerAssessmentQuestionOutput>,
    requested_course_id: Option<i32>,
    requested_attempts: Option<(i32, i32)>,
    requested_question_ids: Vec<i32>,
}

#[derive(Default)]
struct FakeCourseOrganizationStore {
    organizations: Vec<CourseOrganizationOutput>,
    requested_course_id: Option<i32>,
}

struct FakeCourseDeletionStore {
    outcome: CourseDeletionOutcome,
    requested_course_id: Option<i32>,
}

impl CourseDeletionStore for FakeCourseDeletionStore {
    fn delete(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<CourseDeletionOutcome, CourseDeletionError>> {
        self.requested_course_id = Some(course_id);
        let outcome = self.outcome;
        async move { Ok(outcome) }.boxed()
    }
}

impl CourseOrganizationStore for FakeCourseOrganizationStore {
    fn list_for_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseOrganizationOutput>, CourseOrganizationReadError>> {
        self.requested_course_id = Some(course_id);
        let organizations = self.organizations.clone();
        async move { Ok(organizations) }.boxed()
    }
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

    fn list_questions_for_assessments(
        &mut self,
        assessment_ids: Vec<i32>,
    ) -> BoxFuture<'_, Result<Vec<LearnerAssessmentQuestionOutput>, AssessmentReadError>> {
        self.requested_question_ids = assessment_ids;
        let questions = self.questions.clone();
        async move { Ok(questions) }.boxed()
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
        questions: Vec::new(),
    };
    let expected_question = LearnerAssessmentQuestionOutput {
        id: 101,
        assessment_id: 11,
        text: "What does ownership prevent?".to_string(),
        question_type: "short_text".to_string(),
        options: None,
        points: 1,
        order: 0,
    };
    let mut store = FakeAssessmentReadStore {
        assessments: vec![expected.clone()],
        questions: vec![expected_question.clone()],
        ..Default::default()
    };

    let result = list_published_course_assessments(&mut store, 7)
        .await
        .expect("assessments should load");
    let mut expected_with_questions = expected;
    expected_with_questions.questions = vec![expected_question];

    assert_eq!(result, vec![expected_with_questions]);
    assert_eq!(store.requested_course_id, Some(7));
    assert_eq!(store.requested_question_ids, vec![11]);
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

#[tokio::test]
async fn list_course_organizations_uses_course_organization_port() {
    let expected = CourseOrganizationOutput {
        id: 9,
        name: "Learning Org".to_string(),
        website_link: Some("https://example.test".to_string()),
        profile_url: None,
    };
    let mut store = FakeCourseOrganizationStore {
        organizations: vec![expected.clone()],
        ..Default::default()
    };

    let result = list_course_organizations(&mut store, 7)
        .await
        .expect("course organizations should load");

    assert_eq!(result, vec![expected]);
    assert_eq!(store.requested_course_id, Some(7));
}

#[tokio::test]
async fn delete_course_uses_course_deletion_port() {
    let mut store = FakeCourseDeletionStore {
        outcome: CourseDeletionOutcome::Deleted,
        requested_course_id: None,
    };

    let result = delete_course(&mut store, 17)
        .await
        .expect("course delete should run");

    assert_eq!(result, CourseDeletionOutcome::Deleted);
    assert_eq!(store.requested_course_id, Some(17));
}
