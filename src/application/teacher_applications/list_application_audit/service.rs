use futures::future::BoxFuture;

use crate::application::teacher_applications::{
    list_application_audit::{TeacherApplicationAuditError, TeacherApplicationAuditQuery},
    TeacherApplicationAuditEventOutput,
};

pub trait TeacherApplicationAuditUseCase: Send + Sync {
    fn list_application_audit(
        &self,
        query: TeacherApplicationAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError>>;
}
