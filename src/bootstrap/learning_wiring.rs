use std::sync::Arc;

use actix_web::web;

use crate::application::learning::assign_course_role::CourseRoleAssignmentUseCase;
use crate::application::learning::course_enrollment::CourseEnrollmentUseCase;
use crate::application::learning::create_course::CourseCreationUseCase;
use crate::application::learning::delete_course::CourseDeletionUseCase;
use crate::application::learning::discover_courses::CourseDiscoveryUseCase;
use crate::application::learning::get_course::CourseReadUseCase;
use crate::application::learning::get_learner_course_detail::LearnerCourseDetailUseCase;
use crate::application::learning::get_learner_course_learning::LearnerCourseLearningUseCase;
use crate::application::learning::get_teacher_course_enrollment_workspace::TeacherCourseEnrollmentWorkspaceUseCase;
use crate::application::learning::get_teacher_course_students::TeacherCourseStudentsUseCase;
use crate::application::learning::get_teacher_course_workspace::TeacherCourseWorkspaceUseCase;
use crate::application::learning::learner_progress::LearnerProgressUseCase;
use crate::application::learning::list_assessment_attempts::AssessmentAttemptsUseCase;
use crate::application::learning::list_course_assessments::CourseAssessmentsUseCase;
use crate::application::learning::list_course_organizations::CourseOrganizationsUseCase;
use crate::application::learning::list_learner_course_catalog::LearnerCourseCatalogListUseCase;
use crate::application::learning::list_teacher_course_dashboard::TeacherCourseDashboardListUseCase;
use crate::application::learning::submit_assessment_attempt::AssessmentSubmissionUseCase;
use crate::application::learning::update_course::CourseUpdateUseCase;
use crate::application::learning::update_course_lifecycle::CourseLifecycleUseCase;
use crate::infra::postgres::learning::assessment_read_use_case::PostgresAssessmentReadUseCase;
use crate::infra::postgres::learning::assessment_submission_use_case::PostgresAssessmentSubmissionUseCase;
use crate::infra::postgres::learning::course_creation_use_case::PostgresCourseCreationUseCase;
use crate::infra::postgres::learning::course_deletion_use_case::PostgresCourseDeletionUseCase;
use crate::infra::postgres::learning::course_discovery_use_case::PostgresCourseDiscoveryUseCase;
use crate::infra::postgres::learning::course_enrollment_use_case::PostgresCourseEnrollmentUseCase;
use crate::infra::postgres::learning::course_lifecycle_use_case::PostgresCourseLifecycleUseCase;
use crate::infra::postgres::learning::course_organization_use_case::PostgresCourseOrganizationsUseCase;
use crate::infra::postgres::learning::course_read_use_case::PostgresCourseReadUseCase;
use crate::infra::postgres::learning::course_role_assignment_use_case::PostgresCourseRoleAssignmentUseCase;
use crate::infra::postgres::learning::course_update_use_case::PostgresCourseUpdateUseCase;
use crate::infra::postgres::learning::learner_course_catalog_list_use_case::PostgresLearnerCourseCatalogListUseCase;
use crate::infra::postgres::learning::learner_course_detail_use_case::PostgresLearnerCourseDetailUseCase;
use crate::infra::postgres::learning::learner_course_learning_use_case::PostgresLearnerCourseLearningUseCase;
use crate::infra::postgres::learning::learner_progress_use_case::PostgresLearnerProgressUseCase;
use crate::infra::postgres::learning::teacher_course_dashboard_list_use_case::PostgresTeacherCourseDashboardListUseCase;
use crate::infra::postgres::learning::teacher_course_enrollment_workspace_use_case::PostgresTeacherCourseEnrollmentWorkspaceUseCase;
use crate::infra::postgres::learning::teacher_course_students_use_case::PostgresTeacherCourseStudentsUseCase;
use crate::infra::postgres::learning::teacher_course_workspace_use_case::PostgresTeacherCourseWorkspaceUseCase;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct LearningUseCases {
    pub assessment_attempts: Arc<dyn AssessmentAttemptsUseCase>,
    pub assessment_submission: Arc<dyn AssessmentSubmissionUseCase>,
    pub course_assessments: Arc<dyn CourseAssessmentsUseCase>,
    pub course_creation: Arc<dyn CourseCreationUseCase>,
    pub course_deletion: Arc<dyn CourseDeletionUseCase>,
    pub course_discovery: Arc<dyn CourseDiscoveryUseCase>,
    pub course_enrollment: Arc<dyn CourseEnrollmentUseCase>,
    pub course_lifecycle: Arc<dyn CourseLifecycleUseCase>,
    pub course_organizations: Arc<dyn CourseOrganizationsUseCase>,
    pub course_read: Arc<dyn CourseReadUseCase>,
    pub course_role_assignment: Arc<dyn CourseRoleAssignmentUseCase>,
    pub course_update: Arc<dyn CourseUpdateUseCase>,
    pub learner_course_catalog: Arc<dyn LearnerCourseCatalogListUseCase>,
    pub learner_course_detail: Arc<dyn LearnerCourseDetailUseCase>,
    pub learner_course_learning: Arc<dyn LearnerCourseLearningUseCase>,
    pub learner_progress: Arc<dyn LearnerProgressUseCase>,
    pub teacher_course_dashboard: Arc<dyn TeacherCourseDashboardListUseCase>,
    pub teacher_course_enrollment_workspace: Arc<dyn TeacherCourseEnrollmentWorkspaceUseCase>,
    pub teacher_course_students: Arc<dyn TeacherCourseStudentsUseCase>,
    pub teacher_course_workspace: Arc<dyn TeacherCourseWorkspaceUseCase>,
}

