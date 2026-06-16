use bigdecimal::BigDecimal;
use chrono::Utc;
use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::learning::manage_course_completion_terms::*;
use crate::domain::learning::course::CourseCompletionTermsStatus;

#[derive(Default)]
pub struct FakeStore {
    pub allowed: Vec<String>,
    pub terms: Option<CourseCompletionTermsOutput>,
    pub created: Option<CourseCompletionTermsDraft>,
}

impl CourseCompletionTermsStore for FakeStore {
    fn course_context(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsCourseContext, CourseCompletionTermsError>> {
        ready(Ok(CourseCompletionTermsCourseContext {
            course_id,
            organization_id: Some(7),
        }))
        .boxed()
    }

    fn has_course_permission(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseCompletionTermsError>> {
        ready(Ok(self.allowed.iter().any(|item| item == permission))).boxed()
    }

    fn create_terms(
        &mut self,
        draft: CourseCompletionTermsDraft,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        self.created = Some(draft.clone());
        ready(Ok(output_from_draft(draft))).boxed()
    }

    fn terms_by_id(
        &mut self,
        _course_id: i32,
        _terms_id: i64,
    ) -> BoxFuture<'_, Result<Option<CourseCompletionTermsOutput>, CourseCompletionTermsError>>
    {
        ready(Ok(self.terms.clone())).boxed()
    }

    fn active_terms(
        &mut self,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Option<CourseCompletionTermsOutput>, CourseCompletionTermsError>>
    {
        ready(Ok(None)).boxed()
    }

    fn list_terms(
        &mut self,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseCompletionTermsOutput>, CourseCompletionTermsError>> {
        ready(Ok(Vec::new())).boxed()
    }

    fn list_audit_events(
        &mut self,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseCompletionTermsAuditEventOutput>, CourseCompletionTermsError>>
    {
        ready(Ok(Vec::new())).boxed()
    }

    fn activate_terms(
        &mut self,
        _terms_id: i64,
        _actor_user_id: i32,
        _note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        ready(Ok(self.terms.clone().unwrap())).boxed()
    }

    fn reject_terms(
        &mut self,
        _terms_id: i64,
        _actor_user_id: i32,
        _note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        ready(Ok(self.terms.clone().unwrap())).boxed()
    }

    fn withdraw_terms(
        &mut self,
        _terms_id: i64,
        _actor_user_id: i32,
        _note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        ready(Ok(self.terms.clone().unwrap())).boxed()
    }
}

pub fn terms_output(id: i64, status: CourseCompletionTermsStatus) -> CourseCompletionTermsOutput {
    CourseCompletionTermsOutput {
        id,
        course_id: 5,
        teacher_user_id: 11,
        organization_id: Some(7),
        version: 1,
        status,
        completion_reward_amount: BigDecimal::from(10),
        max_enrolled_students: 25,
        reward_policy_id: None,
        proposed_by_user_id: 11,
        accepted_by_user_id: None,
        accepted_at: None,
        activated_at: None,
        superseded_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn output_from_draft(draft: CourseCompletionTermsDraft) -> CourseCompletionTermsOutput {
    let mut output = terms_output(1, draft.status);
    output.teacher_user_id = draft.teacher_user_id;
    output.organization_id = draft.organization_id;
    output
}
