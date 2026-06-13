use serde::Serialize;

use crate::application::learning::learner_course_catalog::{
    LearnerCourseAccessSummaryOutput, LearnerCourseCatalogItemOutput,
    LearnerCourseCatalogOrganizationOutput, LearnerCourseCatalogTeacherOutput,
    LearnerCourseContentSummaryOutput, LearnerCourseEnrollmentSummaryOutput,
    LearnerCourseRewardSummaryOutput,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseCatalogItemResponse {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Vec<String>,
    pub prerequisites: Vec<String>,
    pub organizations: Vec<LearnerCourseCatalogOrganizationResponse>,
    pub teachers: Vec<LearnerCourseCatalogTeacherResponse>,
    pub content: LearnerCourseContentSummaryResponse,
    pub rewards: LearnerCourseRewardSummaryResponse,
    pub enrollment: LearnerCourseEnrollmentSummaryResponse,
    pub access: LearnerCourseAccessSummaryResponse,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseCatalogOrganizationResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseCatalogTeacherResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseContentSummaryResponse {
    pub chapter_count: usize,
    pub content_count: usize,
    pub content_types: Vec<String>,
    pub has_content: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseRewardSummaryResponse {
    pub available: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub token_amounts: Vec<String>,
    pub payment_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseEnrollmentSummaryResponse {
    pub state: String,
    pub request_id: Option<i64>,
    pub can_request_join: bool,
    pub reason: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseAccessSummaryResponse {
    pub can_view_course: bool,
    pub can_view_content: bool,
    pub can_view_rewards: bool,
    pub can_request_join: bool,
}

impl From<LearnerCourseCatalogItemOutput> for LearnerCourseCatalogItemResponse {
    fn from(course: LearnerCourseCatalogItemOutput) -> Self {
        Self {
            id: course.id,
            title: course.title,
            lifecycle_status: course.lifecycle_status,
            description: course.description,
            topics: course.topics,
            prerequisites: course.prerequisites,
            organizations: course.organizations.into_iter().map(Into::into).collect(),
            teachers: course.teachers.into_iter().map(Into::into).collect(),
            content: course.content.into(),
            rewards: course.rewards.into(),
            enrollment: course.enrollment.into(),
            access: course.access.into(),
        }
    }
}

impl From<LearnerCourseCatalogOrganizationOutput> for LearnerCourseCatalogOrganizationResponse {
    fn from(organization: LearnerCourseCatalogOrganizationOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
        }
    }
}

impl From<LearnerCourseCatalogTeacherOutput> for LearnerCourseCatalogTeacherResponse {
    fn from(teacher: LearnerCourseCatalogTeacherOutput) -> Self {
        Self {
            id: teacher.id,
            name: teacher.name,
        }
    }
}

impl From<LearnerCourseContentSummaryOutput> for LearnerCourseContentSummaryResponse {
    fn from(content: LearnerCourseContentSummaryOutput) -> Self {
        Self {
            chapter_count: content.chapter_count,
            content_count: content.content_count,
            content_types: content.content_types,
            has_content: content.has_content,
        }
    }
}

impl From<LearnerCourseRewardSummaryOutput> for LearnerCourseRewardSummaryResponse {
    fn from(rewards: LearnerCourseRewardSummaryOutput) -> Self {
        Self {
            available: rewards.available,
            active_policy_count: rewards.active_policy_count,
            event_types: rewards.event_types,
            token_amounts: rewards.token_amounts,
            payment_strategies: rewards.payment_strategies,
        }
    }
}

impl From<LearnerCourseEnrollmentSummaryOutput> for LearnerCourseEnrollmentSummaryResponse {
    fn from(enrollment: LearnerCourseEnrollmentSummaryOutput) -> Self {
        Self {
            state: enrollment.state,
            request_id: enrollment.request_id,
            can_request_join: enrollment.can_request_join,
            reason: enrollment.reason,
            roles: enrollment.roles,
        }
    }
}

impl From<LearnerCourseAccessSummaryOutput> for LearnerCourseAccessSummaryResponse {
    fn from(access: LearnerCourseAccessSummaryOutput) -> Self {
        Self {
            can_view_course: access.can_view_course,
            can_view_content: access.can_view_content,
            can_view_rewards: access.can_view_rewards,
            can_request_join: access.can_request_join,
        }
    }
}
