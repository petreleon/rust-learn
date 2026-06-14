use futures::future::BoxFuture;

use crate::application::teacher_applications::get_my_application::{
    TeacherApplicationSelfError, TeacherApplicationSelfOutput,
};

pub trait TeacherApplicationSelfUseCase: Send + Sync {
    fn get_my_application(
        &self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<TeacherApplicationSelfOutput, TeacherApplicationSelfError>>;
}
