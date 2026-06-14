use futures::future::BoxFuture;

use crate::application::teacher_applications::get_my_application::{
    TeacherApplicationAuditEventOutput, TeacherApplicationOutput, TeacherApplicationSelfError,
};

pub trait TeacherApplicationSelfStore {
    fn find_latest_application_for_applicant(
        &mut self,
        applicant_user_id: i32,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationSelfError>>;

    fn list_audit_events(
        &mut self,
        application_id: i64,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationSelfError>>;
}
