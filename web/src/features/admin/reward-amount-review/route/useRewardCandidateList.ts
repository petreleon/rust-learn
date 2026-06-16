"use client";

import { useCallback, useMemo, useState } from "react";
import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";
import { type PlatformRewardCandidatesResponse } from "@/lib/admin/PlatformRewardCandidatesResponse";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminRewardCandidates } from "../api/rewardAmountReviewApi";
import {
  defaultRewardCandidateFilters,
  type RewardCandidateFilters,
} from "../model/RewardCandidateFilters";
import { normalizeAdminRewardAmountRouteError } from "./normalizeAdminRewardAmountRouteError";

export function useRewardCandidateList({
  allowed,
  canView,
  session,
}: {
  allowed: boolean;
  canView: boolean;
  session: CurrentSession | null;
}) {
  const [candidateError, setCandidateError] = useState<RouteError | null>(null);
  const [candidateState, setCandidateState] = useState<LoadState>("idle");
  const [candidates, setCandidates] = useState<PlatformRewardCandidatesResponse | null>(null);
  const [filters, setFilters] = useState<RewardCandidateFilters>(defaultRewardCandidateFilters);
  const [selectedCandidateId, setSelectedCandidateId] = useState<number | null>(null);

  const selectedCandidate = useMemo(
    () => candidates?.candidates.find((item) => item.id === selectedCandidateId) || candidates?.candidates[0] || null,
    [candidates, selectedCandidateId],
  );

  const loadCandidates = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!session || !token || !allowed || !canView) return;

    setCandidateState("loading");
    setCandidateError(null);

    try {
      const response = await loadAdminRewardCandidates({
        offset: filters.offset,
        search: filters.appliedSearch,
        status: filters.status,
        token,
      });
      setCandidates(response);
      setCandidateState("success");
      setSelectedCandidateId((current) => nextSelectedCandidateId(current, response.candidates));
    } catch (nextError) {
      setCandidates(null);
      setCandidateError(
        normalizeAdminRewardAmountRouteError(
          nextError,
          "Reward candidate review queue could not be loaded.",
        ),
      );
      setCandidateState("error");
    }
  }, [allowed, canView, filters.appliedSearch, filters.offset, filters.status, session]);

  const applyFilters = useCallback(() => {
    setFilters((current) => ({
      ...current,
      appliedSearch: current.searchInput.trim(),
      offset: 0,
    }));
  }, []);

  const resetFilters = useCallback(() => {
    setFilters(defaultRewardCandidateFilters);
  }, []);

  const updateFilters = useCallback((patch: Partial<RewardCandidateFilters>) => {
    setFilters((current) => ({ ...current, ...patch }));
  }, []);

  const setPageOffset = useCallback((offset: number) => {
    setFilters((current) => ({ ...current, offset: Math.max(0, offset) }));
  }, []);

  return {
    applyFilters,
    candidateError,
    candidates,
    candidateState,
    filters,
    loadCandidates,
    resetFilters,
    selectedCandidate,
    selectedCandidateId,
    setPageOffset,
    setSelectedCandidateId,
    updateFilters,
  };
}

function nextSelectedCandidateId(current: number | null, candidates: PlatformRewardCandidateItem[]) {
  if (current && candidates.some((candidate) => candidate.id === current)) return current;
  return candidates[0]?.id || null;
}
