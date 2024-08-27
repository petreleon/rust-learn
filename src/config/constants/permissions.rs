// src/config/constants/permissions.rs
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Permissions {
    // Organization-related permissions
    CREATE_ORGANIZATION,
    MODIFY_ORGANIZATION,
    DELETE_ORGANIZATION,
    VIEW_ORGANIZATION,
    REQUEST_JOIN_ORGANIZATION,
    ACCEPT_ORGANIZATION_JOIN_REQUEST,

    // Course-related permissions
    CREATE_COURSE,
    MODIFY_COURSE,
    DELETE_COURSE,
    VIEW_COURSE,
    JOIN_COURSE,
    REQUEST_JOIN_COURSE,
    APPROVE_COURSE_JOIN_REQUESTS,

    // User management
    MODIFY_USER,
    DELETE_USER,
    VIEW_USER,
    INVITE_USER_TO_ORGANIZATION,

    // Role and permissions management
    MODIFY_ROLE,
    DELETE_ROLE,
    ASSIGN_ROLES_TO_USER,
    MODIFY_PERMISSION,

    // Content management
    CREATE_CONTENT,
    MODIFY_CONTENT,
    DELETE_CONTENT,
    VIEW_CONTENT,

    // Assessment and evaluations
    CREATE_ASSESSMENT,
    MODIFY_ASSESSMENT,
    DELETE_ASSESSMENT,
    GRADE_ASSESSMENT,
    VIEW_ASSESSMENT,

    // Discussions and forums
    POST_IN_DISCUSSION,
    MODERATE_DISCUSSION,

    // Notifications
    SEND_NOTIFICATION,
    VIEW_NOTIFICATION,

    // Analytics and reports
    VIEW_REPORT,
    GENERATE_REPORT,
}
