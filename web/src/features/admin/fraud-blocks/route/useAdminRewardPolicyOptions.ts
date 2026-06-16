"use client";

import { useCallback, useEffect, useState } from "react";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadActiveRewardPolicies } from "../api/fraudBlocksApi";
import {
  rewardPolicyOption,
  type RewardPolicyOption,
} from "../model/RewardPolicyOption";
import { normalizeAdminFraudBlockRouteError } from "./normalizeAdminFraudBlockRouteError";

export function useAdminRewardPolicyOptions({
  allowed,
  canListPolicies,
}: {
  allowed: boolean;
  canListPolicies: boolean;
}) {
  const [error, setError] = useState<RouteError | null>(null);
  const [options, setOptions] = useState<RewardPolicyOption[]>([]);
  const [state, setState] = useState<LoadState>("idle");

  const loadRewardPolicyOptions = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || !allowed || !canListPolicies) return;

    setError(null);
    setState("loading");
    try {
      const policies = await loadActiveRewardPolicies({ token });
      setOptions(policies.map(rewardPolicyOption));
      setState("success");
    } catch (nextError) {
      setError(normalizeAdminFraudBlockRouteError(nextError, "Reward policies could not be loaded."));
      setOptions([]);
      setState("error");
    }
  }, [allowed, canListPolicies]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRewardPolicyOptions(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRewardPolicyOptions]);

  return {
    loadRewardPolicyOptions,
    rewardPolicyOptions: options,
    rewardPolicyOptionsError: error,
    rewardPolicyOptionsState: state,
  };
}
