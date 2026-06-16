import { type DelegationItem } from "@/lib/admin/DelegationItem";
import { type CurrentSession } from "@/lib/session/CurrentSession";

const delegationPermissions = [
  "VIEW_ROLE_ASSIGNMENTS",
  "MANAGE_ROLE_PERMISSIONS",
  "DELEGATE_REWARD_APPROVAL",
];

export function adminDelegationSession(permissions: string[]): CurrentSession {
  return {
    access: {
      learner: true,
      organization: false,
      platform_admin: permissions.length > 0,
      teacher: false,
      teacher_application: false,
    },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [
        {
          enabled: delegationPermissions.some((permission) => permissions.includes(permission)),
          key: "delegations",
          label: "Delegations",
          permissions: delegationPermissions,
        },
      ],
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: {
      email: "admin@example.com",
      email_verified: true,
      id: 1,
      kyc_verified: true,
      name: "Admin User",
    },
  };
}

export function delegation(overrides: Partial<DelegationItem> = {}): DelegationItem {
  return {
    course_id: null,
    created_at: "2026-06-12T10:00:00Z",
    expires_at: null,
    grantee_user_id: 8,
    grantor_user_id: 1,
    id: 5,
    organization_id: null,
    permission: "APPROVE_REWARD_AMOUNT",
    reason: "Temporary review cover",
    revoked_at: null,
    revoked_by_user_id: null,
    revoke_reason: null,
    scope_type: "platform",
    updated_at: "2026-06-12T10:00:00Z",
    ...overrides,
  };
}
