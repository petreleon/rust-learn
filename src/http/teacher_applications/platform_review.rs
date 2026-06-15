use std::sync::Arc;

use actix_web::web;

use crate::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::errors::platform_review_error;
use crate::http::teacher_applications::platform_review_dto::{
    PlatformTeacherApplicationsParams, PlatformTeacherApplicationsResponse,
};

pub(super) async fn list_platform_review_applications(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationPlatformReviewUseCase>>,
    query: web::Query<PlatformTeacherApplicationsParams>,
) -> Result<web::Json<PlatformTeacherApplicationsResponse>, ApiError> {
    let query = query.into_inner().into_query(requester.user_id());
    use_case
        .list_platform_review_applications(query)
        .await
        .map(PlatformTeacherApplicationsResponse::from)
        .map(web::Json)
        .map_err(platform_review_error)
}
