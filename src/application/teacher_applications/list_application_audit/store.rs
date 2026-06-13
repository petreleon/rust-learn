use futures::future::BoxFuture;

use crate::application::teacher_applications::{
    list_application_audit::TeacherApplicationAuditError, TeacherApplicationAuditEventOutput,
};

pub trait TeacherApplicationAuditStore {
    fn can_review_teacher_applications(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationAuditError>>;

    fn list_audit_events(
        &mut self,
        application_id: i64,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError>>;
}
