use serde::{Deserialize, Serialize};

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewOutput, TeacherApplicationPlatformReviewQuery,
};
use crate::http::teacher_applications::platform_review_item_dto::{
    PlatformTeacherApplicationItemResponse, PlatformTeacherApplicationPermissionsResponse,
    TeacherApplicationDashboardSummaryResponse,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub(super) struct PlatformTeacherApplicationsParams {
    pub(super) status: Option<String>,
    pub(super) search: Option<String>,
    pub(super) limit: Option<i64>,
    pub(super) offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub(super) struct PlatformTeacherApplicationsResponse {
    pub(super) applications: Vec<PlatformTeacherApplicationItemResponse>,
    pub(super) summary: TeacherApplicationDashboardSummaryResponse,
    pub(super) operator_permissions: PlatformTeacherApplicationPermissionsResponse,
    pub(super) total: i64,
    pub(super) limit: i64,
    pub(super) offset: i64,
    pub(super) status: Option<String>,
    pub(super) search: Option<String>,
}

impl PlatformTeacherApplicationsParams {
    pub(super) fn into_query(self, actor_user_id: i32) -> TeacherApplicationPlatformReviewQuery {
        TeacherApplicationPlatformReviewQuery {
            actor_user_id,
            limit: self.limit,
            offset: self.offset,
            search: self.search,
            status: self.status,
        }
    }
}

impl From<TeacherApplicationPlatformReviewOutput> for PlatformTeacherApplicationsResponse {
    fn from(output: TeacherApplicationPlatformReviewOutput) -> Self {
        Self {
            applications: output.applications.into_iter().map(Into::into).collect(),
            limit: output.limit,
            offset: output.offset,
            operator_permissions: output.operator_permissions.into(),
            search: output.search,
            status: output.status,
            summary: output.summary.into(),
            total: output.total,
        }
    }
}