pub fn build_learning_use_cases(pool: &DbPool) -> LearningUseCases {
    LearningUseCases {
        assessment_attempts: Arc::new(PostgresAssessmentReadUseCase::new(pool.clone())),
        assessment_submission: Arc::new(PostgresAssessmentSubmissionUseCase::new(pool.clone())),
        course_assessments: Arc::new(PostgresAssessmentReadUseCase::new(pool.clone())),
        course_creation: Arc::new(PostgresCourseCreationUseCase::new(pool.clone())),
        course_deletion: Arc::new(PostgresCourseDeletionUseCase::new(pool.clone())),
        course_discovery: Arc::new(PostgresCourseDiscoveryUseCase::new(pool.clone())),
        course_enrollment: Arc::new(PostgresCourseEnrollmentUseCase::new(pool.clone())),
        course_lifecycle: Arc::new(PostgresCourseLifecycleUseCase::new(pool.clone())),
        course_organizations: Arc::new(PostgresCourseOrganizationsUseCase::new(pool.clone())),
        course_read: Arc::new(PostgresCourseReadUseCase::new(pool.clone())),
        course_role_assignment: Arc::new(PostgresCourseRoleAssignmentUseCase::new(pool.clone())),
        course_update: Arc::new(PostgresCourseUpdateUseCase::new(pool.clone())),
        learner_course_catalog: Arc::new(PostgresLearnerCourseCatalogListUseCase::new(
            pool.clone(),
        )),
        learner_course_detail: Arc::new(PostgresLearnerCourseDetailUseCase::new(pool.clone())),
        learner_course_learning: Arc::new(PostgresLearnerCourseLearningUseCase::new(pool.clone())),
        learner_progress: Arc::new(PostgresLearnerProgressUseCase::new(pool.clone())),
        teacher_course_dashboard: Arc::new(PostgresTeacherCourseDashboardListUseCase::new(
            pool.clone(),
        )),
        teacher_course_enrollment_workspace: Arc::new(
            PostgresTeacherCourseEnrollmentWorkspaceUseCase::new(pool.clone()),
        ),
        teacher_course_students: Arc::new(PostgresTeacherCourseStudentsUseCase::new(pool.clone())),
        teacher_course_workspace: Arc::new(PostgresTeacherCourseWorkspaceUseCase::new(
            pool.clone(),
        )),
    }
}

pub fn configure_learning_app_data(cfg: &mut web::ServiceConfig, use_cases: &LearningUseCases) {
    cfg.app_data(web::Data::new(use_cases.assessment_attempts.clone()))
        .app_data(web::Data::new(use_cases.assessment_submission.clone()))
        .app_data(web::Data::new(use_cases.course_assessments.clone()))
        .app_data(web::Data::new(use_cases.course_creation.clone()))
        .app_data(web::Data::new(use_cases.course_deletion.clone()))
        .app_data(web::Data::new(use_cases.course_discovery.clone()))
        .app_data(web::Data::new(use_cases.course_enrollment.clone()))
        .app_data(web::Data::new(use_cases.course_lifecycle.clone()))
        .app_data(web::Data::new(use_cases.course_organizations.clone()))
        .app_data(web::Data::new(use_cases.course_read.clone()))
        .app_data(web::Data::new(use_cases.course_role_assignment.clone()))
        .app_data(web::Data::new(use_cases.course_update.clone()))
        .app_data(web::Data::new(use_cases.learner_course_catalog.clone()))
        .app_data(web::Data::new(use_cases.learner_course_detail.clone()))
        .app_data(web::Data::new(use_cases.learner_course_learning.clone()))
        .app_data(web::Data::new(use_cases.learner_progress.clone()))
        .app_data(web::Data::new(use_cases.teacher_course_dashboard.clone()))
        .app_data(web::Data::new(
            use_cases.teacher_course_enrollment_workspace.clone(),
        ))
        .app_data(web::Data::new(use_cases.teacher_course_students.clone()))
        .app_data(web::Data::new(use_cases.teacher_course_workspace.clone()));
}
