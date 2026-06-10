use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    chapters, contents, course_join_requests, course_progress, course_roles, courses,
    courses_organizations, delegated_permissions, organizations, pending_course_organization_invites,
    reward_candidates, reward_policies, role_permission_course, role_permission_organization,
    role_permission_platform, upload_jobs, user_role_course, user_role_organization,
    user_role_platform, users,
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
use crate::models::course_progress::{CourseProgress, NewCourseProgress};
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::delegated_permission::{
    DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM,
};
use crate::models::pending_course_organization_invites::{
    NewPendingCourseOrganizationInvite, PendingCourseOrganizationInvite,
};
use crate::models::reward_candidate::{
    REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TEACHER_APPROVED, REWARD_STATUS_TEACHER_REJECTED,
};
use crate::repositories::course_repository::user_permission_course_request;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::BoolExpressionMethods;
use diesel::{EscapeExpressionMethods, PgTextExpressionMethods};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
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
pub struct LearnerCourseLearningResponse {
    pub course: LearnerCourseCatalogItem,
    pub chapters: Vec<LearnerCourseLearningChapter>,
    pub active_content_id: Option<i32>,
    pub progress_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseDashboardQuery {
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseListQuery {
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub reward_available: Option<bool>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseEnrollmentQuery {
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseDashboardResponse {
    pub courses: Vec<TeacherCourseDashboardItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationCourseListResponse {
    pub organization: LearnerCourseCatalogOrganization,
    pub courses: Vec<OrganizationCourseListItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub reward_available: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationCourseListItem {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub teachers: Vec<LearnerCourseCatalogTeacher>,
    pub content: LearnerCourseContentSummary,
    pub rewards: LearnerCourseRewardSummary,
    pub roster: TeacherCourseRosterSummary,
    pub reward_queue: TeacherCourseRewardQueueSummary,
    pub permissions: OrganizationCoursePermissionSummary,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseDashboardItem {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub organizations: Vec<LearnerCourseCatalogOrganization>,
    pub content: LearnerCourseContentSummary,
    pub rewards: LearnerCourseRewardSummary,
    pub roster: TeacherCourseRosterSummary,
    pub reward_queue: TeacherCourseRewardQueueSummary,
    pub permissions: TeacherCoursePermissionSummary,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseWorkspaceResponse {
    pub course: TeacherCourseDashboardItem,
    pub teacher_roles: Vec<String>,
    pub publication: TeacherCoursePublicationSummary,
    pub chapters: Vec<TeacherCourseWorkspaceChapter>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseEnrollmentWorkspaceResponse {
    pub course: TeacherCourseDashboardItem,
    pub teacher_roles: Vec<String>,
    pub join_requests: TeacherCourseJoinRequestPage,
    pub roster: TeacherCourseRosterPage,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseStudentsResponse {
    pub course: TeacherCourseDashboardItem,
    pub teacher_roles: Vec<String>,
    pub students: Vec<TeacherCourseStudentProgressItem>,
    pub total: i64,
    pub progress_supported: bool,
    pub reward_evidence_supported: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseStudentProgressItem {
    pub user: TeacherEnrollmentUserSummary,
    pub roles: Vec<String>,
    pub access_state: String,
    pub latest_join_request_status: Option<String>,
    pub progress: TeacherStudentProgressSummary,
    pub rewards: TeacherStudentRewardProgressSummary,
}

#[derive(Debug, Serialize)]
pub struct TeacherStudentProgressSummary {
    pub supported: bool,
    pub completed_content_count: Option<i64>,
    pub total_content_count: usize,
    pub completion_percentage: Option<f64>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub note: String,
}

#[derive(Debug, Serialize)]
pub struct TeacherStudentRewardProgressSummary {
    pub reward_candidate_count: i64,
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub teacher_rejected_count: i64,
    pub completed_count: i64,
    pub failed_count: i64,
    pub latest_candidate: Option<TeacherStudentRewardCandidateSummary>,
}

#[derive(Debug, Serialize)]
pub struct TeacherStudentRewardCandidateSummary {
    pub id: i64,
    pub event_type: String,
    pub status: String,
    pub evidence: Value,
    pub teacher_decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseJoinRequestPage {
    pub requests: Vec<TeacherCourseJoinRequestItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseJoinRequestItem {
    pub id: i64,
    pub status: String,
    pub requester: TeacherEnrollmentUserSummary,
    pub reviewer: Option<TeacherEnrollmentUserSummary>,
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
    pub can_decide: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseRosterPage {
    pub learners: Vec<TeacherCourseRosterLearner>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseRosterLearner {
    pub user: TeacherEnrollmentUserSummary,
    pub roles: Vec<String>,
    pub latest_join_request_status: Option<String>,
    pub access_state: String,
    pub can_remove: bool,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherEnrollmentUserSummary {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCoursePublicationSummary {
    pub course_lifecycle_status: String,
    pub content_publication_status_supported: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseWorkspaceChapter {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<TeacherCourseWorkspaceContent>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseWorkspaceContent {
    pub id: i32,
    pub order: i32,
    pub content_type: String,
    pub data_present: bool,
    pub publication_status: String,
    pub display_state: String,
    pub processing_status: Option<String>,
    pub processing_error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseRosterSummary {
    pub enrolled_student_count: i64,
    pub pending_join_request_count: i64,
    pub waitlisted_join_request_count: i64,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseRewardQueueSummary {
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub failed_count: i64,
}

#[derive(Debug, Serialize)]
pub struct TeacherCoursePermissionSummary {
    pub can_manage_settings: bool,
    pub can_manage_content: bool,
    pub can_manage_enrollments: bool,
    pub can_view_reward_candidates: bool,
    pub can_approve_reward_candidates: bool,
    pub can_manage_reward_rules: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationCoursePermissionSummary {
    pub can_view_courses: bool,
    pub can_create_courses: bool,
    pub can_manage_course_settings: bool,
    pub can_manage_enrollments: bool,
    pub can_submit_reward_events: bool,
    pub can_view_reward_reports: bool,
    pub can_manage_reward_budget: bool,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogItem {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Vec<String>,
    pub prerequisites: Vec<String>,
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

enum TeacherCourseCandidateScope {
    All,
    CourseIds(Vec<i32>),
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

#[derive(Debug, Serialize)]
pub struct LearnerCourseLearningChapter {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<LearnerCourseLearningContent>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseLearningContent {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
    pub display_state: String,
    pub processing_status: Option<String>,
    pub processing_error: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LearnerCourseCatalogError {
    PermissionDenied(String),
    NotFound,
    Database(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum TeacherCourseDashboardError {
    PermissionDenied(String),
    NotFound,
    Database(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrganizationCourseListError {
    PermissionDenied(String),
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

impl TeacherCourseDashboardQuery {
    pub fn new(
        search: Option<String>,
        lifecycle_status: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = normalize_optional_string(search);
        let lifecycle_status = normalize_optional_string(lifecycle_status);
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        TeacherCourseDashboardQuery {
            search,
            lifecycle_status,
            limit,
            offset,
        }
    }
}

impl OrganizationCourseListQuery {
    pub fn new(
        search: Option<String>,
        lifecycle_status: Option<String>,
        reward_available: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = normalize_optional_string(search);
        let lifecycle_status = normalize_optional_string(lifecycle_status);
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        OrganizationCourseListQuery {
            search,
            lifecycle_status,
            reward_available,
            limit,
            offset,
        }
    }
}

impl TeacherCourseEnrollmentQuery {
    pub fn new(status: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Self {
        let status = normalize_optional_string(status)
            .map(|value| value.to_ascii_lowercase())
            .or_else(|| Some("open".to_string()));
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        TeacherCourseEnrollmentQuery {
            status,
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

pub async fn discover_teacher_course_dashboard(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    dashboard_query: TeacherCourseDashboardQuery,
) -> Result<TeacherCourseDashboardResponse, TeacherCourseDashboardError> {
    let candidate_scope = teacher_course_candidate_scope(conn, actor_user_id).await?;
    if matches!(
        candidate_scope,
        TeacherCourseCandidateScope::CourseIds(ref ids) if ids.is_empty()
    ) {
        return Ok(TeacherCourseDashboardResponse {
            courses: Vec::new(),
            total: 0,
            limit: dashboard_query.limit,
            offset: dashboard_query.offset,
            search: dashboard_query.search,
            lifecycle_status: dashboard_query.lifecycle_status,
        });
    }

    let mut count_query = courses::table.into_boxed();
    let mut list_query = courses::table.into_boxed();

    match &candidate_scope {
        TeacherCourseCandidateScope::All => {}
        TeacherCourseCandidateScope::CourseIds(course_ids) => {
            count_query = count_query.filter(courses::id.eq_any(course_ids));
            list_query = list_query.filter(courses::id.eq_any(course_ids));
        }
    }

    if let Some(search) = dashboard_query.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        count_query = count_query.filter(
            courses::title
                .ilike(pattern.clone())
                .escape(LIKE_ESCAPE_CHAR),
        );
        list_query = list_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(status) = dashboard_query.lifecycle_status.as_deref() {
        count_query = count_query.filter(courses::lifecycle_status.eq(status));
        list_query = list_query.filter(courses::lifecycle_status.eq(status));
    }

    let total = count_query
        .count()
        .get_result(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;
    let candidate_courses = list_query
        .order(courses::id.asc())
        .limit(dashboard_query.limit)
        .offset(dashboard_query.offset)
        .load::<Course>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;

    let mut items = Vec::new();
    for course in candidate_courses {
        let permissions = build_teacher_course_permissions(conn, actor_user_id, course.id).await?;
        if !permissions.has_teacher_access() {
            continue;
        }

        items.push(build_teacher_course_dashboard_item(conn, course, permissions).await?);
    }

    Ok(TeacherCourseDashboardResponse {
        courses: items,
        total,
        limit: dashboard_query.limit,
        offset: dashboard_query.offset,
        search: dashboard_query.search,
        lifecycle_status: dashboard_query.lifecycle_status,
    })
}

pub async fn discover_organization_courses(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    list_query: OrganizationCourseListQuery,
) -> Result<OrganizationCourseListResponse, OrganizationCourseListError> {
    if !user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?
    {
        return Err(OrganizationCourseListError::PermissionDenied(
            Permissions::VIEW_ORGANIZATION.to_string(),
        ));
    }

    let (organization_id, organization_name) = organizations::table
        .find(organization_id)
        .select((organizations::id, organizations::name))
        .first::<(i32, String)>(conn)
        .await
        .map_err(OrganizationCourseListError::from)?;

    let mut course_query = courses_organizations::table
        .inner_join(courses::table.on(courses_organizations::course_id.eq(courses::id)))
        .filter(courses_organizations::organization_id.eq(organization_id))
        .into_boxed();

    if let Some(search) = list_query.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        course_query = course_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(status) = list_query.lifecycle_status.as_deref() {
        course_query = course_query.filter(courses::lifecycle_status.eq(status));
    }

    let candidate_courses = course_query
        .order(courses_organizations::order.asc())
        .then_order_by(courses::id.asc())
        .select(Course::as_select())
        .load::<Course>(conn)
        .await
        .map_err(OrganizationCourseListError::from)?;

    let permissions =
        build_organization_course_permissions(conn, actor_user_id, organization_id).await?;
    let mut items = Vec::new();
    for course in candidate_courses {
        let item = build_organization_course_list_item(conn, course, permissions.clone()).await?;
        if let Some(reward_available) = list_query.reward_available {
            if item.rewards.available != reward_available {
                continue;
            }
        }
        items.push(item);
    }

    let total = items.len() as i64;
    let courses = items
        .into_iter()
        .skip(list_query.offset as usize)
        .take(list_query.limit as usize)
        .collect();

    Ok(OrganizationCourseListResponse {
        organization: LearnerCourseCatalogOrganization {
            id: organization_id,
            name: organization_name,
        },
        courses,
        total,
        limit: list_query.limit,
        offset: list_query.offset,
        search: list_query.search,
        lifecycle_status: list_query.lifecycle_status,
        reward_available: list_query.reward_available,
    })
}

pub async fn get_teacher_course_workspace(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<TeacherCourseWorkspaceResponse, TeacherCourseDashboardError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;
    let permissions = build_teacher_course_permissions(conn, actor_user_id, course.id).await?;
    if !permissions.has_teacher_access() {
        return Err(TeacherCourseDashboardError::PermissionDenied(
            "teaching course access".to_string(),
        ));
    }

    let teacher_roles = load_actor_course_roles(conn, actor_user_id, course.id).await?;
    let publication = TeacherCoursePublicationSummary {
        course_lifecycle_status: course.lifecycle_status.clone(),
        content_publication_status_supported: false,
    };
    let chapters =
        load_teacher_course_workspace_chapters(conn, course.id, &course.lifecycle_status).await?;
    let course = build_teacher_course_dashboard_item(conn, course, permissions).await?;

    Ok(TeacherCourseWorkspaceResponse {
        course,
        teacher_roles,
        publication,
        chapters,
    })
}

pub async fn get_teacher_course_enrollment_workspace(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    enrollment_query: TeacherCourseEnrollmentQuery,
) -> Result<TeacherCourseEnrollmentWorkspaceResponse, TeacherCourseDashboardError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;
    let permissions = build_teacher_course_permissions(conn, actor_user_id, course.id).await?;
    if !permissions.can_manage_enrollments {
        return Err(TeacherCourseDashboardError::PermissionDenied(
            "course enrollment management".to_string(),
        ));
    }

    let can_manage_enrollments = permissions.can_manage_enrollments;
    let teacher_roles = load_actor_course_roles(conn, actor_user_id, course.id).await?;
    let join_requests = load_teacher_course_join_request_page(
        conn,
        course.id,
        &enrollment_query,
        can_manage_enrollments,
    )
    .await?;
    let roster = load_teacher_course_roster_page(conn, course.id, can_manage_enrollments).await?;
    let course = build_teacher_course_dashboard_item(conn, course, permissions).await?;

    Ok(TeacherCourseEnrollmentWorkspaceResponse {
        course,
        teacher_roles,
        join_requests,
        roster,
        progress_supported: false,
        reward_eligibility_supported: false,
    })
}

pub async fn get_teacher_course_students(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<TeacherCourseStudentsResponse, TeacherCourseDashboardError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;
    let permissions = build_teacher_course_permissions(conn, actor_user_id, course.id).await?;
    let can_view_students = permissions.can_manage_enrollments
        || permissions.can_view_reward_candidates
        || permissions.can_approve_reward_candidates;
    if !can_view_students {
        return Err(TeacherCourseDashboardError::PermissionDenied(
            "course student progress".to_string(),
        ));
    }

    let teacher_roles = load_actor_course_roles(conn, actor_user_id, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let roster =
        load_teacher_course_roster_page(conn, course.id, permissions.can_manage_enrollments)
            .await?;
    let mut students = Vec::with_capacity(roster.learners.len());
    for learner in roster.learners {
        let rewards =
            load_teacher_student_reward_progress(conn, course.id, learner.user.id).await?;
        students.push(TeacherCourseStudentProgressItem {
            user: learner.user,
            roles: learner.roles,
            access_state: learner.access_state,
            latest_join_request_status: learner.latest_join_request_status,
            progress: TeacherStudentProgressSummary {
                supported: false,
                completed_content_count: None,
                total_content_count: content.content_count,
                completion_percentage: None,
                last_activity_at: None,
                note: "Persisted lesson progress is not tracked yet.".to_string(),
            },
            rewards,
        });
    }

    let total = students.len() as i64;
    let course = build_teacher_course_dashboard_item(conn, course, permissions).await?;

    Ok(TeacherCourseStudentsResponse {
        course,
        teacher_roles,
        students,
        total,
        progress_supported: false,
        reward_evidence_supported: true,
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
    let prerequisites = course.prerequisites.clone();

    Ok(LearnerCourseDetailResponse {
        course,
        chapters,
        prerequisites,
    })
}

pub async fn get_learner_course_learning(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<LearnerCourseLearningResponse, LearnerCourseCatalogError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    if !course_visible_to_learner(conn, actor_user_id, &course).await? {
        return Err(LearnerCourseCatalogError::NotFound);
    }

    let content_permission = Permissions::VIEW_CONTENT;
    if !user_has_permission_for_course_context(conn, actor_user_id, course_id, &content_permission)
        .await?
    {
        return Err(LearnerCourseCatalogError::PermissionDenied(
            content_permission.to_string(),
        ));
    }

    let course = build_learner_course_catalog_item(conn, actor_user_id, course).await?;
    let chapters = load_learner_course_learning_chapters(conn, course_id).await?;
    let active_content_id = chapters
        .iter()
        .flat_map(|chapter| {
            chapter
                .contents
                .iter()
                .map(move |content| (chapter.order, chapter.id, content.order, content.id))
        })
        .min_by_key(|(chapter_order, chapter_id, content_order, content_id)| {
            (*chapter_order, *chapter_id, *content_order, *content_id)
        })
        .map(|(_, _, _, content_id)| content_id);

    Ok(LearnerCourseLearningResponse {
        course,
        chapters,
        active_content_id,
        progress_supported: false,
    })
}

pub async fn save_learner_progress(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    content_id: i32,
) -> Result<CourseProgress, LearnerCourseCatalogError> {
    let _course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let _content = contents::table
        .find(content_id)
        .first::<crate::models::content::Content>(conn)
        .await
        .map_err(|_| LearnerCourseCatalogError::NotFound)?;

    diesel::insert_into(course_progress::table)
        .values(NewCourseProgress {
            user_id,
            course_id,
            content_id,
        })
        .on_conflict((course_progress::user_id, course_progress::course_id))
        .do_update()
        .set((
            course_progress::content_id.eq(content_id),
            course_progress::viewed_at.eq(diesel::dsl::now),
        ))
        .get_result::<CourseProgress>(conn)
        .await
        .map_err(|e| {
            log::error!(
                "event=learner_progress_save_failed user_id={} course_id={} content_id={} error={}",
                user_id,
                course_id,
                content_id,
                e
            );
            LearnerCourseCatalogError::Database(e.to_string())
        })
}

pub async fn get_learner_progress(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<Option<CourseProgress>, LearnerCourseCatalogError> {
    let result = course_progress::table
        .filter(course_progress::user_id.eq(user_id))
        .filter(course_progress::course_id.eq(course_id))
        .first::<CourseProgress>(conn)
        .await;

    match result {
        Ok(progress) => Ok(Some(progress)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => {
            log::error!(
                "event=learner_progress_fetch_failed user_id={} course_id={} error={}",
                user_id,
                course_id,
                e
            );
            Err(LearnerCourseCatalogError::Database(e.to_string()))
        }
    }
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

impl From<diesel::result::Error> for TeacherCourseDashboardError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
            other => TeacherCourseDashboardError::Database(other.to_string()),
        }
    }
}

impl From<LearnerCourseCatalogError> for TeacherCourseDashboardError {
    fn from(error: LearnerCourseCatalogError) -> Self {
        match error {
            LearnerCourseCatalogError::Database(message) => {
                TeacherCourseDashboardError::Database(message)
            }
            LearnerCourseCatalogError::NotFound => {
                TeacherCourseDashboardError::Database("course not found".to_string())
            }
            LearnerCourseCatalogError::PermissionDenied(permission) => {
                TeacherCourseDashboardError::Database(format!(
                    "unexpected permission error while building teacher dashboard: {}",
                    permission
                ))
            }
        }
    }
}

impl From<diesel::result::Error> for OrganizationCourseListError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => OrganizationCourseListError::NotFound,
            other => OrganizationCourseListError::Database(other.to_string()),
        }
    }
}

impl From<LearnerCourseCatalogError> for OrganizationCourseListError {
    fn from(error: LearnerCourseCatalogError) -> Self {
        match error {
            LearnerCourseCatalogError::Database(message) => {
                OrganizationCourseListError::Database(message)
            }
            LearnerCourseCatalogError::NotFound => {
                OrganizationCourseListError::Database("course not found".to_string())
            }
            LearnerCourseCatalogError::PermissionDenied(permission) => {
                OrganizationCourseListError::Database(format!(
                    "unexpected permission error while building organization courses: {}",
                    permission
                ))
            }
        }
    }
}

impl From<TeacherCourseDashboardError> for OrganizationCourseListError {
    fn from(error: TeacherCourseDashboardError) -> Self {
        match error {
            TeacherCourseDashboardError::PermissionDenied(permission) => {
                OrganizationCourseListError::Database(format!(
                    "unexpected permission error while building organization courses: {}",
                    permission
                ))
            }
            TeacherCourseDashboardError::NotFound => {
                OrganizationCourseListError::Database("course not found".to_string())
            }
            TeacherCourseDashboardError::Database(message) => {
                OrganizationCourseListError::Database(message)
            }
        }
    }
}

async fn load_learner_course_learning_chapters(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseLearningChapter>, LearnerCourseCatalogError> {
    let chapter_rows = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .order(chapters::order.asc())
        .then_order_by(chapters::id.asc())
        .select((chapters::id, chapters::title, chapters::order))
        .load::<(i32, String, i32)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut result = Vec::with_capacity(chapter_rows.len());
    for (id, title, order) in chapter_rows {
        let content_rows = contents::table
            .filter(contents::chapter_id.eq(id))
            .order(contents::order.asc())
            .then_order_by(contents::id.asc())
            .select((
                contents::id,
                contents::chapter_id,
                contents::order,
                contents::content_type,
                contents::data,
            ))
            .load::<(i32, i32, i32, String, Option<String>)>(conn)
            .await
            .map_err(LearnerCourseCatalogError::from)?;

        let mut learning_contents = Vec::with_capacity(content_rows.len());
        for (id, chapter_id, order, content_type, data) in content_rows {
            let processing = load_latest_content_processing(conn, data.as_deref()).await?;
            let display_state = content_display_state(&content_type, data.as_deref(), &processing);
            learning_contents.push(LearnerCourseLearningContent {
                id,
                chapter_id,
                order,
                content_type,
                data,
                display_state,
                processing_status: processing.as_ref().map(|(status, _)| status.clone()),
                processing_error: processing.and_then(|(_, error)| error),
            });
        }

        result.push(LearnerCourseLearningChapter {
            id,
            title,
            order,
            contents: learning_contents,
        });
    }

    Ok(result)
}

async fn load_latest_content_processing(
    conn: &mut AsyncPgConnection,
    object_key: Option<&str>,
) -> Result<Option<(String, Option<String>)>, LearnerCourseCatalogError> {
    let Some(object_key) = object_key.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };

    upload_jobs::table
        .filter(upload_jobs::object.eq(object_key))
        .order(upload_jobs::created_at.desc())
        .select((upload_jobs::status, upload_jobs::last_error))
        .first::<(String, Option<String>)>(conn)
        .await
        .optional()
        .map_err(LearnerCourseCatalogError::from)
}

fn content_display_state(
    content_type: &str,
    data: Option<&str>,
    processing: &Option<(String, Option<String>)>,
) -> String {
    let normalized_type = content_type.trim().to_ascii_lowercase();
    let has_data = data.map(str::trim).is_some_and(|value| !value.is_empty());

    if !has_data {
        if is_media_content_type(&normalized_type) {
            return "unprocessed_upload".to_string();
        }
        return "unavailable".to_string();
    }

    if let Some((status, _)) = processing {
        return match status.as_str() {
            "queued" | "processing" => "processing".to_string(),
            "failed" => "failed_processing".to_string(),
            "done" => "ready".to_string(),
            _ => "uploaded".to_string(),
        };
    }

    if is_media_content_type(&normalized_type) {
        return "uploaded".to_string();
    }

    "ready".to_string()
}

fn is_media_content_type(content_type: &str) -> bool {
    content_type == "video"
        || content_type.starts_with("video/")
        || content_type == "document"
        || content_type == "pdf"
        || content_type.starts_with("application/pdf")
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
        description: course.description.clone(),
        topics: course
            .topics
            .clone()
            .and_then(|t| serde_json::from_str::<Vec<String>>(&t).ok())
            .unwrap_or_default(),
        prerequisites: course
            .prerequisites
            .clone()
            .map(|p| vec![p])
            .unwrap_or_default(),
        organizations,
        teachers,
        content,
        rewards,
        enrollment,
        access,
    })
}

async fn build_organization_course_list_item(
    conn: &mut AsyncPgConnection,
    course: Course,
    permissions: OrganizationCoursePermissionSummary,
) -> Result<OrganizationCourseListItem, OrganizationCourseListError> {
    let teachers = load_learner_course_teachers(conn, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let rewards = load_learner_course_reward_summary(conn, course.id).await?;
    let roster = load_teacher_course_roster_summary(conn, course.id).await?;
    let reward_queue = load_teacher_course_reward_queue_summary(conn, course.id).await?;

    Ok(OrganizationCourseListItem {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        teachers,
        content,
        rewards,
        roster,
        reward_queue,
        permissions,
    })
}

async fn build_teacher_course_dashboard_item(
    conn: &mut AsyncPgConnection,
    course: Course,
    permissions: TeacherCoursePermissionSummary,
) -> Result<TeacherCourseDashboardItem, TeacherCourseDashboardError> {
    let organizations = load_learner_course_organizations(conn, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let rewards = load_learner_course_reward_summary(conn, course.id).await?;
    let roster = load_teacher_course_roster_summary(conn, course.id).await?;
    let reward_queue = load_teacher_course_reward_queue_summary(conn, course.id).await?;

    Ok(TeacherCourseDashboardItem {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        organizations,
        content,
        rewards,
        roster,
        reward_queue,
        permissions,
    })
}

async fn load_teacher_course_workspace_chapters(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    course_lifecycle_status: &str,
) -> Result<Vec<TeacherCourseWorkspaceChapter>, TeacherCourseDashboardError> {
    let chapter_rows = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .order(chapters::order.asc())
        .then_order_by(chapters::id.asc())
        .select((chapters::id, chapters::title, chapters::order))
        .load::<(i32, String, i32)>(conn)
        .await?;

    let mut result = Vec::with_capacity(chapter_rows.len());
    for (id, title, order) in chapter_rows {
        let content_rows = contents::table
            .filter(contents::chapter_id.eq(id))
            .order(contents::order.asc())
            .then_order_by(contents::id.asc())
            .select((
                contents::id,
                contents::order,
                contents::content_type,
                contents::data,
            ))
            .load::<(i32, i32, String, Option<String>)>(conn)
            .await?;
        let mut workspace_contents = Vec::with_capacity(content_rows.len());

        for (id, order, content_type, data) in content_rows {
            let processing = load_latest_content_processing(conn, data.as_deref()).await?;
            let display_state = content_display_state(&content_type, data.as_deref(), &processing);
            workspace_contents.push(TeacherCourseWorkspaceContent {
                id,
                order,
                content_type,
                data_present: data
                    .as_deref()
                    .map(str::trim)
                    .is_some_and(|value| !value.is_empty()),
                publication_status: teacher_content_publication_status(course_lifecycle_status),
                display_state,
                processing_status: processing.as_ref().map(|(status, _)| status.clone()),
                processing_error: processing.and_then(|(_, error)| error),
            });
        }

        result.push(TeacherCourseWorkspaceChapter {
            id,
            title,
            order,
            contents: workspace_contents,
        });
    }

    Ok(result)
}

fn teacher_content_publication_status(course_lifecycle_status: &str) -> String {
    format!("inherits_course_{}", course_lifecycle_status)
}

async fn teacher_course_candidate_scope(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<TeacherCourseCandidateScope, TeacherCourseDashboardError> {
    let permission_names = teacher_course_dashboard_permission_names();
    if has_platform_teacher_course_scope(conn, actor_user_id, &permission_names).await? {
        return Ok(TeacherCourseCandidateScope::All);
    }

    let mut course_ids = BTreeSet::new();

    for course_id in direct_teacher_course_ids(conn, actor_user_id, &permission_names).await? {
        course_ids.insert(course_id);
    }
    for course_id in delegated_teacher_course_ids(conn, actor_user_id, &permission_names).await? {
        course_ids.insert(course_id);
    }

    let mut organization_ids = BTreeSet::new();
    for organization_id in
        direct_teacher_organization_ids(conn, actor_user_id, &permission_names).await?
    {
        organization_ids.insert(organization_id);
    }
    for organization_id in
        delegated_teacher_organization_ids(conn, actor_user_id, &permission_names).await?
    {
        organization_ids.insert(organization_id);
    }

    let organization_ids: Vec<i32> = organization_ids.into_iter().collect();
    for course_id in courses_for_organizations(conn, &organization_ids).await? {
        course_ids.insert(course_id);
    }

    Ok(TeacherCourseCandidateScope::CourseIds(
        course_ids.into_iter().collect(),
    ))
}

fn teacher_course_dashboard_permission_names() -> Vec<String> {
    [
        Permissions::MANAGE_COURSE_SETTINGS,
        Permissions::CREATE_CONTENT,
        Permissions::MODIFY_CONTENT,
        Permissions::APPROVE_COURSE_CONTENT,
        Permissions::MANAGE_COURSE_ENROLLMENTS,
        Permissions::APPROVE_COURSE_JOIN_REQUESTS,
        Permissions::VIEW_COURSE_REWARD_STATUS,
        Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
        Permissions::MANAGE_COURSE_REWARD_RULES,
    ]
    .into_iter()
    .map(|permission| permission.to_string())
    .collect()
}

async fn has_platform_teacher_course_scope(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<bool, TeacherCourseDashboardError> {
    let has_direct_platform_scope = diesel::select(diesel::dsl::exists(
        user_role_platform::table
            .inner_join(role_permission_platform::table.on(
                user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
            ))
            .filter(user_role_platform::user_id.eq(actor_user_id))
            .filter(role_permission_platform::permission.eq_any(permission_names)),
    ))
    .get_result::<bool>(conn)
    .await?;
    if has_direct_platform_scope {
        return Ok(true);
    }

    let now = Utc::now();
    diesel::select(diesel::dsl::exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_PLATFORM))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::permission.eq_any(permission_names))
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result::<bool>(conn)
    .await
    .map_err(TeacherCourseDashboardError::from)
}

async fn direct_teacher_course_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let rows = user_role_course::table
        .inner_join(
            role_permission_course::table
                .on(user_role_course::course_role_id.eq(role_permission_course::course_role_id)),
        )
        .filter(user_role_course::user_id.eq(actor_user_id))
        .filter(role_permission_course::permission.eq_any(permission_names))
        .select(user_role_course::course_id)
        .load::<Option<i32>>(conn)
        .await?;

    Ok(rows.into_iter().flatten().collect())
}

async fn direct_teacher_organization_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let rows = user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::user_id.eq(actor_user_id))
        .filter(role_permission_organization::permission.eq_any(permission_names))
        .select(user_role_organization::organization_id)
        .load::<Option<i32>>(conn)
        .await?;

    Ok(rows.into_iter().flatten().collect())
}

async fn delegated_teacher_course_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let now = Utc::now();
    let rows = delegated_permissions::table
        .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
        .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_COURSE))
        .filter(delegated_permissions::organization_id.is_null())
        .filter(delegated_permissions::permission.eq_any(permission_names))
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .select(delegated_permissions::course_id)
        .load::<Option<i32>>(conn)
        .await?;

    Ok(rows.into_iter().flatten().collect())
}

async fn delegated_teacher_organization_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let now = Utc::now();
    let rows = delegated_permissions::table
        .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
        .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_ORGANIZATION))
        .filter(delegated_permissions::course_id.is_null())
        .filter(delegated_permissions::permission.eq_any(permission_names))
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .select(delegated_permissions::organization_id)
        .load::<Option<i32>>(conn)
        .await?;

    Ok(rows.into_iter().flatten().collect())
}

async fn courses_for_organizations(
    conn: &mut AsyncPgConnection,
    organization_ids: &[i32],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    if organization_ids.is_empty() {
        return Ok(Vec::new());
    }

    courses_organizations::table
        .filter(courses_organizations::organization_id.eq_any(organization_ids))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)
}

async fn user_has_platform_or_organization_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, OrganizationCourseListError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, actor_user_id, &permission_name)
        .await
        .map_err(OrganizationCourseListError::from)?
    {
        return Ok(true);
    }

    user_permission_organization_request(conn, actor_user_id, organization_id, &permission_name)
        .await
        .map_err(OrganizationCourseListError::from)
}

async fn build_organization_course_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationCoursePermissionSummary, OrganizationCourseListError> {
    Ok(OrganizationCoursePermissionSummary {
        can_view_courses: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORGANIZATION,
        )
        .await?,
        can_create_courses: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::CREATE_COURSE,
        )
        .await?,
        can_manage_course_settings: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_SETTINGS,
        )
        .await?
            || user_has_platform_or_organization_permission(
                conn,
                actor_user_id,
                organization_id,
                Permissions::MANAGE_COURSE_SETTINGS,
            )
            .await?,
        can_manage_enrollments: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::APPROVE_COURSE_JOIN_REQUESTS,
        )
        .await?
            || user_has_platform_or_organization_permission(
                conn,
                actor_user_id,
                organization_id,
                Permissions::MANAGE_COURSE_ENROLLMENTS,
            )
            .await?,
        can_submit_reward_events: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT,
        )
        .await?,
        can_view_reward_reports: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORG_REWARD_REPORTS,
        )
        .await?,
        can_manage_reward_budget: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_REWARD_BUDGET,
        )
        .await?,
    })
}

async fn build_teacher_course_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<TeacherCoursePermissionSummary, TeacherCourseDashboardError> {
    let can_create_content =
        teacher_has_course_permission(conn, actor_user_id, course_id, Permissions::CREATE_CONTENT)
            .await?;
    let can_modify_content =
        teacher_has_course_permission(conn, actor_user_id, course_id, Permissions::MODIFY_CONTENT)
            .await?;
    let can_approve_content = teacher_has_course_permission(
        conn,
        actor_user_id,
        course_id,
        Permissions::APPROVE_COURSE_CONTENT,
    )
    .await?;

    Ok(TeacherCoursePermissionSummary {
        can_manage_settings: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::MANAGE_COURSE_SETTINGS,
        )
        .await?,
        can_manage_content: can_create_content || can_modify_content || can_approve_content,
        can_manage_enrollments: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::MANAGE_COURSE_ENROLLMENTS,
        )
        .await?
            || teacher_has_course_permission(
                conn,
                actor_user_id,
                course_id,
                Permissions::APPROVE_COURSE_JOIN_REQUESTS,
            )
            .await?,
        can_view_reward_candidates: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::VIEW_COURSE_REWARD_STATUS,
        )
        .await?,
        can_approve_reward_candidates: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
        )
        .await?,
        can_manage_reward_rules: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::MANAGE_COURSE_REWARD_RULES,
        )
        .await?,
    })
}

async fn teacher_has_course_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: Permissions,
) -> Result<bool, TeacherCourseDashboardError> {
    user_has_permission_for_course_context(conn, actor_user_id, course_id, &permission)
        .await
        .map_err(TeacherCourseDashboardError::from)
}

async fn load_teacher_course_roster_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRosterSummary, TeacherCourseDashboardError> {
    let enrolled_student_count = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("STUDENT"))
        .count()
        .get_result::<i64>(conn)
        .await?;
    let pending_join_request_count =
        count_course_join_requests_by_status(conn, course_id, COURSE_JOIN_STATUS_PENDING).await?;
    let waitlisted_join_request_count =
        count_course_join_requests_by_status(conn, course_id, COURSE_JOIN_STATUS_WAITLISTED)
            .await?;

    Ok(TeacherCourseRosterSummary {
        enrolled_student_count,
        pending_join_request_count,
        waitlisted_join_request_count,
    })
}

async fn count_course_join_requests_by_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    status: &str,
) -> Result<i64, TeacherCourseDashboardError> {
    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::status.eq(status))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)
}

async fn load_teacher_course_join_request_page(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    query: &TeacherCourseEnrollmentQuery,
    can_manage_enrollments: bool,
) -> Result<TeacherCourseJoinRequestPage, TeacherCourseDashboardError> {
    let mut count_query = course_join_requests::table.into_boxed();
    count_query = count_query.filter(course_join_requests::course_id.eq(course_id));
    count_query = apply_join_request_status_filter(count_query, query.status.as_deref());
    let total = count_query.count().get_result::<i64>(conn).await?;

    let mut list_query = course_join_requests::table.into_boxed();
    list_query = list_query.filter(course_join_requests::course_id.eq(course_id));
    list_query = apply_join_request_status_filter(list_query, query.status.as_deref());
    let rows = list_query
        .order(course_join_requests::updated_at.desc())
        .then_order_by(course_join_requests::id.asc())
        .limit(query.limit)
        .offset(query.offset)
        .load::<CourseJoinRequest>(conn)
        .await?;

    let mut requests = Vec::with_capacity(rows.len());
    for request in rows {
        let requester = load_teacher_enrollment_user_summary(conn, request.requester_user_id)
            .await?
            .ok_or_else(|| {
                TeacherCourseDashboardError::Database(format!(
                    "missing requester user for join request {}",
                    request.id
                ))
            })?;
        let reviewer = match request.reviewer_user_id {
            Some(user_id) => load_teacher_enrollment_user_summary(conn, user_id).await?,
            None => None,
        };
        let can_decide = can_manage_enrollments
            && matches!(
                request.status.as_str(),
                COURSE_JOIN_STATUS_PENDING | COURSE_JOIN_STATUS_WAITLISTED
            );

        requests.push(TeacherCourseJoinRequestItem {
            id: request.id,
            status: request.status,
            requester,
            reviewer,
            decision_reason: request.decision_reason,
            created_at: request.created_at,
            updated_at: request.updated_at,
            decided_at: request.decided_at,
            can_decide,
        });
    }

    Ok(TeacherCourseJoinRequestPage {
        requests,
        total,
        limit: query.limit,
        offset: query.offset,
        status: query.status.clone(),
    })
}

fn apply_join_request_status_filter<'a>(
    query: course_join_requests::BoxedQuery<'a, diesel::pg::Pg>,
    status: Option<&'a str>,
) -> course_join_requests::BoxedQuery<'a, diesel::pg::Pg> {
    match status {
        Some("all") | None => query,
        Some("open") => query.filter(
            course_join_requests::status
                .eq_any([COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED]),
        ),
        Some(status) => query.filter(course_join_requests::status.eq(status)),
    }
}

async fn load_teacher_course_roster_page(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    can_manage_enrollments: bool,
) -> Result<TeacherCourseRosterPage, TeacherCourseDashboardError> {
    let rows = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .inner_join(users::table.on(user_role_course::user_id.eq(users::id.nullable())))
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("STUDENT"))
        .order(users::name.asc())
        .then_order_by(users::id.asc())
        .select((
            users::id,
            users::name,
            users::email,
            users::email_verified,
            users::kyc_verified,
        ))
        .load::<(i32, String, String, bool, bool)>(conn)
        .await?;

    let mut learners = Vec::with_capacity(rows.len());
    let mut seen_user_ids = BTreeSet::new();
    for (id, name, email, email_verified, kyc_verified) in rows {
        if !seen_user_ids.insert(id) {
            continue;
        }
        let roles = load_actor_course_roles(conn, id, course_id).await?;
        let latest_join_request_status =
            load_latest_join_request_status(conn, course_id, id).await?;

        learners.push(TeacherCourseRosterLearner {
            user: TeacherEnrollmentUserSummary {
                id,
                name,
                email,
                email_verified,
                kyc_verified,
            },
            roles,
            latest_join_request_status,
            access_state: "enrolled".to_string(),
            can_remove: can_manage_enrollments,
            progress_supported: false,
            reward_eligibility_supported: false,
        });
    }

    let total = learners.len() as i64;
    Ok(TeacherCourseRosterPage { learners, total })
}

async fn load_teacher_enrollment_user_summary(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Option<TeacherEnrollmentUserSummary>, TeacherCourseDashboardError> {
    users::table
        .find(user_id)
        .select((
            users::id,
            users::name,
            users::email,
            users::email_verified,
            users::kyc_verified,
        ))
        .first::<(i32, String, String, bool, bool)>(conn)
        .await
        .optional()
        .map(|row| {
            row.map(|(id, name, email, email_verified, kyc_verified)| {
                TeacherEnrollmentUserSummary {
                    id,
                    name,
                    email,
                    email_verified,
                    kyc_verified,
                }
            })
        })
        .map_err(TeacherCourseDashboardError::from)
}

async fn load_latest_join_request_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    user_id: i32,
) -> Result<Option<String>, TeacherCourseDashboardError> {
    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(user_id))
        .order(course_join_requests::updated_at.desc())
        .select(course_join_requests::status)
        .first::<String>(conn)
        .await
        .optional()
        .map_err(TeacherCourseDashboardError::from)
}

async fn load_teacher_student_reward_progress(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
) -> Result<TeacherStudentRewardProgressSummary, TeacherCourseDashboardError> {
    let reward_candidate_count =
        count_student_reward_candidates(conn, course_id, student_user_id, None).await?;
    let pending_teacher_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_PENDING_TEACHER_APPROVAL),
    )
    .await?;
    let teacher_approved_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_TEACHER_APPROVED),
    )
    .await?;
    let teacher_rejected_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_TEACHER_REJECTED),
    )
    .await?;
    let completed_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_COMPLETED),
    )
    .await?;
    let failed_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_FAILED),
    )
    .await?;
    let latest_candidate = reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .filter(reward_candidates::student_user_id.eq(student_user_id))
        .order(reward_candidates::updated_at.desc())
        .then_order_by(reward_candidates::id.desc())
        .select((
            reward_candidates::id,
            reward_candidates::event_type,
            reward_candidates::status,
            reward_candidates::evidence,
            reward_candidates::teacher_decision_reason,
            reward_candidates::created_at,
            reward_candidates::updated_at,
        ))
        .first::<(
            i64,
            String,
            String,
            Value,
            Option<String>,
            DateTime<Utc>,
            DateTime<Utc>,
        )>(conn)
        .await
        .optional()?
        .map(
            |(
                id,
                event_type,
                status,
                evidence,
                teacher_decision_reason,
                created_at,
                updated_at,
            )| {
                TeacherStudentRewardCandidateSummary {
                    id,
                    event_type,
                    status,
                    evidence,
                    teacher_decision_reason,
                    created_at,
                    updated_at,
                }
            },
        );

    Ok(TeacherStudentRewardProgressSummary {
        reward_candidate_count,
        pending_teacher_count,
        teacher_approved_count,
        teacher_rejected_count,
        completed_count,
        failed_count,
        latest_candidate,
    })
}

