use serde::Deserialize;

use crate::application::teacher_applications::decide_application::TeacherApplicationDecisionCommand;

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub(super) struct TeacherApplicationDecisionRequest {
    pub(super) status: String,
    pub(super) decision_reason: Option<String>,
}

impl TeacherApplicationDecisionRequest {
    pub(super) fn into_command(
        self,
        actor_user_id: i32,
        application_id: i64,
    ) -> TeacherApplicationDecisionCommand {
        TeacherApplicationDecisionCommand {
            actor_user_id,
            application_id,
            decision_reason: self.decision_reason,
            status: self.status,
        }
    }
}
