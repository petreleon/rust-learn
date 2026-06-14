use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::delete_course::{
    CourseDeletionError, CourseDeletionOutcome, CourseDeletionUseCase,
};

struct RouteOnlyCourseDeletionUseCase;

pub fn course_deletion_data() -> web::Data<Arc<dyn CourseDeletionUseCase>> {
    web::Data::new(Arc::new(RouteOnlyCourseDeletionUseCase) as Arc<dyn CourseDeletionUseCase>)
}

impl CourseDeletionUseCase for RouteOnlyCourseDeletionUseCase {
    fn delete_course(
        &self,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<CourseDeletionOutcome, CourseDeletionError>> {
        ready(Ok(CourseDeletionOutcome::Deleted)).boxed()
    }
}
