use std::sync::Arc;

use actix_web::web;
use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};
use rust_learn::application::teacher_applications::submit_application::{
    TeacherApplicationSubmitCommand, TeacherApplicationSubmitError, TeacherApplicationSubmitUseCase,
};
use rust_learn::application::teacher_applications::TeacherApplicationOutput;

struct RouteOnlyTeacherApplicationSubmitUseCase;

pub fn teacher_application_submit_data() -> web::Data<Arc<dyn TeacherApplicationSubmitUseCase>> {
    web::Data::new(Arc::new(RouteOnlyTeacherApplicationSubmitUseCase)
        as Arc<dyn TeacherApplicationSubmitUseCase>)
}

impl TeacherApplicationSubmitUseCase for RouteOnlyTeacherApplicationSubmitUseCase {
    fn submit_application(
        &self,
        _command: TeacherApplicationSubmitCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationSubmitError>> {
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
        experience_summary: "Route submit".to_string(),
        id: 91,
        idempotency_key: None,
        organization_sponsor_id: None,
        portfolio_links: serde_json::json!([]),
        requested_course_id: None,
        requested_organization_id: None,
        requested_scope: "platform".to_string(),
        reviewer_id: None,
        status: "submitted".to_string(),
        updated_at: now,
    }
}
