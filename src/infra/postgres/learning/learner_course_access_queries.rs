use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::learner_course_catalog::{
    LearnerCourseAccessSummaryOutput, LearnerCourseCatalogError,
};
use crate::config::constants::permissions::Permissions;
use crate::domain::learning::course::status::COURSE_STATUS_PUBLISHED;
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::schema::courses_organizations;

pub async fn course_visible_to_learner(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course: &Course,
) -> Result<bool, LearnerCourseCatalogError> {
    if course.lifecycle_status == COURSE_STATUS_PUBLISHED {
        return Ok(true);
    }

    let permission = Permissions::VIEW_COURSE.to_string();
    if permission_checks::can(
        conn,
        AccessActor::user(actor_user_id),
        AccessAction::permission(permission.clone()),
        AccessScope::course(course.id),
    )
    .await
    .map_err(map_learning_error)?
    {
        return Ok(true);
    }

    for organization_id in course_organization_ids(conn, course.id).await? {
        if permission_checks::can(
            conn,
            AccessActor::user(actor_user_id),
            AccessAction::permission(permission.clone()),
            AccessScope::organization(organization_id),
        )
        .await
        .map_err(map_learning_error)?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

pub async fn build_learner_course_access(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<LearnerCourseAccessSummaryOutput, LearnerCourseCatalogError> {
    let can_view_course = has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_COURSE,
    )
    .await?;
    let can_view_content = has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_CONTENT,
    )
    .await?;
    let can_view_rewards = has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_COURSE_REWARD_STATUS,
    )
    .await?;
    let can_request_join = has_any_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &[Permissions::REQUEST_JOIN_COURSE, Permissions::JOIN_COURSE],
    )
    .await?;

    Ok(LearnerCourseAccessSummaryOutput {
        can_view_course,
        can_view_content,
        can_view_rewards,
        can_request_join,
    })
}

async fn has_any_permission_for_course_context(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permissions: &[Permissions],
) -> Result<bool, LearnerCourseCatalogError> {
    for permission in permissions {
        if has_permission_for_course_context(conn, actor_user_id, course_id, permission).await? {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn has_permission_for_course_context(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &Permissions,
) -> Result<bool, LearnerCourseCatalogError> {
    let permission_name = permission.to_string();
    if permission_checks::can(
        conn,
        AccessActor::user(actor_user_id),
        AccessAction::permission(permission_name.clone()),
        AccessScope::course(course_id),
    )
    .await
    .map_err(map_learning_error)?
    {
        return Ok(true);
    }

    if permission_checks::can(
        conn,
        AccessActor::user(actor_user_id),
        AccessAction::permission(permission_name.clone()),
        AccessScope::platform(),
    )
    .await
    .map_err(map_learning_error)?
    {
        return Ok(true);
    }

    for organization_id in course_organization_ids(conn, course_id).await? {
        if permission_checks::can(
            conn,
            AccessActor::user(actor_user_id),
            AccessAction::permission(permission_name.clone()),
            AccessScope::organization(organization_id),
        )
        .await
        .map_err(map_learning_error)?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn course_organization_ids(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<i32>, LearnerCourseCatalogError> {
    courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await
        .map_err(map_learning_error)
}

fn map_learning_error(error: diesel::result::Error) -> LearnerCourseCatalogError {
    match error {
        diesel::result::Error::NotFound => LearnerCourseCatalogError::NotFound,
        other => LearnerCourseCatalogError::Database(other.to_string()),
    }
}
