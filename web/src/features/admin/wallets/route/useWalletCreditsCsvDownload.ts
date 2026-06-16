"use client";

import { useState } from "react";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { downloadWalletCreditsCsv } from "../api/walletsApi";
import { type WalletCreditsCsvState } from "../model/WalletCreditsCsvState";
import { normalizeAdminWalletRouteError } from "./normalizeAdminWalletRouteError";
import { startWalletCreditsCsvDownload } from "./startWalletCreditsCsvDownload";

export function useWalletCreditsCsvDownload({ canExport }: { canExport: boolean }) {
  const [csvError, setCsvError] = useState<RouteError | null>(null);
  const [csvState, setCsvState] = useState<WalletCreditsCsvState>("idle");

  async function handleDownload() {
    const token = readBrowserSessionToken();
    if (!token || !canExport) return;

    setCsvState("downloading");
    setCsvError(null);

    try {
      const csv = await downloadWalletCreditsCsv({ token });
      startWalletCreditsCsvDownload(csv);
      setCsvState("success");
      window.setTimeout(() => setCsvState("idle"), 3000);
    } catch (nextError) {
      setCsvError(normalizeAdminWalletRouteError(nextError, "Wallet credits CSV could not be downloaded."));
      setCsvState("error");
    }
  }

  return {
    csvError,
    csvState,
    handleDownload,
  };
}
