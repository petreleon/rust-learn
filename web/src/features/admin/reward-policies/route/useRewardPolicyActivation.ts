"use client";

import { useState } from "react";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { normalizeRouteError } from "@/features/admin/shared/route-kit/normalizeRouteError";
import { updateAdminRewardPolicyActivation } from "../api/rewardPoliciesApi";

export function useRewardPolicyActivation({
  canManage,
  onUpdated,
}: {
  canManage: boolean;
  onUpdated: (policy: RewardPolicyItem) => void;
}) {
  const [activationError, setActivationError] = useState<RouteError | null>(null);
  const [activationState, setActivationState] = useState<LoadState>("idle");
  const [updatedPolicy, setUpdatedPolicy] = useState<RewardPolicyItem | null>(null);

  async function setPolicyActive(policyId: number, active: boolean) {
    const token = readBrowserSessionToken();
    if (!token || !canManage) return;
    setActivationError(null);
    setActivationState("loading");
    try {
      const updated = await updateAdminRewardPolicyActivation({ active, policyId, token });
      setUpdatedPolicy(updated);
      setActivationState("success");
      onUpdated(updated);
    } catch (nextError) {
      setActivationError(
        normalizeRouteError(nextError, "Reward policy activation could not be changed."),
      );
      setActivationState("error");
    }
  }

  return {
    activationError,
    activationState,
    setPolicyActive,
    updatedPolicy,
  };
}
