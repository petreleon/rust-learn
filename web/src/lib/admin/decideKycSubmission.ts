import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type KycDecisionOptions } from "./KycDecisionOptions";
import { type KycSubmission } from "./KycSubmission";

export function decideKycSubmission({
  apiRoot = "/api",
  rejectionReason,
  status,
  submissionId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: KycDecisionOptions): Promise<KycSubmission> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({
      rejection_reason: status === "rejected" ? rejectionReason?.trim() || null : null,
      status,
    }),
    method: "PUT",
    path: `/kyc/review/${submissionId}`,
    timeoutMs,
    token,
  });
}
