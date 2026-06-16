import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type AdminUserRoleAssignmentAuditEvent } from "./AdminUserRoleAssignmentAuditEvent";

export function fetchAdminUserRoleAssignmentAudit({
  apiRoot,
  token,
  userId,
}: AdminRequestOptions & { userId: number }): Promise<AdminUserRoleAssignmentAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/user/${userId}/role/audit`,
    token,
  });
}
