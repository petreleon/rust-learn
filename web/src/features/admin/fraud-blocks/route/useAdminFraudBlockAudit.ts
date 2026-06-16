"use client";

import { useCallback, useState } from "react";
import { type FraudBlockAuditEvent } from "@/lib/admin/FraudBlockAuditEvent";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminFraudBlockAudit } from "../api/fraudBlocksApi";
import { normalizeAdminFraudBlockRouteError } from "./normalizeAdminFraudBlockRouteError";

export function useAdminFraudBlockAudit({ canView }: { canView: boolean }) {
  const [auditError, setAuditError] = useState<RouteError | null>(null);
  const [auditEvents, setAuditEvents] = useState<FraudBlockAuditEvent[]>([]);
  const [auditState, setAuditState] = useState<LoadState>("idle");

  const resetAudit = useCallback(() => {
    setAuditError(null);
    setAuditEvents([]);
    setAuditState("idle");
  }, []);

  const loadAudit = useCallback(
    async (blockId: number) => {
      const token = readBrowserSessionToken();
      if (!token || !canView) return;

      setAuditState("loading");
      setAuditError(null);

      try {
        setAuditEvents(await loadAdminFraudBlockAudit({ blockId, token }));
        setAuditState("success");
      } catch (nextError) {
        setAuditEvents([]);
        setAuditError(
          normalizeAdminFraudBlockRouteError(nextError, "Fraud block audit could not be loaded."),
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
