mod catalog_dashboard_builders;
mod course_discovery;
mod course_lifecycle_invites;
mod course_mutations;
mod course_permission_summaries;
mod errors;
mod learner_course_reads;
mod learner_course_types;
mod learner_enrollment;
mod learner_learning_helpers;
mod learner_metadata;
mod learner_permissions;
mod learner_progress;
mod organization_discovery;
mod query_builders;
mod query_types;
mod shared_helpers;
mod teacher_course_types;
mod teacher_delegated_scope;
mod teacher_enrollment_types;
mod teacher_join_requests;
mod teacher_reward_eligibility;
mod teacher_reward_progress;
mod teacher_roster_pages;
mod teacher_scope;
mod teacher_student_progress;
mod teacher_workspaces;

pub use course_discovery::{discover_courses, discover_teacher_course_dashboard};
pub use course_lifecycle_invites::{
    accept_course_organization_invite, create_course_organization_invite, update_course_lifecycle,
};
pub use course_mutations::{
    create_course_with_invites, create_course_with_invites_for_actor, update_course_for_actor,
};
pub use errors::{
    CourseCreationError, CourseLifecycleError, CourseLifecycleUpdateRequest, CourseUpdateError,
    LearnerCourseCatalogError, OrganizationCourseListError, TeacherCourseDashboardError,
};
pub use learner_course_reads::{
    discover_learner_course_catalog, get_learner_course_detail, get_learner_course_learning,
};
pub use learner_course_types::{
    LearnerCourseAccessSummary, LearnerCourseCatalogChapter, LearnerCourseCatalogContent,
    LearnerCourseCatalogItem, LearnerCourseCatalogOrganization, LearnerCourseCatalogTeacher,
    LearnerCourseContentSummary, LearnerCourseEnrollmentSummary, LearnerCourseLearningChapter,
    LearnerCourseLearningContent, LearnerCourseRewardSummary,
};
pub use learner_progress::{get_learner_progress, save_learner_progress};
pub use organization_discovery::discover_organization_courses;
pub use query_types::{
    CourseDiscoveryQuery, CourseDiscoveryResponse, LearnerCourseCatalogQuery,
    LearnerCourseCatalogResponse, LearnerCourseDetailResponse, LearnerCourseLearningResponse,
    OrganizationCourseListQuery, TeacherCourseDashboardQuery, TeacherCourseEnrollmentQuery,
};
pub use teacher_course_types::{
    OrganizationCourseListItem, OrganizationCourseListResponse, TeacherCourseDashboardItem,
    TeacherCourseDashboardResponse, TeacherCourseEnrollmentWorkspaceResponse,
    TeacherCourseRewardEligibilitySummary, TeacherCourseStudentProgressItem,
    TeacherCourseStudentsResponse, TeacherCourseWorkspaceResponse, TeacherStudentProgressSummary,
    TeacherStudentRewardCandidateSummary, TeacherStudentRewardEligibilitySummary,
    TeacherStudentRewardProgressSummary,
};
pub use teacher_enrollment_types::{
    OrganizationCoursePermissionSummary, TeacherCourseJoinRequestItem,
    TeacherCourseJoinRequestPage, TeacherCoursePermissionSummary, TeacherCoursePublicationSummary,
    TeacherCourseRewardQueueSummary, TeacherCourseRosterLearner, TeacherCourseRosterPage,
    TeacherCourseRosterSummary, TeacherCourseWorkspaceChapter, TeacherCourseWorkspaceContent,
    TeacherEnrollmentUserSummary,
};
pub use teacher_workspaces::{
    get_teacher_course_enrollment_workspace, get_teacher_course_students,
    get_teacher_course_workspace,
};

pub(super) const DEFAULT_COURSE_LIMIT: i64 = 25;
pub(super) const MAX_COURSE_LIMIT: i64 = 100;
pub(super) const LIKE_ESCAPE_CHAR: char = '\\';

#[cfg(test)]
mod tests;
