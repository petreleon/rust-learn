"use client";

import { useCallback, useEffect, useState } from "react";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";
import { normalizeRouteError } from "./normalizeRouteError";

export function useOrganizationSession() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const loadSession = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setError(null);
      setHasToken(false);
      setLoadState("idle");
      setSession(null);
      return;
    }

    setError(null);
    setHasToken(true);
    setLoadState("loading");

    try {
      const nextSession = await fetchCurrentSession({ token });
      setSession(nextSession);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setError(routeError);
      setLoadState("error");
      setSession(null);
    }
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  function signOut() {
    clearStoredSessionToken();
    setError(null);
    setHasToken(false);
    setLoadState("idle");
    setSession(null);
  }

  return {
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
  };
}
