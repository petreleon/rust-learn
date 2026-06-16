"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadSessionWorkspace } from "../api/sessionWorkspaceApi";
import { type LoadState } from "../model/LoadState";
import { type RouteError } from "../model/RouteError";
import { sessionWorkspaceCount } from "../model/sessionWorkspaceCount";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { normalizeSessionWorkspaceRouteError } from "./normalizeSessionWorkspaceRouteError";

export function useSessionWorkspaceRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setError(null);
    setHasToken(false);
    setLoadState(nextLoadState);
    setSession(null);
  }, []);

  const loadSession = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) {
      clearRoute("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      setSession(await loadSessionWorkspace({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeSessionWorkspaceRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearBrowserSession();
        setHasToken(false);
      }
      setError(routeError);
      setLoadState("error");
      setSession(null);
    }
  }, [clearRoute]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
    workspaceCount: useMemo(() => sessionWorkspaceCount(session), [session]),
  };
}

export type SessionWorkspaceRouteController = ReturnType<typeof useSessionWorkspaceRoute>;
