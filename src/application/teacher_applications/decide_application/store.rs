use futures::future::BoxFuture;

use crate::application::teacher_applications::{
    decide_application::TeacherApplicationDecisionError, TeacherApplicationOutput,
};

pub trait TeacherApplicationDecisionStore {
    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: String,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationDecisionError>>;

    fn application(
        &mut self,
        application_id: i64,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>>;

    fn apply_decision(
        &mut self,
        actor_user_id: i32,
        current: TeacherApplicationOutput,
        target_status: String,
        decision_reason: Option<String>,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>>;
}
