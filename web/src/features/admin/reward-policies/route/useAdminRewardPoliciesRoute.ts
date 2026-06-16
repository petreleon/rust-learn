"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace } from "@/lib/admin/buildPlatformAdminWorkspace";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { normalizeRouteError } from "@/features/admin/shared/route-kit/normalizeRouteError";
import { loadAdminRewardPolicySession } from "../api/rewardPoliciesApi";
import {
  canManageRewardPolicies,
  emptyRewardPolicyWorkspace,
  findRewardPolicyCapability,
} from "../model/rewardPolicyAccessModel";
import { useRewardPolicyCreation } from "./useRewardPolicyCreation";
import { useRewardPolicyActivation } from "./useRewardPolicyActivation";
import { useRewardPolicyAudit } from "./useRewardPolicyAudit";
import { useRewardPolicyList } from "./useRewardPolicyList";

function expiredSession(error: RouteError) {
  return error.status === 401 || error.status === 404;
}

export function useAdminRewardPoliciesRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const workspace = useMemo(
    () => (session ? buildPlatformAdminWorkspace(session) : emptyRewardPolicyWorkspace),
    [session],
  );
  const allowed = session ? hasPlatformAdminAccess(session) : false;
  const canManage = canManageRewardPolicies(workspace);
  const rewardPolicyCapability = findRewardPolicyCapability(workspace);
  const list = useRewardPolicyList({ canManage });
  const { loadPolicies } = list;
  const create = useRewardPolicyCreation({ canManage, onCreated: loadPolicies });
  const audit = useRewardPolicyAudit({ canManage });
  const { loadPolicyAudit, resetPolicyAudit } = audit;
  const activation = useRewardPolicyActivation({
    canManage,
    onUpdated: (policy) => {
      void loadPolicies();
      void loadPolicyAudit(policy.id);
    },
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
      setSession(await loadAdminRewardPolicySession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError, "Platform admin session could not be loaded.");
      if (expiredSession(routeError)) {
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
    if (session && allowed && canManage) {
      const timeout = window.setTimeout(() => void loadPolicies(), 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [allowed, canManage, loadPolicies, session]);

  useEffect(() => {
    if (session && allowed && canManage && list.selectedPolicyId) {
      const timeout = window.setTimeout(() => void loadPolicyAudit(list.selectedPolicyId), 0);
      return () => window.clearTimeout(timeout);
    }
    resetPolicyAudit();
    return undefined;
  }, [allowed, canManage, list.selectedPolicyId, loadPolicyAudit, resetPolicyAudit, session]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    allowed,
    canManage,
    error,
    hasToken,
    loadSession,
    loadState,
    rewardPolicyCapability,
    session,
    signOut,
    workspace,
    ...list,
    ...create,
    ...audit,
    ...activation,
  };
}

export type AdminRewardPoliciesRouteController = ReturnType<typeof useAdminRewardPoliciesRoute>;
