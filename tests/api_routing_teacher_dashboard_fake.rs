use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::get_teacher_course_workspace::{
    TeacherCoursePublicationSummaryOutput, TeacherCourseWorkspaceOutput,
    TeacherCourseWorkspaceQuery, TeacherCourseWorkspaceUseCase,
};
use rust_learn::application::learning::list_teacher_course_dashboard::{
    TeacherCourseDashboardListOutput, TeacherCourseDashboardListQuery,
    TeacherCourseDashboardListUseCase,
};
use rust_learn::application::learning::teacher_course_dashboard::{
    TeacherCourseContentSummaryOutput, TeacherCourseDashboardError,
    TeacherCourseDashboardItemOutput, TeacherCoursePermissionSummaryOutput,
    TeacherCourseRewardQueueSummaryOutput, TeacherCourseRewardSummaryOutput,
    TeacherCourseRosterSummaryOutput,
};

struct RouteOnlyTeacherDashboardUseCase;
struct RouteOnlyTeacherWorkspaceUseCase;

pub fn teacher_dashboard_data() -> web::Data<Arc<dyn TeacherCourseDashboardListUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyTeacherDashboardUseCase) as Arc<dyn TeacherCourseDashboardListUseCase>
    )
}

pub fn teacher_workspace_data() -> web::Data<Arc<dyn TeacherCourseWorkspaceUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyTeacherWorkspaceUseCase) as Arc<dyn TeacherCourseWorkspaceUseCase>
    )
}

impl TeacherCourseDashboardListUseCase for RouteOnlyTeacherDashboardUseCase {
    fn list_teacher_course_dashboard(
        &self,
        query: TeacherCourseDashboardListQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseDashboardListOutput, TeacherCourseDashboardError>> {
        ready(Ok(TeacherCourseDashboardListOutput {
            courses: Vec::new(),
            total: 0,
            limit: query.limit,
            offset: query.offset,
            search: query.search,
            lifecycle_status: query.lifecycle_status,
        }))
        .boxed()
    }
}

impl TeacherCourseWorkspaceUseCase for RouteOnlyTeacherWorkspaceUseCase {
    fn get_teacher_course_workspace(
        &self,
        _query: TeacherCourseWorkspaceQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseWorkspaceOutput, TeacherCourseDashboardError>> {
        ready(Ok(TeacherCourseWorkspaceOutput {
            course: route_course(),
            teacher_roles: Vec::new(),
            publication: TeacherCoursePublicationSummaryOutput {
                course_lifecycle_status: "draft".to_string(),
                content_publication_status_supported: false,
            },
            chapters: Vec::new(),
        }))
        .boxed()
    }
}

fn route_course() -> TeacherCourseDashboardItemOutput {
    TeacherCourseDashboardItemOutput {
        id: 12,
        title: "Route smoke".to_string(),
        lifecycle_status: "draft".to_string(),
        organizations: Vec::new(),
        content: TeacherCourseContentSummaryOutput {
            chapter_count: 0,
            content_count: 0,
            content_types: Vec::new(),
            has_content: false,
        },
        rewards: TeacherCourseRewardSummaryOutput {
            available: false,
            active_policy_count: 0,
            event_types: Vec::new(),
            token_amounts: Vec::new(),
            payment_strategies: Vec::new(),
        },
        roster: TeacherCourseRosterSummaryOutput {
            enrolled_student_count: 0,
            pending_join_request_count: 0,
            waitlisted_join_request_count: 0,
        },
        reward_queue: TeacherCourseRewardQueueSummaryOutput {
            pending_teacher_count: 0,
            teacher_approved_count: 0,
            failed_count: 0,
        },
        permissions: TeacherCoursePermissionSummaryOutput {
            can_manage_settings: false,
            can_manage_content: false,
            can_manage_enrollments: false,
            can_view_reward_candidates: false,
            can_approve_reward_candidates: false,
            can_manage_reward_rules: false,
        },
    }
}
