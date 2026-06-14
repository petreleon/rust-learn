use futures::future::BoxFuture;

use crate::application::teacher_applications::{
    nominate_application::{
        TeacherApplicationNominationCommand, TeacherApplicationNominationError,
    },
    TeacherApplicationOutput,
};

pub trait TeacherApplicationNominationUseCase: Send + Sync {
    fn nominate_application(
        &self,
        command: TeacherApplicationNominationCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationNominationError>>;
}
