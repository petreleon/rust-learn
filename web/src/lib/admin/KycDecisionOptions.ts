import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type KycDecisionStatus } from "./KycDecisionStatus";

export type KycDecisionOptions = AdminRequestOptions & {
  rejectionReason?: string;
  status: KycDecisionStatus;
  submissionId: number;
};
