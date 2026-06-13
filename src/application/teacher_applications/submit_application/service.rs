use futures::future::BoxFuture;

use crate::application::teacher_applications::{
    submit_application::{TeacherApplicationSubmitCommand, TeacherApplicationSubmitError},
    TeacherApplicationOutput,
};

pub trait TeacherApplicationSubmitUseCase: Send + Sync {
    fn submit_application(
        &self,
        command: TeacherApplicationSubmitCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationSubmitError>>;
}
