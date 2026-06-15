use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::teacher_applications::{
    list_application_audit::TeacherApplicationAuditError, TeacherApplicationAuditEventOutput,
};

pub trait TeacherApplicationAuditStore:
    AccessDecisionStore<Error = TeacherApplicationAuditError>
{
    fn list_audit_events(
        &mut self,
        application_id: i64,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError>>;
}
