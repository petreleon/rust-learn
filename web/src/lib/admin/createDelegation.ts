import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type DelegationCreateOptions } from "./DelegationCreateOptions";
import { type DelegationItem } from "./DelegationItem";

export async function createDelegation({
  apiRoot = "/api",
  course_id,
  expires_at,
  grantee_user_id,
  organization_id,
  permission,
  reason,
  scope_type,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: DelegationCreateOptions): Promise<DelegationItem> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({
      course_id: course_id ?? null,
      expires_at: expires_at ?? null,
      grantee_user_id,
      organization_id: organization_id ?? null,
      permission,
      reason: reason?.trim() || null,
      scope_type,
    }),
    method: "POST",
    path: "/delegated-permissions",
    timeoutMs,
    token,
  });
}
