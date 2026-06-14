use std::sync::Arc;

use actix_web::web;
use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};
use rust_learn::application::teacher_applications::decide_application::{
    TeacherApplicationDecisionCommand, TeacherApplicationDecisionError,
    TeacherApplicationDecisionUseCase,
};
use rust_learn::application::teacher_applications::TeacherApplicationOutput;

struct RouteOnlyTeacherApplicationDecisionUseCase;

pub fn teacher_application_decision_data() -> web::Data<Arc<dyn TeacherApplicationDecisionUseCase>>
{
    web::Data::new(Arc::new(RouteOnlyTeacherApplicationDecisionUseCase)
        as Arc<dyn TeacherApplicationDecisionUseCase>)
}

impl TeacherApplicationDecisionUseCase for RouteOnlyTeacherApplicationDecisionUseCase {
    fn decide_application(
        &self,
        _command: TeacherApplicationDecisionCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>> {
        async move { Ok(application()) }.boxed()
    }
}

fn application() -> TeacherApplicationOutput {
    let now = Utc::now();
    TeacherApplicationOutput {
        applicant_user_id: 10,
        created_at: now,
        decided_at: Some(now),
        decision_reason: Some("Route decision".to_string()),
        experience_summary: "Route decision".to_string(),
        id: 91,
        idempotency_key: None,
        organization_sponsor_id: None,
        portfolio_links: serde_json::json!([]),
        requested_course_id: None,
        requested_organization_id: None,
        requested_scope: "platform".to_string(),
        reviewer_id: Some(11),
        status: "approved".to_string(),
        updated_at: now,
    }
}
