import { type AdminRole, type AdminUserProfile, type AdminUserRoleAssignmentAuditEvent } from "@/lib/admin";
import { type CurrentSession } from "@/lib/session";

export function adminSession(permissions: string[]): CurrentSession {
  return {
    access: { learner: true, organization: false, platform_admin: permissions.length > 0, teacher: false, teacher_application: false },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [
        { enabled: permissions.includes("VIEW_USER"), key: "users", label: "User management", permissions: ["VIEW_USER", "ASSIGN_ROLES_TO_USER", "VIEW_ROLE_ASSIGNMENTS"] },
      ],
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: { email: "admin@example.com", email_verified: true, id: 1, kyc_verified: true, name: "Admin User" },
  };
}

export function profile(id: number, overrides: Partial<AdminUserProfile> = {}): AdminUserProfile {
  return {
    created_at: "2026-06-12 10:00:00",
    date_of_birth: null,
    email: "ada@example.com",
    email_verified: true,
    id,
    kyc_verified: false,
    name: "Ada Lovelace",
    platform_permissions: [],
    platform_roles: [],
    ...overrides,
  };
}

export function role(name: string): AdminRole {
  return { description: null, id: 3, name };
}

export function roleAuditEvent(roleName = "PLATFORM_ADMIN"): AdminUserRoleAssignmentAuditEvent {
  return {
    actor_user_id: 1,
    created_at: "2026-06-16T10:00:00Z",
    event_type: "role_assigned",
    id: 11,
    platform_role_id: 3,
    role_name: roleName,
    target_user_id: 7,
  };
}
