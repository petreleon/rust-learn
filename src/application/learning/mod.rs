pub mod assessment;
pub mod assign_course_role;
pub mod course_enrollment;
pub mod create_course;
pub mod delete_course;
pub mod discover_courses;
pub mod get_course;
pub mod get_learner_course_detail;
pub mod get_learner_course_learning;
pub mod get_teacher_course_enrollment_workspace;
pub mod get_teacher_course_students;
pub mod get_teacher_course_workspace;
pub mod learner_course_catalog;
pub mod learner_progress;
pub mod list_assessment_attempts;
pub mod list_course_assessments;
pub mod list_course_organizations;
pub mod list_learner_course_catalog;
pub mod list_teacher_course_dashboard;
pub mod manage_assessments;
pub mod ports;
pub mod submit_assessment_attempt;
pub mod teacher_course_dashboard;
pub mod teacher_course_enrollment;
pub mod update_course;
pub mod update_course_lifecycle;

#[cfg(test)]
mod tests;
