"use client";

import { useState } from "react";
import { type PlatformCsvReport } from "@/lib/admin/PlatformCsvReport";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { downloadAdminDashboardCsv } from "../api/dashboardApi";
import { type DashboardCsvState } from "../model/DashboardCsvState";
import { normalizeAdminDashboardRouteError } from "./normalizeAdminDashboardRouteError";
import { startDashboardCsvDownload } from "./startDashboardCsvDownload";

export function useAdminDashboardCsvDownload() {
  const [csvError, setCsvError] = useState<RouteError | null>(null);
  const [csvFilename, setCsvFilename] = useState<string | null>(null);
  const [csvState, setCsvState] = useState<DashboardCsvState>("idle");

  async function handleCsvDownload(report: PlatformCsvReport, label: string) {
    const token = readBrowserSessionToken();
    if (!token) {
      setCsvError({
        code: "missing_token",
        message: "Sign in again before downloading CSV exports.",
        status: 401,
      });
      setCsvState("error");
      return;
    }

    setCsvError(null);
    setCsvFilename(null);
    setCsvState("downloading");

    try {
      const csv = await downloadAdminDashboardCsv({ report, token });
      startDashboardCsvDownload(csv);
      setCsvFilename(csv.filename || label);
      setCsvState("success");
    } catch (nextError) {
      setCsvError(normalizeAdminDashboardRouteError(nextError, `${label} could not be exported.`));
      setCsvState("error");
    }
  }

  return {
    csvError,
    csvFilename,
    csvState,
    handleCsvDownload,
  };
}
