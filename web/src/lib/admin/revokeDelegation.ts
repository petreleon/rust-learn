import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type DelegationItem } from "./DelegationItem";

export async function revokeDelegation({
  apiRoot = "/api",
  delegationId,
  revokeReason,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { delegationId: number; revokeReason?: string | null }): Promise<DelegationItem> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({ revoke_reason: revokeReason?.trim() || null }),
    method: "PUT",
    path: `/delegated-permissions/${delegationId}/revoke`,
    timeoutMs,
    token,
  });
}
