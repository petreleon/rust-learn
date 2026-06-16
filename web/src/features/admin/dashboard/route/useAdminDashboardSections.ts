"use client";

import { useCallback, useState } from "react";
import { type PlatformFraudDashboard } from "@/lib/admin/PlatformFraudDashboard";
import { type PlatformReportSummary } from "@/lib/admin/PlatformReportSummary";
import { type PlatformRewardDashboard } from "@/lib/admin/PlatformRewardDashboard";
import { type PlatformSystemStatus } from "@/lib/admin/PlatformSystemStatus";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadPlatformFraudControls,
  loadPlatformRewardOperations,
  loadPlatformSummary,
  loadPlatformSystemStatus,
} from "../api/dashboardApi";
import { normalizeAdminDashboardRouteError } from "./normalizeAdminDashboardRouteError";

export function useAdminDashboardSections({
  allowed,
  canViewRewardOperations,
  canViewSummary,
  session,
}: {
  allowed: boolean;
  canViewRewardOperations: boolean;
  canViewSummary: boolean;
  session: CurrentSession | null;
}) {
  const [fraudDashboard, setFraudDashboard] = useState<PlatformFraudDashboard | null>(null);
  const [fraudError, setFraudError] = useState<RouteError | null>(null);
  const [fraudState, setFraudState] = useState<LoadState>("idle");
  const [rewardDashboard, setRewardDashboard] = useState<PlatformRewardDashboard | null>(null);
  const [rewardError, setRewardError] = useState<RouteError | null>(null);
  const [rewardState, setRewardState] = useState<LoadState>("idle");
  const [summary, setSummary] = useState<PlatformReportSummary | null>(null);
  const [summaryError, setSummaryError] = useState<RouteError | null>(null);
  const [summaryState, setSummaryState] = useState<LoadState>("idle");
  const [systemError, setSystemError] = useState<RouteError | null>(null);
  const [systemState, setSystemState] = useState<LoadState>("idle");
  const [systemStatus, setSystemStatus] = useState<PlatformSystemStatus | null>(null);

  const loadSummary = useCallback(
    async (token: string) => {
      if (!canViewSummary) {
        setSummary(null);
        setSummaryError(null);
        setSummaryState("idle");
        return;
      }

      setSummaryError(null);
      setSummaryState("loading");

      try {
        setSummary(await loadPlatformSummary({ token }));
        setSummaryState("success");
      } catch (nextError) {
        setSummary(null);
        setSummaryError(
          normalizeAdminDashboardRouteError(nextError, "Platform summary could not be loaded."),
        );
        setSummaryState("error");
      }
    },
    [canViewSummary],
  );

  const loadRewardAndFraud = useCallback(
    (token: string) => {
      if (!canViewRewardOperations) {
        setRewardDashboard(null);
        setRewardError(null);
        setRewardState("idle");
        setFraudDashboard(null);
        setFraudError(null);
        setFraudState("idle");
        return;
      }

      setRewardError(null);
      setRewardState("loading");
      loadPlatformRewardOperations({ token })
        .then((nextDashboard) => {
          setRewardDashboard(nextDashboard);
          setRewardState("success");
        })
        .catch((nextError) => {
          setRewardDashboard(null);
          setRewardError(
            normalizeAdminDashboardRouteError(nextError, "Reward operations could not be loaded."),
          );
          setRewardState("error");
        });

      setFraudError(null);
      setFraudState("loading");
      loadPlatformFraudControls({ token })
        .then((nextDashboard) => {
          setFraudDashboard(nextDashboard);
          setFraudState("success");
        })
        .catch((nextError) => {
          setFraudDashboard(null);
          setFraudError(
            normalizeAdminDashboardRouteError(nextError, "Fraud controls could not be loaded."),
          );
          setFraudState("error");
        });
    },
    [canViewRewardOperations],
  );

  const loadSystemStatus = useCallback(async () => {
    setSystemError(null);
    setSystemState("loading");

    try {
      setSystemStatus(await loadPlatformSystemStatus());
      setSystemState("success");
    } catch (nextError) {
      setSystemStatus(null);
      setSystemError(normalizeAdminDashboardRouteError(nextError, "System status could not be loaded."));
      setSystemState("error");
    }
  }, []);

  const loadDashboard = useCallback(() => {
    const token = readBrowserSessionToken();
    if (!session || !token || !allowed) return;

    void loadSummary(token);
    loadRewardAndFraud(token);
    void loadSystemStatus();
  }, [allowed, loadRewardAndFraud, loadSummary, loadSystemStatus, session]);

  return {
    fraudDashboard,
    fraudError,
    fraudState,
    loadDashboard,
    rewardDashboard,
    rewardError,
    rewardState,
    summary,
    summaryError,
    summaryState,
    systemError,
    systemState,
    systemStatus,
  };
}
