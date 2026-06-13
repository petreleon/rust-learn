use std::sync::Arc;

use actix_web::web;
use futures::future::{BoxFuture, FutureExt};
use rust_learn::application::teacher_applications::list_application_audit::{
    TeacherApplicationAuditError, TeacherApplicationAuditQuery, TeacherApplicationAuditUseCase,
};
use rust_learn::application::teacher_applications::TeacherApplicationAuditEventOutput;

struct RouteOnlyTeacherApplicationAuditUseCase;

pub fn teacher_application_audit_data() -> web::Data<Arc<dyn TeacherApplicationAuditUseCase>> {
    web::Data::new(Arc::new(RouteOnlyTeacherApplicationAuditUseCase)
        as Arc<dyn TeacherApplicationAuditUseCase>)
}

impl TeacherApplicationAuditUseCase for RouteOnlyTeacherApplicationAuditUseCase {
    fn list_application_audit(
        &self,
        _query: TeacherApplicationAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError>>
    {
        async move { Ok(Vec::new()) }.boxed()
    }
}
