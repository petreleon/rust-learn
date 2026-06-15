// src/domain/access_control/roles.rs
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Roles {
    SUPER_ADMIN,
    ADMIN,
    MODERATOR,
    USER,
    GUEST,
    TEACHER,
    TEACHER_COURSE,
    STUDENT,
    STUDENT_COURSE,
}
