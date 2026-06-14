pub mod status;

pub use status::{
    CourseJoinRequestStatus, CourseJoinRequestStatusParseError, COURSE_JOIN_STATUS_APPROVED,
    COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_REJECTED, COURSE_JOIN_STATUS_WAITLISTED,
};
