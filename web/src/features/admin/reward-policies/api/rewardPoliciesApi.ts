import { createRewardPolicy } from "@/lib/admin/createRewardPolicy";
import { fetchRewardPolicies } from "@/lib/admin/fetchRewardPolicies";
import { type RewardPolicyCreatePayload } from "@/lib/admin/RewardPolicyCreateOptions";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import {
  ADMIN_REWARD_POLICY_PAGE_SIZE,
  type RewardPolicyFilters,
} from "../model/RewardPolicyFilters";

export function loadAdminRewardPolicySession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function loadRewardPolicyItems({
  filters,
  token,
}: {
  filters: RewardPolicyFilters;
  token: string;
}): Promise<RewardPolicyItem[]> {
  return fetchRewardPolicies({
    active: filters.active,
    event_type: filters.eventType || null,
    limit: ADMIN_REWARD_POLICY_PAGE_SIZE,
    offset: filters.offset,
    scope_type: filters.scopeType || null,
    token,
  });
}

export function createAdminRewardPolicy({
  payload,
  token,
}: {
  payload: RewardPolicyCreatePayload;
  token: string;
}): Promise<RewardPolicyItem> {
  return createRewardPolicy({ payload, token });
}
