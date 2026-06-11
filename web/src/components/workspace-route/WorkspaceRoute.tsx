"use client";

import { BriefcaseBusiness, ShieldCheck } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { accessSummary, countDelegatedPermissions } from "@/lib/access";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession, SessionRequestError } from "@/lib/session";
import { ProductShell } from "../product-shell";
import { DeniedState } from "./DeniedState";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";
import { SignedOutState } from "./SignedOutState";
import { StatusPill } from "./StatusPill";
import { WorkspaceContent } from "./WorkspaceContent";
import { isWorkspaceAllowed } from "./isWorkspaceAllowed";
import { workspaceConfig } from "./WorkspaceConfig";
import { workspaceNotice } from "./workspaceNotice";
import { type LoadState } from "./LoadState";
import { type WorkspaceKind } from "./WorkspaceKind";

export function WorkspaceRoute({ kind }: { kind: WorkspaceKind }) {
  const config = workspaceConfig[kind];
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<{ code: string; message: string } | null>(null);

  const loadSession = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const nextSession = await fetchCurrentSession({ token });
      setSession(nextSession);
      setLoadState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof SessionRequestError
          ? nextError
          : new SessionRequestError("Workspace could not be loaded.", 0, "network_error");
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setError({ code: requestError.code, message: requestError.message });
      setLoadState("error");
    }
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  const access = useMemo(() => accessSummary(session), [session]);
  const allowed = Boolean(session && isWorkspaceAllowed(kind, access));

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setError(null);
    setLoadState("idle");
  }

  const notice = workspaceNotice(error, kind);

  return (
    <ProductShell
      activeNav={config.activeNav}
      breadcrumbs={[{ href: "/session", label: "Workspace" }, { label: config.title }]}
      description={config.description}
      eyebrow={config.eyebrow}
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={
        <>
          <StatusPill
            icon={<ShieldCheck size={16} aria-hidden />}
            label={allowed ? "Access available" : loadState === "loading" ? "Resolving access" : "Access gated"}
            tone={allowed ? "good" : loadState === "error" ? "warn" : "neutral"}
          />
          <StatusPill
            icon={<BriefcaseBusiness size={16} aria-hidden />}
            label={`${countDelegatedPermissions(session)} delegations`}
            tone="neutral"
          />
        </>
      }
      title={config.title}
    >
      {loadState === "idle" && !session ? <SignedOutState redirect={`/${kind}`} /> : null}
      {loadState === "loading" ? <LoadingState /> : null}
      {error ? <ErrorState error={error} redirect={`/${kind}`} /> : null}
      {session && !allowed ? <DeniedState config={config} /> : null}
      {session && allowed ? <WorkspaceContent kind={kind} session={session} /> : null}
    </ProductShell>
  );
}
