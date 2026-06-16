"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace } from "@/lib/admin/buildPlatformAdminWorkspace";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminFraudBlockSession } from "../api/fraudBlocksApi";
import {
  canCreateFraudBlocks,
  canListRewardPoliciesForBlocks,
  canRevokeFraudBlocks,
  canViewFraudBlocks,
  emptyFraudBlockWorkspace,
  findFraudBlockCapability,
} from "../model/fraudBlockAccessModel";
import { useAdminFraudBlockActions } from "./useAdminFraudBlockActions";
import { useAdminFraudBlockAudit } from "./useAdminFraudBlockAudit";
import { useAdminFraudBlockList } from "./useAdminFraudBlockList";
import { useAdminRewardPolicyOptions } from "./useAdminRewardPolicyOptions";
import { normalizeAdminFraudBlockRouteError } from "./normalizeAdminFraudBlockRouteError";

function isExpiredSession(error: RouteError) {
  return error.status === 401 || error.status === 404;
}

export function useAdminFraudBlocksRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const workspace = useMemo(
    () => (session ? buildPlatformAdminWorkspace(session) : emptyFraudBlockWorkspace),
    [session],
  );
  const allowed = session ? hasPlatformAdminAccess(session) : false;
  const canCreate = canCreateFraudBlocks(workspace);
  const canListRewardPolicies = canListRewardPoliciesForBlocks(workspace);
  const canRevoke = canRevokeFraudBlocks(workspace);
  const canView = canViewFraudBlocks(workspace);
  const fraudBlockCapability = findFraudBlockCapability(workspace);
  const list = useAdminFraudBlockList({ allowed, canView, session });
  const rewardPolicyOptions = useAdminRewardPolicyOptions({ allowed, canListPolicies: canListRewardPolicies });
  const audit = useAdminFraudBlockAudit({ canView });
  const { loadAudit, resetAudit } = audit;
  const { loadBlocks, selectedBlock, selectedBlockId } = list;
  const actions = useAdminFraudBlockActions({
    canCreate,
    canRevoke,
    onBlocksChanged: loadBlocks,
    onSelectedRevoked: loadAudit,
    selectedBlockId,
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
      setSession(await loadAdminFraudBlockSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeAdminFraudBlockRouteError(
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
      const timeout = window.setTimeout(() => void loadBlocks(), 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [allowed, canView, loadBlocks, session]);

  useEffect(() => {
    if (!selectedBlock?.id) {
      const timeout = window.setTimeout(() => resetAudit(), 0);
      return () => window.clearTimeout(timeout);
    }
    const timeout = window.setTimeout(() => void loadAudit(selectedBlock.id), 0);
    return () => window.clearTimeout(timeout);
  }, [selectedBlock?.id, loadAudit, resetAudit]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    allowed,
    canCreate,
    canListRewardPolicies,
    canRevoke,
    canView,
    error,
    fraudBlockCapability,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
    workspace,
    ...list,
    ...audit,
    ...actions,
    ...rewardPolicyOptions,
  };
}

export type AdminFraudBlocksRouteController = ReturnType<typeof useAdminFraudBlocksRoute>;
