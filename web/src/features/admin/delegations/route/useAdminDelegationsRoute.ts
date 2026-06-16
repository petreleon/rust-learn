"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace } from "@/lib/admin/buildPlatformAdminWorkspace";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminDelegationSession } from "../api/delegationsApi";
import {
  canGrantDelegations,
  canRevokeDelegations,
  canViewDelegations,
  emptyDelegationWorkspace,
  findDelegationCapability,
} from "../model/delegationAccessModel";
import { normalizeAdminDelegationRouteError } from "./normalizeAdminDelegationRouteError";
import { useAdminDelegationActions } from "./useAdminDelegationActions";
import { useAdminDelegationList } from "./useAdminDelegationList";

function isExpiredSession(error: RouteError) {
  return error.status === 401 || error.status === 404;
}

export function useAdminDelegationsRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const workspace = useMemo(
    () => (session ? buildPlatformAdminWorkspace(session) : emptyDelegationWorkspace),
    [session],
  );
  const allowed = session ? hasPlatformAdminAccess(session) : false;
  const canGrant = canGrantDelegations(workspace);
  const canRevoke = canRevokeDelegations(workspace);
  const canView = canViewDelegations(workspace);
  const delegationCapability = findDelegationCapability(workspace);
  const list = useAdminDelegationList({ allowed, canView, session });
  const { loadDelegations, prependDelegation, replaceDelegation } = list;
  const actions = useAdminDelegationActions({
    canGrant,
    canRevoke,
    onCreated: prependDelegation,
    onRevoked: replaceDelegation,
  });

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

    setError(null);
    setHasToken(true);
    setLoadState("loading");

    try {
      setSession(await loadAdminDelegationSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeAdminDelegationRouteError(
        nextError,
        "Platform admin session could not be loaded.",
      );
      if (isExpiredSession(routeError)) {
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

  useEffect(() => {
    if (session && allowed && canView) {
      const timeout = window.setTimeout(() => void loadDelegations(), 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [allowed, canView, loadDelegations, session]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    allowed,
    canGrant,
    canRevoke,
    canView,
    delegationCapability,
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
    workspace,
    ...list,
    ...actions,
  };
}

export type AdminDelegationsRouteController = ReturnType<typeof useAdminDelegationsRoute>;
