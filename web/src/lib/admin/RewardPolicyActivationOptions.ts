import { type AdminRequestOptions } from "./AdminRequestOptions";

export type RewardPolicyActivationOptions = AdminRequestOptions & {
  active: boolean;
  policyId: number;
};
