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

    // Wallets and payments
    CREATE_WALLET,
    VIEW_WALLET,
    MANAGE_WALLETS,
    TRANSFER_MONEY,
    DEPOSIT_MONEY,
    WITHDRAW_MONEY,
    VIEW_TRANSACTIONS,
    CENTRALIZED_WALLET_TRANSFER,

    // Platform / Admin
    MANAGE_PLATFORM_SETTINGS,
    MANAGE_API_KEYS,
    MANAGE_INTEGRATIONS,
    VIEW_AUDIT_LOGS,
    IMPERSONATE_USER,
    MANAGE_BILLING,
    EXPORT_DATA,
    RUN_MAINTENANCE_TASKS,

    // Organization management (medium priority)
    MANAGE_ORG_MEMBERS,
    MANAGE_ORG_SETTINGS,
    MANAGE_ORG_BILLING,
    MANAGE_ORG_WALLETS,

    // Courses & Content (medium priority)
    MANAGE_COURSE_ENROLLMENTS,
    APPROVE_COURSE_CONTENT,
    PUBLISH_CONTENT,
    MANAGE_COURSE_SETTINGS,
    CREATE_COURSE_TEMPLATES,
    MANAGE_ASSESSMENT_TEMPLATES,

    // Roles & Permissions
    MANAGE_ROLE_PERMISSIONS,
    VIEW_ROLE_ASSIGNMENTS,
    ASSIGN_ROLES_TO_ORG_USERS,

    // Wallet / Transactions (expanded)
    INITIATE_EXTERNAL_TRANSFER,
    APPROVE_CENTRALIZED_TRANSFER,
    REFUND_TRANSACTION,
    RECONCILE_WALLETS,
    MANAGE_PAYMENT_METHODS,
    VIEW_FINANCIAL_REPORTS,

    // Transactions & Security
    VIEW_SENSITIVE_TRANSACTIONS,
    EXPORT_TRANSACTIONS,
    MANAGE_COMPLIANCE_FLAGS,

    // Moderation & Community
    MANAGE_DISCUSSIONS,
    SUSPEND_USER,
    BAN_USER,

    // Notifications & templates
    MANAGE_NOTIFICATION_TEMPLATES,
    SEND_BULK_NOTIFICATION,

    // Integrations / Blockchain / Storage
    MANAGE_SMART_CONTRACTS,
    MANAGE_MINIO_OBJECTS,
    MANAGE_ETHEREUM_ACCOUNTS,

    // Data & Reporting
    VIEW_ANALYTICS_DASHBOARD,
    MANAGE_EXPORT_JOBS,

    // Tests / QA
    RUN_TESTS,
    VIEW_TEST_RESULTS,
    MANAGE_TEST_SUITES,
}
