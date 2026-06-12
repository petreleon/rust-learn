import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type KycReviewQueueResponse } from "./KycReviewQueueResponse";

export function fetchKycReviewQueue(options: AdminRequestOptions): Promise<KycReviewQueueResponse> {
  return adminJsonRequest({
    path: "/kyc/review",
    ...options,
  });
}
