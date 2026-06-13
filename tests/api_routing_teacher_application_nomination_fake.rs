use std::sync::Arc;

use actix_web::web;
use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};
use rust_learn::application::teacher_applications::nominate_application::{
    TeacherApplicationNominationCommand, TeacherApplicationNominationError,
    TeacherApplicationNominationUseCase,
};
use rust_learn::application::teacher_applications::TeacherApplicationOutput;

struct RouteOnlyTeacherApplicationNominationUseCase;

pub fn teacher_application_nomination_data(
) -> web::Data<Arc<dyn TeacherApplicationNominationUseCase>> {
    web::Data::new(Arc::new(RouteOnlyTeacherApplicationNominationUseCase)
        as Arc<dyn TeacherApplicationNominationUseCase>)
}

impl TeacherApplicationNominationUseCase for RouteOnlyTeacherApplicationNominationUseCase {
    fn nominate_application(
        &self,
        _command: TeacherApplicationNominationCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationNominationError>> {
        async move { Ok(application()) }.boxed()
    }
}

fn application() -> TeacherApplicationOutput {
    let now = Utc::now();
    TeacherApplicationOutput {
        applicant_user_id: 10,
        created_at: now,
        decided_at: None,
        decision_reason: None,
        experience_summary: "Route nomination".to_string(),
        id: 91,
        idempotency_key: None,
        organization_sponsor_id: Some(56),
        portfolio_links: serde_json::json!([]),
        requested_course_id: None,
        requested_organization_id: Some(56),
        requested_scope: "organization".to_string(),
        reviewer_id: None,
        status: "submitted".to_string(),
        updated_at: now,
    }
}
