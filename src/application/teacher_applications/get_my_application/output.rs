use crate::application::teacher_applications::TeacherApplicationAuditEventOutput;
use crate::application::teacher_applications::TeacherApplicationOutput;

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherApplicationSelfOutput {
    pub application: Option<TeacherApplicationOutput>,
    pub audit_events: Vec<TeacherApplicationAuditEventOutput>,
}
