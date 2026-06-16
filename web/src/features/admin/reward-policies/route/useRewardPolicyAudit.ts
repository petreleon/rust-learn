"use client";

import { useCallback, useState } from "react";
import { type RewardPolicyAuditEvent } from "@/lib/admin/RewardPolicyAuditEvent";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { normalizeRouteError } from "@/features/admin/shared/route-kit/normalizeRouteError";
import { loadAdminRewardPolicyAudit } from "../api/rewardPoliciesApi";

export function useRewardPolicyAudit({ canManage }: { canManage: boolean }) {
  const [auditError, setAuditError] = useState<RouteError | null>(null);
  const [auditEvents, setAuditEvents] = useState<RewardPolicyAuditEvent[]>([]);
  const [auditPolicyId, setAuditPolicyId] = useState<number | null>(null);
  const [auditState, setAuditState] = useState<LoadState>("idle");

  const resetPolicyAudit = useCallback(() => {
    setAuditError(null);
    setAuditEvents([]);
    setAuditPolicyId(null);
    setAuditState("idle");
  }, []);

  const loadPolicyAudit = useCallback(
    async (policyId: number) => {
      const token = readBrowserSessionToken();
      if (!token || !canManage) return;
      setAuditError(null);
      setAuditPolicyId(policyId);
      setAuditState("loading");
      try {
        setAuditEvents(await loadAdminRewardPolicyAudit({ policyId, token }));
        setAuditState("success");
      } catch (nextError) {
        setAuditEvents([]);
        setAuditError(normalizeRouteError(nextError, "Reward policy audit could not be loaded."));
        setAuditState("error");
      }
    },
    [canManage],
  );

  return {
    auditError,
    auditEvents,
    auditPolicyId,
    auditState,
    loadPolicyAudit,
    resetPolicyAudit,
  };
}
