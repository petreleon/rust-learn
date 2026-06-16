"use client";

import { useState } from "react";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { normalizeRouteError } from "@/features/admin/shared/route-kit/normalizeRouteError";
import { createAdminRewardPolicy } from "../api/rewardPoliciesApi";
import {
  defaultRewardPolicyDraft,
  rewardPolicyPayload,
  validateRewardPolicyDraft,
  type RewardPolicyDraft,
} from "../model/RewardPolicyDraft";

export function useRewardPolicyCreation({
  canManage,
  onCreated,
}: {
  canManage: boolean;
  onCreated: () => void;
}) {
  const [createdPolicy, setCreatedPolicy] = useState<RewardPolicyItem | null>(null);
  const [draft, setDraft] = useState<RewardPolicyDraft>(defaultRewardPolicyDraft);
  const [error, setError] = useState<RouteError | null>(null);
  const [state, setState] = useState<LoadState>("idle");

  async function createPolicy() {
    const token = readBrowserSessionToken();
    if (!token || !canManage) return;
    const validationMessage = validateRewardPolicyDraft(draft);
    if (validationMessage) {
      setError({ code: "invalid_reward_policy", message: validationMessage, status: 400 });
      setState("error");
      return;
    }
    setError(null);
    setState("loading");
    try {
      const created = await createAdminRewardPolicy({
        payload: rewardPolicyPayload(draft),
        token,
      });
      setCreatedPolicy(created);
      setDraft(defaultRewardPolicyDraft);
      setState("success");
      onCreated();
    } catch (nextError) {
      setError(normalizeRouteError(nextError, "Reward policy could not be created."));
      setState("error");
    }
  }

  function updateDraft(patch: Partial<RewardPolicyDraft>) {
    setDraft((current) => ({ ...current, ...patch }));
  }

  return {
    createPolicy,
    createdPolicy,
    createError: error,
    createState: state,
    draft,
    updateDraft,
  };
}
