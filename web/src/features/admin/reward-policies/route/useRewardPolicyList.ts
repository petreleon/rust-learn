"use client";

import { useCallback, useMemo, useState } from "react";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadRewardPolicyItems } from "../api/rewardPoliciesApi";
import {
  ADMIN_REWARD_POLICY_PAGE_SIZE,
  defaultRewardPolicyFilters,
  type RewardPolicyFilters,
} from "../model/RewardPolicyFilters";
import { findRewardPolicyById } from "../model/rewardPolicyInspection";
import { normalizeRouteError } from "@/features/admin/shared/route-kit/normalizeRouteError";

export function useRewardPolicyList({ canManage }: { canManage: boolean }) {
  const [error, setError] = useState<RouteError | null>(null);
  const [filters, setFilters] = useState<RewardPolicyFilters>(defaultRewardPolicyFilters);
  const [policies, setPolicies] = useState<RewardPolicyItem[]>([]);
  const [selectedPolicyId, setSelectedPolicyId] = useState<number | null>(null);
  const [state, setState] = useState<LoadState>("idle");
  const selectedPolicy = useMemo(
    () => findRewardPolicyById(policies, selectedPolicyId),
    [policies, selectedPolicyId],
  );

  const loadPolicies = useCallback(
    async (nextFilters = filters) => {
      const token = readBrowserSessionToken();
      if (!token || !canManage) return;
      setError(null);
      setState("loading");
      try {
        const nextPolicies = await loadRewardPolicyItems({ filters: nextFilters, token });
        setPolicies(nextPolicies);
        setSelectedPolicyId((current) => findRewardPolicyById(nextPolicies, current)?.id ?? nextPolicies[0]?.id ?? null);
        setState("success");
      } catch (nextError) {
        setError(normalizeRouteError(nextError, "Reward policies could not be loaded."));
        setPolicies([]);
        setSelectedPolicyId(null);
        setState("error");
      }
    },
    [canManage, filters],
  );

  function updateFilters(patch: Partial<RewardPolicyFilters>) {
    setFilters((current) => ({ ...current, ...patch }));
  }

  function applyFilters() {
    const nextFilters = { ...filters, offset: 0 };
    setFilters(nextFilters);
    void loadPolicies(nextFilters);
  }

  function resetFilters() {
    setFilters(defaultRewardPolicyFilters);
    void loadPolicies(defaultRewardPolicyFilters);
  }

  function setPageOffset(offset: number) {
    const nextFilters = { ...filters, offset: Math.max(0, offset) };
    setFilters(nextFilters);
    void loadPolicies(nextFilters);
  }

  return {
    applyFilters,
    filters,
    hasNextPage: policies.length >= ADMIN_REWARD_POLICY_PAGE_SIZE,
    loadPolicies,
    policies,
    policiesError: error,
    policiesState: state,
    resetFilters,
    selectedPolicy,
    selectedPolicyId,
    selectPolicy: setSelectedPolicyId,
    setPageOffset,
    updateFilters,
  };
}
