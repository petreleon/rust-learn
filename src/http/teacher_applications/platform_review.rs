use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewError, TeacherApplicationPlatformReviewUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::platform_review_dto::{
    PlatformTeacherApplicationsParams, PlatformTeacherApplicationsResponse,
};

pub(super) async fn list_platform_review_applications(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationPlatformReviewUseCase>>,
    query: web::Query<PlatformTeacherApplicationsParams>,
) -> impl Responder {
    let query = query.into_inner().into_query(requester.user_id());
    match use_case.list_platform_review_applications(query).await {
        Ok(response) => {
            HttpResponse::Ok().json(PlatformTeacherApplicationsResponse::from(response))
        }
        Err(error) => platform_review_error_response(error),
    }
}

fn platform_review_error_response(error: TeacherApplicationPlatformReviewError) -> HttpResponse {
    match error {
        TeacherApplicationPlatformReviewError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        TeacherApplicationPlatformReviewError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        TeacherApplicationPlatformReviewError::Connection(message)
        | TeacherApplicationPlatformReviewError::Database(message) => {
            log::error!(
                "event=teacher_application_platform_review_api_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
    }
}