async fn count_student_reward_candidates(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    status: Option<&str>,
) -> Result<i64, TeacherCourseDashboardError> {
    let mut query = reward_candidates::table.into_boxed();
    query = query
        .filter(reward_candidates::course_id.eq(course_id))
        .filter(reward_candidates::student_user_id.eq(student_user_id));
    if let Some(status) = status {
        query = query.filter(reward_candidates::status.eq(status));
    }

    query
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)
}

async fn load_teacher_course_reward_queue_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRewardQueueSummary, TeacherCourseDashboardError> {
    Ok(TeacherCourseRewardQueueSummary {
        pending_teacher_count: count_reward_candidates_by_status(
            conn,
            course_id,
            REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        )
        .await?,
        teacher_approved_count: count_reward_candidates_by_status(
            conn,
            course_id,
            REWARD_STATUS_TEACHER_APPROVED,
        )
        .await?,
        failed_count: count_reward_candidates_by_status(conn, course_id, REWARD_STATUS_FAILED)
            .await?,
    })
}

async fn count_reward_candidates_by_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    status: &str,
) -> Result<i64, TeacherCourseDashboardError> {
    reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .filter(reward_candidates::status.eq(status))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)
}

impl TeacherCoursePermissionSummary {
    fn has_teacher_access(&self) -> bool {
        self.can_manage_settings
            || self.can_manage_content
            || self.can_manage_enrollments
            || self.can_view_reward_candidates
            || self.can_approve_reward_candidates
            || self.can_manage_reward_rules
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::course::{
        COURSE_STATUS_APPROVED, COURSE_STATUS_ARCHIVED, COURSE_STATUS_DRAFT,
        COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED, COURSE_STATUS_SUBMITTED,
        COURSE_STATUS_SUSPENDED,
    };

    // ── course_title_search_pattern ──

    #[test]
    fn wraps_search_with_wildcards() {
        assert_eq!(course_title_search_pattern("hello"), "%hello%");
    }

    #[test]
    fn escapes_like_special_chars() {
        assert_eq!(course_title_search_pattern("50%"), r"%50\%%");
        assert_eq!(course_title_search_pattern("a_b"), r"%a\_b%");
        assert_eq!(course_title_search_pattern(r"a\b"), r"%a\\b%");
    }

    #[test]
    fn escapes_combined_special_chars() {
        assert_eq!(course_title_search_pattern(r"a\%_"), r"%a\\\%\_%");
    }

    #[test]
    fn empty_search_returns_wildcards() {
        assert_eq!(course_title_search_pattern(""), "%%");
    }

    // ── normalize_optional_string ──

    #[test]
    fn returns_trimmed_optional_string() {
        assert_eq!(
            normalize_optional_string(Some("  hello  ".into())),
            Some("hello".into())
        );
    }

    #[test]
    fn returns_none_for_empty_or_whitespace_os() {
        assert_eq!(normalize_optional_string(None), None);
        assert_eq!(normalize_optional_string(Some("".into())), None);
        assert_eq!(normalize_optional_string(Some("   ".into())), None);
    }

    // ── content_display_state ──

    #[test]
    fn text_content_with_data_is_ready() {
        assert_eq!(content_display_state("text", Some("data"), &None), "ready");
        assert_eq!(content_display_state("article", Some("data"), &None), "ready");
    }

    #[test]
    fn text_content_without_data_is_unavailable() {
        assert_eq!(content_display_state("text", None, &None), "unavailable");
        assert_eq!(
            content_display_state("article", Some(""), &None),
            "unavailable"
        );
    }

    #[test]
    fn media_content_without_data_is_unprocessed_upload() {
        assert_eq!(content_display_state("video", None, &None), "unprocessed_upload");
        assert_eq!(
            content_display_state("video", Some(""), &None),
            "unprocessed_upload"
        );
        assert_eq!(content_display_state("document", None, &None), "unprocessed_upload");
    }

    #[test]
    fn media_with_no_processing_status_is_uploaded() {
        assert_eq!(content_display_state("video", Some("data"), &None), "uploaded");
    }

    #[test]
    fn processing_status_maps_correctly() {
        assert_eq!(
            content_display_state("video", Some("data"), &Some(("queued".into(), None))),
            "processing"
        );
        assert_eq!(
            content_display_state("video", Some("data"), &Some(("processing".into(), None))),
            "processing"
        );
        assert_eq!(
            content_display_state("video", Some("data"), &Some(("done".into(), None))),
            "ready"
        );
        assert_eq!(
            content_display_state(
                "video",
                Some("data"),
                &Some(("failed".into(), Some("error".into())))
            ),
            "failed_processing"
        );
    }

    #[test]
    fn unknown_processing_status_is_uploaded() {
        assert_eq!(
            content_display_state("video", Some("data"), &Some(("unknown".into(), None))),
            "uploaded"
        );
    }

    // ── is_media_content_type ──

    #[test]
    fn recognizes_media_types() {
        assert!(is_media_content_type("video"));
        assert!(is_media_content_type("video/mp4"));
        assert!(is_media_content_type("document"));
        assert!(is_media_content_type("pdf"));
        assert!(is_media_content_type("application/pdf"));
    }

    #[test]
    fn rejects_non_media_types() {
        assert!(!is_media_content_type("text"));
        assert!(!is_media_content_type("article"));
        assert!(!is_media_content_type(""));
        assert!(!is_media_content_type("image"));
    }

    // ── teacher_content_publication_status ──

    #[test]
    fn inherits_course_lifecycle_status() {
        assert_eq!(
            teacher_content_publication_status(COURSE_STATUS_DRAFT),
            "inherits_course_draft"
        );
        assert_eq!(
            teacher_content_publication_status(COURSE_STATUS_PUBLISHED),
            "inherits_course_published"
        );
    }

    // ── teacher_course_dashboard_permission_names ──

    #[test]
    fn includes_all_expected_permissions() {
        let names = teacher_course_dashboard_permission_names();
        assert!(names.contains(&Permissions::MANAGE_COURSE_SETTINGS.to_string()));
        assert!(names.contains(&Permissions::CREATE_CONTENT.to_string()));
        assert!(names.contains(&Permissions::MODIFY_CONTENT.to_string()));
        assert!(names.contains(&Permissions::APPROVE_COURSE_CONTENT.to_string()));
        assert!(names.contains(&Permissions::MANAGE_COURSE_ENROLLMENTS.to_string()));
        assert!(names.contains(&Permissions::APPROVE_COURSE_JOIN_REQUESTS.to_string()));
        assert!(names.contains(&Permissions::VIEW_COURSE_REWARD_STATUS.to_string()));
        assert!(names.contains(&Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()));
        assert!(names.contains(&Permissions::MANAGE_COURSE_REWARD_RULES.to_string()));
        assert_eq!(names.len(), 9);
    }

    // ── TeacherCoursePermissionSummary::has_teacher_access ──

    fn summary_with(permission: &str) -> TeacherCoursePermissionSummary {
        let mut summary = TeacherCoursePermissionSummary {
            can_manage_settings: false,
            can_manage_content: false,
            can_manage_enrollments: false,
            can_view_reward_candidates: false,
            can_approve_reward_candidates: false,
            can_manage_reward_rules: false,
        };
        match permission {
            "can_manage_settings" => summary.can_manage_settings = true,
            "can_manage_content" => summary.can_manage_content = true,
            "can_manage_enrollments" => summary.can_manage_enrollments = true,
            "can_view_reward_candidates" => summary.can_view_reward_candidates = true,
            "can_approve_reward_candidates" => summary.can_approve_reward_candidates = true,
            "can_manage_reward_rules" => summary.can_manage_reward_rules = true,
            _ => {}
        }
        summary
    }

    #[test]
    fn has_access_when_any_permission_is_true() {
        let perms = [
            "can_manage_settings",
            "can_manage_content",
            "can_manage_enrollments",
            "can_view_reward_candidates",
            "can_approve_reward_candidates",
            "can_manage_reward_rules",
        ];
        for p in perms {
            assert!(summary_with(p).has_teacher_access(), "should have access with {}", p);
        }
    }

    #[test]
    fn no_access_when_all_permissions_false() {
        let summary = TeacherCoursePermissionSummary {
            can_manage_settings: false,
            can_manage_content: false,
            can_manage_enrollments: false,
            can_view_reward_candidates: false,
            can_approve_reward_candidates: false,
            can_manage_reward_rules: false,
        };
        assert!(!summary.has_teacher_access());
    }

    // ── normalize_course_status ──

    #[test]
    fn normalizes_all_valid_course_statuses() {
        let statuses = vec![
            COURSE_STATUS_DRAFT,
            COURSE_STATUS_SUBMITTED,
            COURSE_STATUS_NEEDS_CHANGES,
            COURSE_STATUS_APPROVED,
            COURSE_STATUS_PUBLISHED,
            COURSE_STATUS_ARCHIVED,
            COURSE_STATUS_SUSPENDED,
        ];
        for status in statuses {
            assert_eq!(normalize_course_status(status).unwrap(), status);
        }
    }

    #[test]
    fn normalizes_course_status_case_and_whitespace() {
        assert_eq!(normalize_course_status("  DRAFT  ").unwrap(), COURSE_STATUS_DRAFT);
        assert_eq!(
            normalize_course_status("Published").unwrap(),
            COURSE_STATUS_PUBLISHED
        );
    }

    #[test]
    fn rejects_invalid_course_status() {
        assert!(normalize_course_status("").is_err());
        assert!(normalize_course_status("unknown").is_err());
        assert!(normalize_course_status("deleted").is_err());
    }

    // ── Query constructors ──

    #[test]
    fn course_discovery_query_defaults() {
        let q = CourseDiscoveryQuery::new(None, None, None, None);
        assert_eq!(q.search, None);
        assert_eq!(q.organization_id, None);
        assert_eq!(q.limit, 25);
        assert_eq!(q.offset, 0);
    }

    #[test]
    fn course_discovery_query_clamps_limits() {
        let q = CourseDiscoveryQuery::new(None, None, Some(0), Some(-1));
        assert_eq!(q.limit, 1);
        assert_eq!(q.offset, 0);
        let q2 = CourseDiscoveryQuery::new(None, None, Some(500), None);
        assert_eq!(q2.limit, 100);
    }

    #[test]
    fn course_discovery_query_trims_search() {
        let q = CourseDiscoveryQuery::new(Some("  query  ".into()), None, None, None);
        assert_eq!(q.search, Some("query".into()));
        let q2 = CourseDiscoveryQuery::new(Some("   ".into()), None, None, None);
        assert_eq!(q2.search, None);
    }

    #[test]
    fn learner_catalog_query_defaults() {
        let q = LearnerCourseCatalogQuery::new(None, None, None, None, None, None, None);
        assert_eq!(q.limit, 25);
        assert_eq!(q.offset, 0);
        assert_eq!(q.search, None);
        assert_eq!(q.reward_available, None);
    }

    #[test]
    fn teacher_dashboard_query_defaults() {
        let q = TeacherCourseDashboardQuery::new(None, None, None, None);
        assert_eq!(q.limit, 25);
        assert_eq!(q.offset, 0);
    }

    #[test]
    fn teacher_enrollment_query_defaults_status_to_open() {
        let q = TeacherCourseEnrollmentQuery::new(None, None, None);
        assert_eq!(q.status, Some("open".into()));
        assert_eq!(q.limit, 25);
        assert_eq!(q.offset, 0);
    }

    #[test]
    fn teacher_enrollment_query_normalizes_custom_status() {
        let q = TeacherCourseEnrollmentQuery::new(Some("  PENDING  ".into()), None, None);
        assert_eq!(q.status, Some("pending".into()));
    }

    #[test]
    fn organization_course_list_query_defaults() {
        let q = OrganizationCourseListQuery::new(None, None, None, None, None);
        assert_eq!(q.limit, 25);
        assert_eq!(q.offset, 0);
        assert_eq!(q.reward_available, None);
    }

    // ── CourseLifecycleError from diesel::Error ──

    #[test]
    fn course_lifecycle_diesel_not_found_maps_to_not_found() {
        assert_eq!(
            CourseLifecycleError::from(diesel::result::Error::NotFound),
            CourseLifecycleError::NotFound
        );
    }

    #[test]
    fn course_lifecycle_diesel_other_errors_map_to_database() {
        assert!(matches!(
            CourseLifecycleError::from(diesel::result::Error::RollbackTransaction),
            CourseLifecycleError::Database(_)
        ));
    }
}
