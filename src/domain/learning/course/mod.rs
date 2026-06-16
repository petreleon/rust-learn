mod catalog_visibility;
pub mod status;

pub use catalog_visibility::is_generated_course_title;
pub use status::{
    CourseLifecycleStatus, CourseLifecycleStatusParseError, COURSE_STATUS_APPROVED,
    COURSE_STATUS_ARCHIVED, COURSE_STATUS_DRAFT, COURSE_STATUS_NEEDS_CHANGES,
    COURSE_STATUS_PUBLISHED, COURSE_STATUS_SUBMITTED, COURSE_STATUS_SUSPENDED,
};
