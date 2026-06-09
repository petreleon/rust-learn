use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    chapters, contents, course_join_requests, course_roles, courses, courses_organizations,
    organizations, pending_course_organization_invites, reward_policies, user_role_course, users,
};
use crate::models::course::{
    Course, NewCourse, UpdateCourse, COURSE_STATUS_APPROVED, COURSE_STATUS_ARCHIVED,
    COURSE_STATUS_DRAFT, COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED,
    COURSE_STATUS_SUBMITTED, COURSE_STATUS_SUSPENDED,
};
use crate::models::course_join_request::{
    CourseJoinRequest, COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_PENDING,
    COURSE_JOIN_STATUS_REJECTED, COURSE_JOIN_STATUS_WAITLISTED,
};
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::pending_course_organization_invites::{
    NewPendingCourseOrganizationInvite, PendingCourseOrganizationInvite,
};
use crate::repositories::course_repository::user_permission_course_request;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::{EscapeExpressionMethods, PgTextExpressionMethods};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;

const DEFAULT_COURSE_LIMIT: i64 = 25;
const MAX_COURSE_LIMIT: i64 = 100;
const LIKE_ESCAPE_CHAR: char = '\\';

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogQuery {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogResponse {
    pub courses: Vec<LearnerCourseCatalogItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseDetailResponse {
    pub course: LearnerCourseCatalogItem,
    pub chapters: Vec<LearnerCourseCatalogChapter>,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogItem {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Vec<String>,
    pub organizations: Vec<LearnerCourseCatalogOrganization>,
    pub teachers: Vec<LearnerCourseCatalogTeacher>,
    pub content: LearnerCourseContentSummary,
    pub rewards: LearnerCourseRewardSummary,
    pub enrollment: LearnerCourseEnrollmentSummary,
    pub access: LearnerCourseAccessSummary,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogOrganization {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogTeacher {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseContentSummary {
    pub chapter_count: usize,
    pub content_count: usize,
    pub content_types: Vec<String>,
    pub has_content: bool,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseRewardSummary {
    pub available: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub token_amounts: Vec<String>,
    pub payment_strategies: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseEnrollmentSummary {
    pub state: String,
    pub request_id: Option<i64>,
    pub can_request_join: bool,
    pub reason: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseAccessSummary {
    pub can_view_course: bool,
    pub can_view_content: bool,
    pub can_view_rewards: bool,
    pub can_request_join: bool,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogChapter {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<LearnerCourseCatalogContent>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogContent {
    pub id: i32,
    pub order: i32,
    pub content_type: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LearnerCourseCatalogError {
    NotFound,
    Database(String),
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

impl LearnerCourseCatalogQuery {
    pub fn new(
        search: Option<String>,
        organization_id: Option<i32>,
        lifecycle_status: Option<String>,
        enrollment_status: Option<String>,
        reward_available: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = normalize_optional_string(search);
        let lifecycle_status = normalize_optional_string(lifecycle_status);
        let enrollment_status = normalize_optional_string(enrollment_status);
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        LearnerCourseCatalogQuery {
            search,
            organization_id,
            lifecycle_status,
            enrollment_status,
            reward_available,
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
        let pattern = course_title_search_pattern(search);
        count_query = count_query.filter(
            courses::title
                .ilike(pattern.clone())
                .escape(LIKE_ESCAPE_CHAR),
        );
        list_query = list_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(organization_id) = discovery.organization_id {
        let course_ids_for_organization = || {
            courses_organizations::table
                .filter(courses_organizations::organization_id.eq(organization_id))
                .select(courses_organizations::course_id)
        };

        count_query = count_query.filter(courses::id.eq_any(course_ids_for_organization()));
        list_query = list_query.filter(courses::id.eq_any(course_ids_for_organization()));
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

pub async fn discover_learner_course_catalog(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    catalog_query: LearnerCourseCatalogQuery,
) -> Result<LearnerCourseCatalogResponse, LearnerCourseCatalogError> {
    let mut course_query = courses::table.into_boxed();

    if let Some(search) = catalog_query.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        course_query = course_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(organization_id) = catalog_query.organization_id {
        course_query = course_query.filter(
            courses::id.eq_any(
                courses_organizations::table
                    .filter(courses_organizations::organization_id.eq(organization_id))
                    .select(courses_organizations::course_id),
            ),
        );
    }

    if let Some(status) = catalog_query.lifecycle_status.as_deref() {
        course_query = course_query.filter(courses::lifecycle_status.eq(status));
    }

    let candidate_courses = course_query
        .order(courses::id.asc())
        .load::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut items = Vec::new();
    for course in candidate_courses {
        if !course_visible_to_learner(conn, actor_user_id, &course).await? {
            continue;
        }

        let item = build_learner_course_catalog_item(conn, actor_user_id, course).await?;

        if let Some(reward_available) = catalog_query.reward_available {
            if item.rewards.available != reward_available {
                continue;
            }
        }

        if let Some(enrollment_status) = catalog_query.enrollment_status.as_deref() {
            if item.enrollment.state != enrollment_status {
                continue;
            }
        }

        items.push(item);
    }

    let total = items.len() as i64;
    let courses = items
        .into_iter()
        .skip(catalog_query.offset as usize)
        .take(catalog_query.limit as usize)
        .collect();

    Ok(LearnerCourseCatalogResponse {
        courses,
        total,
        limit: catalog_query.limit,
        offset: catalog_query.offset,
        search: catalog_query.search,
        organization_id: catalog_query.organization_id,
        lifecycle_status: catalog_query.lifecycle_status,
        enrollment_status: catalog_query.enrollment_status,
        reward_available: catalog_query.reward_available,
    })
}

pub async fn get_learner_course_detail(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<LearnerCourseDetailResponse, LearnerCourseCatalogError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    if !course_visible_to_learner(conn, actor_user_id, &course).await? {
        return Err(LearnerCourseCatalogError::NotFound);
    }

    let course = build_learner_course_catalog_item(conn, actor_user_id, course).await?;
    let chapters = load_learner_course_chapters(conn, course_id).await?;

    Ok(LearnerCourseDetailResponse {
        course,
        chapters,
        prerequisites: Vec::new(),
    })
}

fn course_title_search_pattern(search: &str) -> String {
    let mut escaped = String::with_capacity(search.len());
    for ch in search.chars() {
        match ch {
            LIKE_ESCAPE_CHAR | '%' | '_' => {
                escaped.push(LIKE_ESCAPE_CHAR);
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }

    format!("%{}%", escaped)
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

impl From<diesel::result::Error> for LearnerCourseCatalogError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => LearnerCourseCatalogError::NotFound,
            other => LearnerCourseCatalogError::Database(other.to_string()),
        }
    }
}

async fn build_learner_course_catalog_item(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course: Course,
) -> Result<LearnerCourseCatalogItem, LearnerCourseCatalogError> {
    let organizations = load_learner_course_organizations(conn, course.id).await?;
    let teachers = load_learner_course_teachers(conn, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let rewards = load_learner_course_reward_summary(conn, course.id).await?;
    let access = build_learner_course_access(conn, actor_user_id, course.id).await?;
    let enrollment = build_learner_course_enrollment(
        conn,
        actor_user_id,
        course.id,
        &course.lifecycle_status,
        access.can_request_join,
    )
    .await?;

    Ok(LearnerCourseCatalogItem {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        description: None,
        topics: Vec::new(),
        organizations,
        teachers,
        content,
        rewards,
        enrollment,
        access,
    })
}

async fn load_learner_course_organizations(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseCatalogOrganization>, LearnerCourseCatalogError> {
    let rows = courses_organizations::table
        .inner_join(
            organizations::table.on(courses_organizations::organization_id.eq(organizations::id)),
        )
        .filter(courses_organizations::course_id.eq(course_id))
        .order(courses_organizations::order.asc())
        .select((organizations::id, organizations::name))
        .load::<(i32, String)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| LearnerCourseCatalogOrganization { id, name })
        .collect())
}

async fn load_learner_course_teachers(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseCatalogTeacher>, LearnerCourseCatalogError> {
    let rows = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .inner_join(users::table.on(user_role_course::user_id.eq(users::id.nullable())))
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("TEACHER"))
        .order(users::name.asc())
        .select((users::id, users::name))
        .load::<(i32, String)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| LearnerCourseCatalogTeacher { id, name })
        .collect())
}

async fn load_learner_course_content_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<LearnerCourseContentSummary, LearnerCourseCatalogError> {
    let chapter_count = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let content_types = contents::table
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(chapters::course_id.eq(course_id))
        .order(contents::content_type.asc())
        .select(contents::content_type)
        .load::<String>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut unique_types = BTreeSet::new();
    for content_type in &content_types {
        unique_types.insert(content_type.clone());
    }

    Ok(LearnerCourseContentSummary {
        chapter_count: chapter_count as usize,
        content_count: content_types.len(),
        content_types: unique_types.into_iter().collect(),
        has_content: !content_types.is_empty(),
    })
}

async fn load_learner_course_reward_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<LearnerCourseRewardSummary, LearnerCourseCatalogError> {
    let rows = reward_policies::table
        .filter(reward_policies::course_id.eq(course_id))
        .filter(reward_policies::active.eq(true))
        .order(reward_policies::event_type.asc())
        .select((
            reward_policies::event_type,
            reward_policies::token_amount,
            reward_policies::payment_strategy,
        ))
        .load::<(String, BigDecimal, String)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut event_types = BTreeSet::new();
    let mut token_amounts = BTreeSet::new();
    let mut payment_strategies = BTreeSet::new();

    for (event_type, token_amount, payment_strategy) in &rows {
        event_types.insert(event_type.clone());
        token_amounts.insert(token_amount.to_string());
        payment_strategies.insert(payment_strategy.clone());
    }

    Ok(LearnerCourseRewardSummary {
        available: !rows.is_empty(),
        active_policy_count: rows.len(),
        event_types: event_types.into_iter().collect(),
        token_amounts: token_amounts.into_iter().collect(),
        payment_strategies: payment_strategies.into_iter().collect(),
    })
}

async fn build_learner_course_access(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<LearnerCourseAccessSummary, LearnerCourseCatalogError> {
    let can_view_course = user_has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_COURSE,
    )
    .await?;
    let can_view_content = user_has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_CONTENT,
    )
    .await?;
    let can_view_rewards = user_has_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &Permissions::VIEW_COURSE_REWARD_STATUS,
    )
    .await?;
    let can_request_join = user_has_any_permission_for_course_context(
        conn,
        actor_user_id,
        course_id,
        &[Permissions::REQUEST_JOIN_COURSE, Permissions::JOIN_COURSE],
    )
    .await?;

    Ok(LearnerCourseAccessSummary {
        can_view_course,
        can_view_content,
        can_view_rewards,
        can_request_join,
    })
}

async fn build_learner_course_enrollment(
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

async fn load_actor_course_roles(
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

async fn load_learner_course_chapters(
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

async fn course_visible_to_learner(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course: &Course,
) -> Result<bool, LearnerCourseCatalogError> {
    if course.lifecycle_status == COURSE_STATUS_PUBLISHED {
        return Ok(true);
    }

    let permission = Permissions::VIEW_COURSE.to_string();
    if user_permission_course_request(conn, actor_user_id, course.id, &permission).await? {
        return Ok(true);
    }

    for organization_id in course_organization_ids(conn, course.id).await? {
        if user_permission_organization_request(conn, actor_user_id, organization_id, &permission)
            .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn user_has_any_permission_for_course_context(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permissions: &[Permissions],
) -> Result<bool, LearnerCourseCatalogError> {
    for permission in permissions {
        if user_has_permission_for_course_context(conn, actor_user_id, course_id, permission)
            .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn user_has_permission_for_course_context(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &Permissions,
) -> Result<bool, LearnerCourseCatalogError> {
    let permission_name = permission.to_string();
    if user_permission_course_request(conn, actor_user_id, course_id, &permission_name).await? {
        return Ok(true);
    }

    if user_permission_platform_request(conn, actor_user_id, &permission_name).await? {
        return Ok(true);
    }

    for organization_id in course_organization_ids(conn, course_id).await? {
        if user_permission_organization_request(
            conn,
            actor_user_id,
            organization_id,
            &permission_name,
        )
        .await?
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
        .map_err(LearnerCourseCatalogError::from)
}

pub async fn create_course_with_invites(
    conn: &mut AsyncPgConnection,
    title: String,
    organization_ids: Vec<i32>,
) -> QueryResult<Course> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let new_course = NewCourse { title };

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
