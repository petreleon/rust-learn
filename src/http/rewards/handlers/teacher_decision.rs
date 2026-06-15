use std::sync::Arc;

use actix_web::web;

use crate::application::rewards::decide_teacher_candidate::TeacherRewardCandidateDecisionUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::{
    TeacherRewardCandidateDecisionRequest, TeacherRewardCandidateDecisionResponse,
};
use crate::http::rewards::errors::teacher_decision_error;

pub async fn decide_reward_candidate_by_teacher(
    requester: AuthUser,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn TeacherRewardCandidateDecisionUseCase>>,
    body: web::Json<TeacherRewardCandidateDecisionRequest>,
) -> Result<web::Json<TeacherRewardCandidateDecisionResponse>, ApiError> {
    let (course_id, candidate_id) = path.into_inner();

    use_case
        .decide_teacher_reward_candidate(
            requester.user_id(),
            course_id,
            candidate_id,
            body.into_inner().into(),
        )
        .await
        .map(TeacherRewardCandidateDecisionResponse::from)
        .map(web::Json)
        .map_err(teacher_decision_error)
}
