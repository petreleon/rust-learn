use futures::future::BoxFuture;

use crate::application::teacher_applications::{
    decide_application::{TeacherApplicationDecisionCommand, TeacherApplicationDecisionError},
    TeacherApplicationOutput,
};

pub trait TeacherApplicationDecisionUseCase: Send + Sync {
    fn decide_application(
        &self,
        command: TeacherApplicationDecisionCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>>;
}
