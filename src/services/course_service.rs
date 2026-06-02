use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, courses_organizations, pending_course_organization_invites};
use crate::models::course::{
    Course, NewCourse, UpdateCourse, COURSE_STATUS_APPROVED, COURSE_STATUS_ARCHIVED,
    COURSE_STATUS_DRAFT, COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED,
    COURSE_STATUS_SUBMITTED, COURSE_STATUS_SUSPENDED,
};
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::pending_course_organization_invites::{
    NewPendingCourseOrganizationInvite, PendingCourseOrganizationInvite,
};
use crate::repositories::course_repository::user_permission_course_request;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use diesel::prelude::*;
use diesel::PgTextExpressionMethods;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::Deserialize;
use serde::Serialize;

const DEFAULT_COURSE_LIMIT: i64 = 25;
const MAX_COURSE_LIMIT: i64 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseDiscoveryQuery {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize)]
pub struct CourseDiscoveryResponse {
    pub courses: Vec<Course>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CourseLifecycleUpdateRequest {
    pub status: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CourseLifecycleError {
    PermissionDenied(String),
    InvalidStatus(String),
    NotFound,
    Database(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CourseCreationError {
    PermissionDenied(String),
    Database(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CourseUpdateError {
    PermissionDenied(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for CourseUpdateError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => CourseUpdateError::NotFound,
            other => CourseUpdateError::Database(other.to_string()),
        }
    }
}

impl From<diesel::result::Error> for CourseCreationError {
    fn from(error: diesel::result::Error) -> Self {
        CourseCreationError::Database(error.to_string())
    }
}

impl From<diesel::result::Error> for CourseLifecycleError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => CourseLifecycleError::NotFound,
            other => CourseLifecycleError::Database(other.to_string()),
        }
    }
}

impl CourseDiscoveryQuery {
    pub fn new(
        search: Option<String>,
        organization_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = search
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        CourseDiscoveryQuery {
            search,
            organization_id,
            limit,
            offset,
        }
    }
}

pub async fn discover_courses(
    conn: &mut AsyncPgConnection,
    discovery: CourseDiscoveryQuery,
) -> QueryResult<CourseDiscoveryResponse> {
    let mut count_query = courses::table.into_boxed();
    let mut list_query = courses::table.into_boxed();

    if let Some(search) = discovery.search.as_deref() {
        let pattern = format!("%{}%", search);
        count_query = count_query.filter(courses::title.ilike(pattern.clone()));
        list_query = list_query.filter(courses::title.ilike(pattern));
    }

    if let Some(organization_id) = discovery.organization_id {
        let course_ids = courses_organizations::table
            .filter(courses_organizations::organization_id.eq(organization_id))
            .select(courses_organizations::course_id);
        count_query = count_query.filter(courses::id.eq_any(course_ids));

        let course_ids = courses_organizations::table
            .filter(courses_organizations::organization_id.eq(organization_id))
            .select(courses_organizations::course_id);
        list_query = list_query.filter(courses::id.eq_any(course_ids));
    }

    let total = count_query.count().get_result(conn).await?;
    let courses = list_query
        .order(courses::id.asc())
        .limit(discovery.limit)
        .offset(discovery.offset)
        .load::<Course>(conn)
        .await?;

    Ok(CourseDiscoveryResponse {
        courses,
        total,
        limit: discovery.limit,
        offset: discovery.offset,
        search: discovery.search,
        organization_id: discovery.organization_id,
    })
}

pub async fn create_course_with_invites(
    conn: &mut AsyncPgConnection,
    title: String,
    organization_ids: Vec<i32>,
) -> QueryResult<Course> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let new_course = NewCourse { title: title };

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

pub async fn update_course_lifecycle(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    request: CourseLifecycleUpdateRequest,
) -> Result<Course, CourseLifecycleError> {
    let target_status = normalize_course_status(&request.status)?;
    ensure_lifecycle_permission(conn, actor_user_id, course_id, &target_status).await?;

    diesel::update(courses::table.find(course_id))
        .set(courses::lifecycle_status.eq(target_status))
        .get_result::<Course>(conn)
        .await
        .map_err(CourseLifecycleError::from)
}

async fn ensure_lifecycle_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    target_status: &str,
) -> Result<(), CourseLifecycleError> {
    let required_course_permission = match target_status {
        COURSE_STATUS_DRAFT
        | COURSE_STATUS_SUBMITTED
        | COURSE_STATUS_ARCHIVED
        | COURSE_STATUS_SUSPENDED => Permissions::MANAGE_COURSE_SETTINGS,
        COURSE_STATUS_NEEDS_CHANGES | COURSE_STATUS_APPROVED => Permissions::APPROVE_COURSE_CONTENT,
        COURSE_STATUS_PUBLISHED => Permissions::PUBLISH_CONTENT,
        _ => {
            return Err(CourseLifecycleError::InvalidStatus(
                "unsupported course lifecycle status".to_string(),
            ))
        }
    };

    let course_permission_name = required_course_permission.to_string();
    if user_permission_course_request(conn, user_id, course_id, &course_permission_name).await? {
        return Ok(());
    }

    let platform_permission_name = Permissions::MODIFY_COURSE.to_string();
    if user_permission_platform_request(conn, user_id, &platform_permission_name).await? {
        return Ok(());
    }

    Err(CourseLifecycleError::PermissionDenied(
        course_permission_name,
    ))
}

fn normalize_course_status(status: &str) -> Result<String, CourseLifecycleError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        COURSE_STATUS_DRAFT
        | COURSE_STATUS_SUBMITTED
        | COURSE_STATUS_NEEDS_CHANGES
        | COURSE_STATUS_APPROVED
        | COURSE_STATUS_PUBLISHED
        | COURSE_STATUS_ARCHIVED
        | COURSE_STATUS_SUSPENDED => Ok(normalized),
        _ => Err(CourseLifecycleError::InvalidStatus(
            "unsupported course lifecycle status".to_string(),
        )),
    }
}

pub async fn create_course_organization_invite(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) -> QueryResult<usize> {
    let max_order_active: Option<i32> = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(diesel::dsl::max(courses_organizations::order))
        .first(conn)
        .await
        .optional()?
        .flatten();

    let max_order_pending: Option<i32> = pending_course_organization_invites::table
        .filter(pending_course_organization_invites::course_id.eq(course_id))
        .select(diesel::dsl::max(pending_course_organization_invites::order))
        .first(conn)
        .await
        .optional()?
        .flatten();

    let next_order = match (max_order_active, max_order_pending) {
        (Some(a), Some(b)) => std::cmp::max(a, b) + 1,
        (Some(a), None) => a + 1,
        (None, Some(b)) => b + 1,
        (None, None) => 0,
    };

    let new_invite = NewPendingCourseOrganizationInvite {
        course_id,
        organization_id,
        order: next_order,
    };
    diesel::insert_into(pending_course_organization_invites::table)
        .values(&new_invite)
        .execute(conn)
        .await
}

pub async fn accept_course_organization_invite(
    conn: &mut AsyncPgConnection,
    invite_id: i32,
) -> QueryResult<usize> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let invite = pending_course_organization_invites::table
                .find(invite_id)
                .first::<PendingCourseOrganizationInvite>(conn)
                .await?;

            let new_link = NewCourseOrganization {
                course_id: invite.course_id,
                organization_id: invite.organization_id,
                order: invite.order,
            };

            diesel::insert_into(courses_organizations::table)
                .values(&new_link)
                .execute(conn)
                .await?;

            diesel::delete(pending_course_organization_invites::table.find(invite_id))
                .execute(conn)
                .await
        })
    })
    .await
}
