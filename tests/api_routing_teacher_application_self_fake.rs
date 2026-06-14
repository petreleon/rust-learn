use std::sync::Arc;

use actix_web::web;
use futures::future::{BoxFuture, FutureExt};
use rust_learn::application::teacher_applications::get_my_application::{
    TeacherApplicationSelfError, TeacherApplicationSelfOutput, TeacherApplicationSelfUseCase,
};

struct RouteOnlyTeacherApplicationSelfUseCase;

pub fn teacher_application_self_data() -> web::Data<Arc<dyn TeacherApplicationSelfUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyTeacherApplicationSelfUseCase) as Arc<dyn TeacherApplicationSelfUseCase>
    )
}

impl TeacherApplicationSelfUseCase for RouteOnlyTeacherApplicationSelfUseCase {
    fn get_my_application(
        &self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<TeacherApplicationSelfOutput, TeacherApplicationSelfError>> {
        async move {
            Ok(TeacherApplicationSelfOutput {
                application: None,
                audit_events: Vec::new(),
            })
        }
        .boxed()
    }
}
