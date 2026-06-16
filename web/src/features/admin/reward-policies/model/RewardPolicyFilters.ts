export type RewardPolicyFilters = {
  active: boolean | null;
  eventType: string;
  offset: number;
  scopeType: string;
};

export const ADMIN_REWARD_POLICY_PAGE_SIZE = 25;

export const defaultRewardPolicyFilters: RewardPolicyFilters = {
  active: true,
  eventType: "",
  offset: 0,
  scopeType: "",
};
