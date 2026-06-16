import { createDelegation } from "@/lib/admin/createDelegation";
import { fetchDelegations } from "@/lib/admin/fetchDelegations";
import { revokeDelegation } from "@/lib/admin/revokeDelegation";
import { type DelegationItem } from "@/lib/admin/DelegationItem";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";

export type DelegationListResult = {
  delegations: DelegationItem[];
  limit: number;
  offset: number;
  total: number;
};

export function loadAdminDelegationSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function loadAdminDelegations({ token }: { token: string }): Promise<DelegationListResult> {
  return fetchDelegations({ limit: 100, token });
}

export function grantAdminDelegation({
  courseId,
  expiresAt,
  granteeUserId,
  organizationId,
  permission,
  reason,
  scopeType,
  token,
}: {
  courseId: string;
  expiresAt: string;
  granteeUserId: string;
  organizationId: string;
  permission: string;
  reason: string;
  scopeType: string;
  token: string;
}): Promise<DelegationItem> {
  return createDelegation({
    course_id: courseId ? Number(courseId) : null,
    expires_at: expiresAt.trim() || null,
    grantee_user_id: Number(granteeUserId),
    organization_id: organizationId ? Number(organizationId) : null,
    permission,
    reason: reason.trim() || null,
    scope_type: scopeType,
    token,
  });
}

export function revokeAdminDelegation({
  delegationId,
  revokeReason,
  token,
}: {
  delegationId: number;
  revokeReason: string;
  token: string;
}): Promise<DelegationItem> {
  return revokeDelegation({
    delegationId,
    revokeReason: revokeReason.trim() || null,
    token,
  });
}
