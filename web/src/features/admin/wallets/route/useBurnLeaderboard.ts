"use client";

import { useCallback, useEffect, useState } from "react";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { loadBurnLeaderboard } from "../api/walletsApi";
import {
  type BurnLeaderboard,
  type BurnLeaderboardScope,
  type BurnLeaderboardWindow,
} from "../model/BurnLeaderboard";
import { normalizeAdminWalletRouteError } from "./normalizeAdminWalletRouteError";

export function useBurnLeaderboard({
  allowed,
  canView,
}: {
  allowed: boolean;
  canView: boolean;
}) {
  const [leaderboard, setLeaderboard] = useState<BurnLeaderboard | null>(null);
  const [scope, setScope] = useState<BurnLeaderboardScope>("all");
  const [state, setState] = useState<LoadState>("idle");
  const [error, setError] = useState<RouteError | null>(null);
  const [windowRange, setWindowRange] = useState<BurnLeaderboardWindow>("7d");

  const load = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!allowed || !canView || !token) return;
    setState("loading");
    setError(null);
    try {
      setLeaderboard(await loadBurnLeaderboard({ scope, token, window: windowRange }));
      setState("success");
    } catch (nextError) {
      setLeaderboard(null);
      setError(normalizeAdminWalletRouteError(nextError, "Burn leaderboard could not be loaded."));
      setState("error");
    }
  }, [allowed, canView, scope, windowRange]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void load(), 0);
    return () => window.clearTimeout(timeout);
  }, [load]);

  return {
    burnLeaderboard: leaderboard,
    burnLeaderboardError: error,
    burnLeaderboardScope: scope,
    burnLeaderboardState: state,
    burnLeaderboardWindow: windowRange,
    loadBurnLeaderboard: load,
    setBurnLeaderboardScope: setScope,
    setBurnLeaderboardWindow: setWindowRange,
  };
}
