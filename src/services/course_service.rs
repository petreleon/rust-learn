use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    chapters, contents, course_join_requests, course_progress, course_roles, courses,
    courses_organizations, delegated_permissions, organizations,
    pending_course_organization_invites, reward_candidates, reward_policies,
    role_permission_course, role_permission_organization, role_permission_platform, upload_jobs,
    user_role_course, user_role_organization, user_role_platform, users,
};
use crate::domain::rewards::policy::{
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
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

include!("course_service/query_types.rs");
include!("course_service/teacher_course_types.rs");
include!("course_service/teacher_enrollment_types.rs");
include!("course_service/learner_course_types.rs");
include!("course_service/errors.rs");
include!("course_service/query_builders.rs");
include!("course_service/course_discovery.rs");
include!("course_service/organization_discovery.rs");
include!("course_service/teacher_workspaces.rs");
include!("course_service/learner_course_reads.rs");
include!("course_service/learner_progress.rs");
include!("course_service/shared_helpers.rs");
include!("course_service/learner_learning_helpers.rs");
include!("course_service/catalog_dashboard_builders.rs");
include!("course_service/teacher_scope.rs");
include!("course_service/teacher_delegated_scope.rs");
include!("course_service/course_permission_summaries.rs");
include!("course_service/teacher_join_requests.rs");
include!("course_service/teacher_reward_eligibility.rs");
include!("course_service/teacher_roster_pages.rs");
include!("course_service/teacher_reward_progress.rs");
include!("course_service/teacher_student_progress.rs");
include!("course_service/learner_metadata.rs");
include!("course_service/learner_enrollment.rs");
include!("course_service/learner_permissions.rs");
include!("course_service/course_mutations.rs");
include!("course_service/course_lifecycle_invites.rs");

#[cfg(test)]
mod tests;
