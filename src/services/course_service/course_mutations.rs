use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, courses_organizations};
use crate::models::course::{Course, NewCourse, UpdateCourse};
use crate::models::courses_organizations::NewCourseOrganization;
use crate::repositories::course_repository::user_permission_course_request;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use super::course_lifecycle_invites::create_course_organization_invite;
use super::errors::{CourseCreationError, CourseUpdateError};

pub async fn create_course_with_invites(
    conn: &mut AsyncPgConnection,
    title: String,
    organization_ids: Vec<i32>,
) -> QueryResult<Course> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let new_course = NewCourse {
                title,
                description: None,
                topics: None,
                prerequisites: None,
            };

            let course = diesel::insert_into(courses::table)
                .values(&new_course)
                .get_result::<Course>(conn)
                .await?;

            if let Some(first_org_id) = organization_ids.as_slice().first() {
                // Add first organization directly
                let new_link = NewCourseOrganization {
                    course_id: course.id,
                    organization_id: *first_org_id,
                    order: 0,
                };
                diesel::insert_into(courses_organizations::table)
                    .values(&new_link)
                    .execute(conn)
                    .await?;

                // Add remaining organizations as pending invites
                for org_id in organization_ids.iter().skip(1) {
                    create_course_organization_invite(conn, course.id, *org_id).await?;
                }
            }

            Ok(course)
        })
    })
    .await
}

pub async fn create_course_with_invites_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    title: String,
    organization_ids: Vec<i32>,
) -> Result<Course, CourseCreationError> {
    ensure_course_creation_permission(conn, actor_user_id, organization_ids.as_slice()).await?;
    create_course_with_invites(conn, title, organization_ids)
        .await
        .map_err(CourseCreationError::from)
}

pub async fn update_course_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    update: UpdateCourse,
) -> Result<Course, CourseUpdateError> {
    ensure_course_update_permission(conn, actor_user_id, course_id).await?;

    diesel::update(courses::table.find(course_id))
        .set(&update)
        .get_result::<Course>(conn)
        .await
        .map_err(CourseUpdateError::from)
}

async fn ensure_course_update_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseUpdateError> {
    let course_permission = Permissions::MANAGE_COURSE_SETTINGS.to_string();
    match user_permission_course_request(conn, user_id, course_id, &course_permission).await {
        Ok(true) => return Ok(()),
        Ok(false) => {}
        Err(error) => return Err(CourseUpdateError::from(error)),
    }

    let platform_permission = Permissions::MODIFY_COURSE.to_string();
    match user_permission_platform_request(conn, user_id, &platform_permission).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(CourseUpdateError::PermissionDenied(course_permission)),
        Err(error) => Err(CourseUpdateError::from(error)),
    }
}

async fn ensure_course_creation_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_ids: &[i32],
) -> Result<(), CourseCreationError> {
    let platform_permission = Permissions::CREATE_COURSE.to_string();
    match user_permission_platform_request(conn, user_id, &platform_permission).await {
        Ok(true) => return Ok(()),
        Ok(false) => {}
        Err(error) => return Err(CourseCreationError::from(error)),
    }

    if let Some(owner_organization_id) = organization_ids.first() {
        match user_permission_organization_request(
            conn,
            user_id,
            *owner_organization_id,
            &platform_permission,
        )
        .await
        {
            Ok(true) => return Ok(()),
            Ok(false) => {}
            Err(error) => return Err(CourseCreationError::from(error)),
        }
    }

    Err(CourseCreationError::PermissionDenied(platform_permission))
}
