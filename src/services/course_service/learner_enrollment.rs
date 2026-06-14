use crate::db::schema::{chapters, contents, course_join_requests, course_roles, user_role_course};
use crate::domain::learning::course::status::COURSE_STATUS_PUBLISHED;
use crate::domain::learning::enrollment::status::{
    COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_REJECTED,
    COURSE_JOIN_STATUS_WAITLISTED,
};
use crate::models::course_join_request::CourseJoinRequest;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::errors::LearnerCourseCatalogError;
use super::learner_course_types::{
    LearnerCourseCatalogChapter, LearnerCourseCatalogContent, LearnerCourseEnrollmentSummary,
};

pub(super) async fn build_learner_course_enrollment(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    lifecycle_status: &str,
    can_request_join: bool,
) -> Result<LearnerCourseEnrollmentSummary, LearnerCourseCatalogError> {
    let roles = load_actor_course_roles(conn, actor_user_id, course_id).await?;
    let latest_request = course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(actor_user_id))
        .order(course_join_requests::updated_at.desc())
        .first::<CourseJoinRequest>(conn)
        .await
        .optional()
        .map_err(LearnerCourseCatalogError::from)?;

    let is_student = roles.iter().any(|role| role == "STUDENT");
    if is_student {
        return Ok(LearnerCourseEnrollmentSummary {
            state: "enrolled".to_string(),
            request_id: latest_request.as_ref().map(|request| request.id),
            can_request_join: false,
            reason: Some("You already have course access.".to_string()),
            roles,
        });
    }

    if let Some(request) = latest_request {
        match request.status.as_str() {
            COURSE_JOIN_STATUS_PENDING | COURSE_JOIN_STATUS_WAITLISTED => {
                return Ok(LearnerCourseEnrollmentSummary {
                    state: request.status,
                    request_id: Some(request.id),
                    can_request_join: false,
                    reason: Some("Your join request is waiting for review.".to_string()),
                    roles,
                });
            }
            COURSE_JOIN_STATUS_REJECTED => {
                return Ok(LearnerCourseEnrollmentSummary {
                    state: COURSE_JOIN_STATUS_REJECTED.to_string(),
                    request_id: Some(request.id),
                    can_request_join,
                    reason: Some(
                        "Your previous join request was rejected; you can request again if access is open."
                            .to_string(),
                    ),
                    roles,
                });
            }
            COURSE_JOIN_STATUS_APPROVED => {
                return Ok(LearnerCourseEnrollmentSummary {
                    state: "enrolled".to_string(),
                    request_id: Some(request.id),
                    can_request_join: false,
                    reason: Some("Your join request was approved.".to_string()),
                    roles,
                });
            }
            _ => {}
        }
    }

    if lifecycle_status != COURSE_STATUS_PUBLISHED {
        return Ok(LearnerCourseEnrollmentSummary {
            state: "unavailable".to_string(),
            request_id: None,
            can_request_join: false,
            reason: Some("This course is not published for learner enrollment.".to_string()),
            roles,
        });
    }

    Ok(LearnerCourseEnrollmentSummary {
        state: "available".to_string(),
        request_id: None,
        can_request_join,
        reason: if can_request_join {
            Some("Enrollment can be requested.".to_string())
        } else {
            Some("Your account cannot request course enrollment yet.".to_string())
        },
        roles,
    })
}

pub(super) async fn load_actor_course_roles(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<Vec<String>, LearnerCourseCatalogError> {
    let mut roles = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .filter(user_role_course::user_id.eq(actor_user_id))
        .filter(user_role_course::course_id.eq(course_id))
        .order(course_roles::name.asc())
        .select(course_roles::name)
        .load::<String>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    roles.dedup();
    Ok(roles)
}

pub(super) async fn load_learner_course_chapters(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseCatalogChapter>, LearnerCourseCatalogError> {
    let chapter_rows = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .order(chapters::order.asc())
        .select((chapters::id, chapters::title, chapters::order))
        .load::<(i32, String, i32)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut result = Vec::with_capacity(chapter_rows.len());
    for (id, title, order) in chapter_rows {
        let content_rows = contents::table
            .filter(contents::chapter_id.eq(id))
            .order(contents::order.asc())
            .select((contents::id, contents::order, contents::content_type))
            .load::<(i32, i32, String)>(conn)
            .await
            .map_err(LearnerCourseCatalogError::from)?;

        result.push(LearnerCourseCatalogChapter {
            id,
            title,
            order,
            contents: content_rows
                .into_iter()
                .map(|(id, order, content_type)| LearnerCourseCatalogContent {
                    id,
                    order,
                    content_type,
                })
                .collect(),
        });
    }

    Ok(result)
}
