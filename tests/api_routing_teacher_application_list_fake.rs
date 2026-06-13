use std::sync::Arc;

use actix_web::web;
use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};
use rust_learn::application::teacher_applications::list_applications::{
    TeacherApplicationListError, TeacherApplicationListQuery, TeacherApplicationListUseCase,
};
use rust_learn::application::teacher_applications::TeacherApplicationOutput;

struct RouteOnlyTeacherApplicationListUseCase;

pub fn teacher_application_list_data() -> web::Data<Arc<dyn TeacherApplicationListUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyTeacherApplicationListUseCase) as Arc<dyn TeacherApplicationListUseCase>
    )
}

impl TeacherApplicationListUseCase for RouteOnlyTeacherApplicationListUseCase {
    fn list_applications(
        &self,
        _query: TeacherApplicationListQuery,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationOutput>, TeacherApplicationListError>> {
        async move { Ok(vec![application()]) }.boxed()
    }
}

fn application() -> TeacherApplicationOutput {
    let now = Utc::now();
    TeacherApplicationOutput {
        applicant_user_id: 10,
        created_at: now,
        decided_at: None,
        decision_reason: None,
        experience_summary: "Route smoke".to_string(),
        id: 90,
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
