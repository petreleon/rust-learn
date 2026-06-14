use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::get_teacher_course_students::{
    TeacherCourseStudentsOutput, TeacherCourseStudentsQuery, TeacherCourseStudentsUseCase,
};
use rust_learn::application::learning::teacher_course_dashboard::{
    TeacherCourseContentSummaryOutput, TeacherCourseDashboardError,
    TeacherCourseDashboardItemOutput, TeacherCoursePermissionSummaryOutput,
    TeacherCourseRewardQueueSummaryOutput, TeacherCourseRewardSummaryOutput,
    TeacherCourseRosterSummaryOutput,
};
use rust_learn::application::learning::teacher_course_enrollment::TeacherCourseRewardEligibilitySummaryOutput;

struct RouteOnlyTeacherStudentsUseCase;

pub fn teacher_students_data() -> web::Data<Arc<dyn TeacherCourseStudentsUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyTeacherStudentsUseCase) as Arc<dyn TeacherCourseStudentsUseCase>
    )
}

impl TeacherCourseStudentsUseCase for RouteOnlyTeacherStudentsUseCase {
    fn get_teacher_course_students(
        &self,
        _query: TeacherCourseStudentsQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseStudentsOutput, TeacherCourseDashboardError>> {
        ready(Ok(TeacherCourseStudentsOutput {
            course: route_course(),
            teacher_roles: Vec::new(),
            students: Vec::new(),
            total: 0,
            progress_supported: true,
            reward_eligibility_supported: true,
            reward_eligibility: TeacherCourseRewardEligibilitySummaryOutput {
                supported: true,
                active_policy_count: 0,
                event_types: Vec::new(),
            },
            reward_evidence_supported: true,
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
