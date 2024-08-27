// src/config/constants/roles.rs
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Debug, PartialEq)]
#[allow(non_camel_case_types)]
enum Roles {
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