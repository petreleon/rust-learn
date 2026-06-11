import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type FraudBlockAuditEvent } from "./FraudBlockAuditEvent";

export async function fetchFraudBlockAudit({
  apiRoot = "/api",
  blockId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { blockId: number }): Promise<FraudBlockAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/reward-fraud-blocks/${blockId}/audit`,
    timeoutMs,
    token,
  });
}
