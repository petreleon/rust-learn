import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type KycAuditEvent } from "./KycAuditEvent";

export async function fetchKycSubmissionAudit({
  apiRoot = "/api",
  submissionId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { submissionId: number }): Promise<KycAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/kyc/review/${submissionId}/audit`,
    timeoutMs,
    token,
  });
}
