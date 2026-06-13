use serde::Serialize;

use crate::application::organizations::list_organization_courses::{
    OrganizationCourseContentSummaryOutput, OrganizationCourseListItemOutput,
    OrganizationCoursePermissionSummaryOutput, OrganizationCourseRewardQueueSummaryOutput,
    OrganizationCourseRewardSummaryOutput, OrganizationCourseRosterSummaryOutput,
    OrganizationCourseSummaryOutput, OrganizationCourseTeacherOutput,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCourseSummaryResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCourseListItemResponse {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub teachers: Vec<OrganizationCourseTeacherResponse>,
    pub content: OrganizationCourseContentSummaryResponse,
    pub rewards: OrganizationCourseRewardSummaryResponse,
    pub roster: OrganizationCourseRosterSummaryResponse,
    pub reward_queue: OrganizationCourseRewardQueueSummaryResponse,
    pub permissions: OrganizationCoursePermissionSummaryResponse,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCourseTeacherResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCourseContentSummaryResponse {
    pub chapter_count: usize,
    pub content_count: usize,
    pub content_types: Vec<String>,
    pub has_content: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCourseRewardSummaryResponse {
    pub available: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub token_amounts: Vec<String>,
    pub payment_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCourseRosterSummaryResponse {
    pub enrolled_student_count: i64,
    pub pending_join_request_count: i64,
    pub waitlisted_join_request_count: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCourseRewardQueueSummaryResponse {
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub failed_count: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCoursePermissionSummaryResponse {
    pub can_view_courses: bool,
    pub can_create_courses: bool,
    pub can_manage_course_settings: bool,
    pub can_manage_enrollments: bool,
    pub can_submit_reward_events: bool,
    pub can_view_reward_reports: bool,
    pub can_manage_reward_budget: bool,
}

impl From<OrganizationCourseSummaryOutput> for OrganizationCourseSummaryResponse {
    fn from(organization: OrganizationCourseSummaryOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
        }
    }
}

impl From<OrganizationCourseListItemOutput> for OrganizationCourseListItemResponse {
    fn from(course: OrganizationCourseListItemOutput) -> Self {
        Self {
            id: course.id,
            title: course.title,
            lifecycle_status: course.lifecycle_status,
            teachers: course.teachers.into_iter().map(Into::into).collect(),
            content: course.content.into(),
            rewards: course.rewards.into(),
            roster: course.roster.into(),
            reward_queue: course.reward_queue.into(),
            permissions: course.permissions.into(),
        }
    }
}

impl From<OrganizationCourseTeacherOutput> for OrganizationCourseTeacherResponse {
    fn from(teacher: OrganizationCourseTeacherOutput) -> Self {
        Self {
            id: teacher.id,
            name: teacher.name,
        }
    }
}

impl From<OrganizationCourseContentSummaryOutput> for OrganizationCourseContentSummaryResponse {
    fn from(content: OrganizationCourseContentSummaryOutput) -> Self {
        Self {
            chapter_count: content.chapter_count,
            content_count: content.content_count,
            content_types: content.content_types,
            has_content: content.has_content,
        }
    }
}

impl From<OrganizationCourseRewardSummaryOutput> for OrganizationCourseRewardSummaryResponse {
    fn from(rewards: OrganizationCourseRewardSummaryOutput) -> Self {
        Self {
            available: rewards.available,
            active_policy_count: rewards.active_policy_count,
            event_types: rewards.event_types,
            token_amounts: rewards.token_amounts,
            payment_strategies: rewards.payment_strategies,
        }
    }
}

impl From<OrganizationCourseRosterSummaryOutput> for OrganizationCourseRosterSummaryResponse {
    fn from(roster: OrganizationCourseRosterSummaryOutput) -> Self {
        Self {
            enrolled_student_count: roster.enrolled_student_count,
            pending_join_request_count: roster.pending_join_request_count,
            waitlisted_join_request_count: roster.waitlisted_join_request_count,
        }
    }
}

impl From<OrganizationCourseRewardQueueSummaryOutput>
    for OrganizationCourseRewardQueueSummaryResponse
{
    fn from(queue: OrganizationCourseRewardQueueSummaryOutput) -> Self {
        Self {
            pending_teacher_count: queue.pending_teacher_count,
            teacher_approved_count: queue.teacher_approved_count,
            failed_count: queue.failed_count,
        }
    }
}

impl From<OrganizationCoursePermissionSummaryOutput>
    for OrganizationCoursePermissionSummaryResponse
{
    fn from(permissions: OrganizationCoursePermissionSummaryOutput) -> Self {
        Self {
            can_view_courses: permissions.can_view_courses,
            can_create_courses: permissions.can_create_courses,
            can_manage_course_settings: permissions.can_manage_course_settings,
            can_manage_enrollments: permissions.can_manage_enrollments,
            can_submit_reward_events: permissions.can_submit_reward_events,
            can_view_reward_reports: permissions.can_view_reward_reports,
            can_manage_reward_budget: permissions.can_manage_reward_budget,
        }
    }
}
