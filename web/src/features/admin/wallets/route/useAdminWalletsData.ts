"use client";

import { useCallback, useState } from "react";
import { type PlatformReportSummary } from "@/lib/admin/PlatformReportSummary";
import { type PlatformWalletReconciliation } from "@/lib/admin/PlatformWalletReconciliation";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminWalletReconciliation, loadAdminWalletSummary } from "../api/walletsApi";
import { normalizeAdminWalletRouteError } from "./normalizeAdminWalletRouteError";

export function useAdminWalletsData({
  allowed,
  canView,
  session,
}: {
  allowed: boolean;
  canView: boolean;
  session: CurrentSession | null;
}) {
  const [reconciliation, setReconciliation] = useState<PlatformWalletReconciliation | null>(null);
  const [reconciliationError, setReconciliationError] = useState<RouteError | null>(null);
  const [reconciliationState, setReconciliationState] = useState<LoadState>("idle");
  const [summary, setSummary] = useState<PlatformReportSummary | null>(null);
  const [summaryError, setSummaryError] = useState<RouteError | null>(null);
  const [summaryState, setSummaryState] = useState<LoadState>("idle");

  const loadSummary = useCallback(async (token: string) => {
    setSummaryState("loading");
    setSummaryError(null);

    try {
      setSummary(await loadAdminWalletSummary({ token }));
      setSummaryState("success");
    } catch (nextError) {
      setSummary(null);
      setSummaryError(normalizeAdminWalletRouteError(nextError, "Platform summary could not be loaded."));
      setSummaryState("error");
    }
  }, []);

  const loadReconciliation = useCallback(async (token: string) => {
    setReconciliationState("loading");
    setReconciliationError(null);

    try {
      setReconciliation(await loadAdminWalletReconciliation({ token }));
      setReconciliationState("success");
    } catch (nextError) {
      setReconciliation(null);
      setReconciliationError(
        normalizeAdminWalletRouteError(nextError, "Wallet reconciliation could not be loaded."),
      );
      setReconciliationState("error");
    }
  }, []);

  const loadWalletAudit = useCallback(() => {
    const token = readBrowserSessionToken();
    if (!session || !token || !allowed || !canView) return;

    void loadSummary(token);
    void loadReconciliation(token);
  }, [allowed, canView, loadReconciliation, loadSummary, session]);

  return {
    loadReconciliation,
    loadSummary,
    loadWalletAudit,
    reconciliation,
    reconciliationError,
    reconciliationState,
    summary,
    summaryError,
    summaryState,
  };
}
