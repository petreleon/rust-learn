use crate::domain::access_control::permissions::Permissions;
use crate::domain::access_control::permissions::Permissions::*;

pub(super) struct CapabilityDefinition {
    pub(super) key: &'static str,
    pub(super) label: &'static str,
    pub(super) permissions: &'static [Permissions],
}

pub(super) const PLATFORM: &[CapabilityDefinition] = &[
    capability("summary", "Platform summary", &[VIEW_REPORT]),
    capability(
        "users",
        "User management",
        &[VIEW_USER, ASSIGN_ROLES_TO_USER, VIEW_ROLE_ASSIGNMENTS],
    ),
    capability("kyc_reviews", "KYC review", &[REVIEW_KYC_SUBMISSIONS]),
    capability(
        "teacher_applications",
        "Teacher review",
        &[
            REVIEW_TEACHER_APPLICATIONS,
            APPROVE_TEACHER_APPLICATION,
            REJECT_TEACHER_APPLICATION,
        ],
    ),
    capability(
        "reward_amount_review",
        "Reward amount review",
        &[APPROVE_REWARD_AMOUNT, VIEW_REWARD_AUDIT],
    ),
    capability(
        "fraud_blocks",
        "Fraud controls",
        &[
            MANAGE_REWARD_FRAUD_BLOCKS,
            BLOCK_REWARD_TEACHER,
            BLOCK_REWARD_ORGANIZATION,
        ],
    ),
    capability(
        "delegations",
        "Delegations",
        &[
            DELEGATE_REWARD_APPROVAL,
            MANAGE_ROLE_PERMISSIONS,
            VIEW_ROLE_ASSIGNMENTS,
        ],
    ),
    capability("exports", "Exports", &[EXPORT_DATA]),
    capability(
        "wallets",
        "Wallet audit",
        &[
            MANAGE_WALLETS,
            VIEW_TRANSACTIONS,
            VIEW_SENSITIVE_TRANSACTIONS,
        ],
    ),
    capability(
        "system",
        "System status",
        &[
            VIEW_REPORT,
            VIEW_AUDIT_LOGS,
            VIEW_ANALYTICS_DASHBOARD,
            MANAGE_TEST_SUITES,
        ],
    ),
];

pub(super) const ORGANIZATION: &[CapabilityDefinition] = &[
    capability("courses", "Courses", &[VIEW_ORGANIZATION]),
    capability("members", "Members", &[VIEW_ORGANIZATION]),
    capability(
        "reports",
        "Reports",
        &[VIEW_REPORT, GENERATE_REPORT, VIEW_ORG_REWARD_REPORTS],
    ),
    capability(
        "member_management",
        "Member management",
        &[
            ASSIGN_ROLES_TO_ORG_USERS,
            INVITE_USER_TO_ORGANIZATION,
            MANAGE_ORG_MEMBERS,
        ],
    ),
    capability(
        "wallet",
        "Wallet",
        &[
            MANAGE_ORG_BILLING,
            MANAGE_ORG_REWARD_BUDGET,
            MANAGE_ORG_WALLETS,
        ],
    ),
    capability(
        "teacher_applications",
        "Teacher nominations",
        &[
            NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
            VIEW_ORG_TEACHER_APPLICATIONS,
        ],
    ),
    capability(
        "course_rewards",
        "Course rewards",
        &[SUBMIT_ORG_COURSE_REWARD_EVENT],
    ),
    capability(
        "settings",
        "Settings",
        &[MANAGE_ORG_SETTINGS, VIEW_ORGANIZATION],
    ),
];

pub(super) const COURSE: &[CapabilityDefinition] = &[
    capability(
        "teaching",
        "Teaching",
        &[
            APPROVE_COURSE_CONTENT,
            APPROVE_COURSE_JOIN_REQUESTS,
            APPROVE_STUDENT_REWARD_CANDIDATE,
            CREATE_CONTENT,
            DELETE_CONTENT,
            MANAGE_ASSESSMENT_TEMPLATES,
            MANAGE_COURSE_ENROLLMENTS,
            MANAGE_COURSE_REWARD_RULES,
            MANAGE_COURSE_SETTINGS,
            MODIFY_CONTENT,
            SUBMIT_COURSE_REWARD_EVENT,
        ],
    ),
    capability(
        "reward_status",
        "Reward status",
        &[VIEW_COURSE_REWARD_STATUS],
    ),
];

const fn capability(
    key: &'static str,
    label: &'static str,
    permissions: &'static [Permissions],
) -> CapabilityDefinition {
    CapabilityDefinition {
        key,
        label,
        permissions,
    }
}
