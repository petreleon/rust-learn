"use client";

import { useCallback, useEffect, useState } from "react";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { normalizeRouteError } from "./normalizeRouteError";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";

export function useAdminSession() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [token, setToken] = useState<string | null>(null);

  const loadSession = useCallback(async () => {
    const storedToken = readStoredSessionToken();
    if (!storedToken) {
      setError(null);
      setHasToken(false);
      setLoadState("idle");
      setSession(null);
      setToken(null);
      return;
    }

    setError(null);
    setHasToken(true);
    setLoadState("loading");
    setToken(storedToken);

    try {
      const nextSession = await fetchCurrentSession({ token: storedToken });
      setSession(nextSession);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError, "Platform admin session could not be loaded.");
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
        setToken(null);
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
    setToken(null);
  }

  return {
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
    token,
  };
}
