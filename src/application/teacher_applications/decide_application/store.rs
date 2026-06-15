use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::teacher_applications::{
    decide_application::TeacherApplicationDecisionError, TeacherApplicationOutput,
};

pub trait TeacherApplicationDecisionStore:
    AccessDecisionStore<Error = TeacherApplicationDecisionError>
{
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
