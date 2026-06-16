"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace } from "@/lib/admin/buildPlatformAdminWorkspace";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminRewardAmountSession } from "../api/rewardAmountReviewApi";
import {
  canApproveRewardAmount,
  canViewRewardAmountReview,
  emptyRewardAmountWorkspace,
  findRewardAmountCapability,
} from "../model/rewardAmountAccessModel";
import { normalizeAdminRewardAmountRouteError } from "./normalizeAdminRewardAmountRouteError";
import { useRewardAmountDecision } from "./useRewardAmountDecision";
import { useRewardCandidateAudit } from "./useRewardCandidateAudit";
import { useRewardCandidateList } from "./useRewardCandidateList";

function isExpiredSession(error: RouteError) {
  return error.status === 401 || error.status === 404;
}

export function useAdminRewardAmountReviewRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const workspace = useMemo(
    () => (session ? buildPlatformAdminWorkspace(session) : emptyRewardAmountWorkspace),
    [session],
  );
  const allowed = session ? hasPlatformAdminAccess(session) : false;
  const canApprove = canApproveRewardAmount(workspace);
  const canView = canViewRewardAmountReview(workspace);
  const rewardAmountCapability = findRewardAmountCapability(workspace);
  const list = useRewardCandidateList({ allowed, canView, session });
  const audit = useRewardCandidateAudit({ canView });
  const { loadAudit, resetAudit } = audit;
  const { loadCandidates, selectedCandidate } = list;
  const decision = useRewardAmountDecision({
    onCandidateChanged: loadCandidates,
    onConflict: loadAudit,
    onDecisionSaved: loadAudit,
    selectedCandidate,
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
      setSession(await loadAdminRewardAmountSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeAdminRewardAmountRouteError(
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
      const timeout = window.setTimeout(() => void loadCandidates(), 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [allowed, canView, loadCandidates, session]);

  useEffect(() => {
    if (!selectedCandidate?.id) {
      const timeout = window.setTimeout(() => resetAudit(), 0);
      return () => window.clearTimeout(timeout);
    }
    const timeout = window.setTimeout(() => void loadAudit(selectedCandidate.id), 0);
    return () => window.clearTimeout(timeout);
  }, [selectedCandidate?.id, loadAudit, resetAudit]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    allowed,
    canApprove,
    canView,
    error,
    hasToken,
    loadSession,
    loadState,
    rewardAmountCapability,
    session,
    signOut,
    workspace,
    ...list,
    ...audit,
    ...decision,
  };
}

export type AdminRewardAmountReviewRouteController = ReturnType<
  typeof useAdminRewardAmountReviewRoute
>;
