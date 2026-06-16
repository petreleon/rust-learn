"use client";

import { useCallback, useState } from "react";
import { type RewardAuditEvent } from "@/lib/admin/RewardAuditEvent";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminRewardCandidateAudit } from "../api/rewardAmountReviewApi";
import { normalizeAdminRewardAmountRouteError } from "./normalizeAdminRewardAmountRouteError";

export function useRewardCandidateAudit({ canView }: { canView: boolean }) {
  const [auditError, setAuditError] = useState<RouteError | null>(null);
  const [auditEvents, setAuditEvents] = useState<RewardAuditEvent[]>([]);
  const [auditState, setAuditState] = useState<LoadState>("idle");

  const resetAudit = useCallback(() => {
    setAuditError(null);
    setAuditEvents([]);
    setAuditState("idle");
  }, []);

  const loadAudit = useCallback(
    async (candidateId: number) => {
      const token = readBrowserSessionToken();
      if (!token || !canView) return;

      setAuditState("loading");
      setAuditError(null);

      try {
        setAuditEvents(await loadAdminRewardCandidateAudit({ candidateId, token }));
        setAuditState("success");
      } catch (nextError) {
        setAuditEvents([]);
        setAuditError(
          normalizeAdminRewardAmountRouteError(nextError, "Reward candidate audit could not be loaded."),
        );
        setAuditState("error");
      }
    },
    [canView],
  );

  return {
    auditError,
    auditEvents,
    auditState,
    loadAudit,
    resetAudit,
  };
}
