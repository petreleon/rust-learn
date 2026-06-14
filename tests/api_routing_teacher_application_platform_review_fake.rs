use std::sync::Arc;

use actix_web::web;
use futures::future::{BoxFuture, FutureExt};
use rust_learn::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewOutput, TeacherApplicationPlatformReviewQuery,
    TeacherApplicationPlatformReviewSummaryOutput, TeacherApplicationPlatformReviewUseCase,
};

struct RouteOnlyTeacherApplicationPlatformReviewUseCase;

pub fn teacher_application_platform_review_data(
) -> web::Data<Arc<dyn TeacherApplicationPlatformReviewUseCase>> {
    web::Data::new(Arc::new(RouteOnlyTeacherApplicationPlatformReviewUseCase)
        as Arc<dyn TeacherApplicationPlatformReviewUseCase>)
}

impl TeacherApplicationPlatformReviewUseCase for RouteOnlyTeacherApplicationPlatformReviewUseCase {
    fn list_platform_review_applications(
        &self,
        _query: TeacherApplicationPlatformReviewQuery,
    ) -> BoxFuture<
        '_,
        Result<
            TeacherApplicationPlatformReviewOutput,
            rust_learn::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewError,
        >,
    >{
        async move {
            Ok(TeacherApplicationPlatformReviewOutput {
                applications: Vec::new(),
                limit: 25,
                offset: 0,
                operator_permissions: Default::default(),
                search: None,
                status: None,
                summary: TeacherApplicationPlatformReviewSummaryOutput::default(),
                total: 0,
            })
        }
        .boxed()
    }
}
