use serde::Serialize;

use crate::application::learning::list_teacher_course_dashboard::TeacherCourseDashboardListOutput;
use crate::application::learning::teacher_course_dashboard::{
    TeacherCourseContentSummaryOutput, TeacherCourseDashboardItemOutput,
    TeacherCourseDashboardOrganizationOutput, TeacherCoursePermissionSummaryOutput,
    TeacherCourseRewardQueueSummaryOutput, TeacherCourseRewardSummaryOutput,
    TeacherCourseRosterSummaryOutput,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseDashboardResponse {
    pub courses: Vec<TeacherCourseDashboardItemResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseDashboardItemResponse {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Option<String>,
    pub prerequisites: Option<String>,
    pub organizations: Vec<TeacherCourseDashboardOrganizationResponse>,
    pub content: TeacherCourseContentSummaryResponse,
    pub rewards: TeacherCourseRewardSummaryResponse,
    pub roster: TeacherCourseRosterSummaryResponse,
    pub reward_queue: TeacherCourseRewardQueueSummaryResponse,
    pub permissions: TeacherCoursePermissionSummaryResponse,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseDashboardOrganizationResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseContentSummaryResponse {
    pub chapter_count: usize,
    pub content_count: usize,
    pub content_types: Vec<String>,
    pub has_content: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseRewardSummaryResponse {
    pub available: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub token_amounts: Vec<String>,
    pub payment_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseRosterSummaryResponse {
    pub enrolled_student_count: i64,
    pub pending_join_request_count: i64,
    pub waitlisted_join_request_count: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseRewardQueueSummaryResponse {
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub failed_count: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCoursePermissionSummaryResponse {
    pub can_manage_settings: bool,
    pub can_manage_content: bool,
    pub can_manage_enrollments: bool,
    pub can_view_reward_candidates: bool,
    pub can_approve_reward_candidates: bool,
    pub can_manage_reward_rules: bool,
}

impl From<TeacherCourseDashboardListOutput> for TeacherCourseDashboardResponse {
    fn from(output: TeacherCourseDashboardListOutput) -> Self {
        Self {
            courses: output.courses.into_iter().map(Into::into).collect(),
            total: output.total,
            limit: output.limit,
            offset: output.offset,
            search: output.search,
            lifecycle_status: output.lifecycle_status,
        }
    }
}

impl From<TeacherCourseDashboardItemOutput> for TeacherCourseDashboardItemResponse {
    fn from(course: TeacherCourseDashboardItemOutput) -> Self {
        Self {
            id: course.id,
            title: course.title,
            lifecycle_status: course.lifecycle_status,
            description: course.description,
            topics: course.topics,
            prerequisites: course.prerequisites,
            organizations: course.organizations.into_iter().map(Into::into).collect(),
            content: course.content.into(),
            rewards: course.rewards.into(),
            roster: course.roster.into(),
            reward_queue: course.reward_queue.into(),
            permissions: course.permissions.into(),
        }
    }
}

impl From<TeacherCourseDashboardOrganizationOutput> for TeacherCourseDashboardOrganizationResponse {
    fn from(organization: TeacherCourseDashboardOrganizationOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
        }
    }
}

impl From<TeacherCourseContentSummaryOutput> for TeacherCourseContentSummaryResponse {
    fn from(content: TeacherCourseContentSummaryOutput) -> Self {
        Self {
            chapter_count: content.chapter_count,
            content_count: content.content_count,
            content_types: content.content_types,
            has_content: content.has_content,
        }
    }
}

impl From<TeacherCourseRewardSummaryOutput> for TeacherCourseRewardSummaryResponse {
    fn from(rewards: TeacherCourseRewardSummaryOutput) -> Self {
        Self {
            available: rewards.available,
            active_policy_count: rewards.active_policy_count,
            event_types: rewards.event_types,
            token_amounts: rewards.token_amounts,
            payment_strategies: rewards.payment_strategies,
        }
    }
}

impl From<TeacherCourseRosterSummaryOutput> for TeacherCourseRosterSummaryResponse {
    fn from(roster: TeacherCourseRosterSummaryOutput) -> Self {
        Self {
            enrolled_student_count: roster.enrolled_student_count,
            pending_join_request_count: roster.pending_join_request_count,
            waitlisted_join_request_count: roster.waitlisted_join_request_count,
        }
    }
}

impl From<TeacherCourseRewardQueueSummaryOutput> for TeacherCourseRewardQueueSummaryResponse {
    fn from(queue: TeacherCourseRewardQueueSummaryOutput) -> Self {
        Self {
            pending_teacher_count: queue.pending_teacher_count,
            teacher_approved_count: queue.teacher_approved_count,
            failed_count: queue.failed_count,
        }
    }
}

impl From<TeacherCoursePermissionSummaryOutput> for TeacherCoursePermissionSummaryResponse {
    fn from(permissions: TeacherCoursePermissionSummaryOutput) -> Self {
        Self {
            can_manage_settings: permissions.can_manage_settings,
            can_manage_content: permissions.can_manage_content,
            can_manage_enrollments: permissions.can_manage_enrollments,
            can_view_reward_candidates: permissions.can_view_reward_candidates,
            can_approve_reward_candidates: permissions.can_approve_reward_candidates,
            can_manage_reward_rules: permissions.can_manage_reward_rules,
        }
    }
}
