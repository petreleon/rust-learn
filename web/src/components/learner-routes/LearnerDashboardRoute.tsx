"use client";

import { useCallback, useEffect, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { fetchLearnerDashboard, type LearnerDashboardSnapshot } from "@/lib/learner";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { ErrorState } from "./ErrorState";
import { LearnerDashboardContent } from "./LearnerDashboardContent";
import { LoadingState } from "./LoadingState";
import { SignedOutState } from "./SignedOutState";
import { StatusPill } from "./StatusPill";
import { learnerNotice } from "./learnerNotice";
import { normalizeRouteError } from "./normalizeRouteError";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";

export function LearnerDashboardRoute() {
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [dashboard, setDashboard] = useState<LearnerDashboardSnapshot | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<RouteError | null>(null);

  const loadRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setDashboard(null);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const [nextSession, nextDashboard] = await Promise.all([
        fetchCurrentSession({ token }),
        fetchLearnerDashboard({ token }),
      ]);
      setSession(nextSession);
      setDashboard(nextDashboard);
      setLoadState("success");
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setDashboard(null);
      setError({
        code: requestError.code,
        message: requestError.message,
        status: requestError.status,
      });
      setLoadState("error");
    }
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setDashboard(null);
    setError(null);
    setLoadState("idle");
  }

  const statusLabel = loadState === "loading" ? "Loading" : session ? "Dashboard ready" : "Sign in required";

  return (
    <ProductShell
      activeNav="learn"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { label: "Learner" },
      ]}
      description="Continue learning, inspect rewards, and keep your wallet ready."
      eyebrow="Learner"
      isSignedIn={hasToken || Boolean(session)}
      notice={learnerNotice(error, "dashboard")}
      onSignOut={signOut}
      session={session}
      statusItems={<StatusPill label={statusLabel} tone={session ? "good" : "neutral"} />}
      title="Learner dashboard"
    >
      {loadState === "idle" && !session ? <SignedOutState redirect="/learn" /> : null}
      {loadState === "loading" ? <LoadingState /> : null}
      {error ? <ErrorState error={error} onRetry={loadRoute} redirect="/learn" /> : null}
      {loadState === "success" && session && dashboard ? (
        <LearnerDashboardContent dashboard={dashboard} onRefresh={loadRoute} session={session} />
      ) : null}
    </ProductShell>
  );
}
