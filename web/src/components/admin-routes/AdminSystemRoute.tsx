"use client";

import { Database } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace, fetchPlatformSystemStatus, type PlatformSystemStatus } from "@/lib/admin";
import { ProductShell } from "../product-shell";
import { AdminDeniedState } from "./AdminDeniedState";
import { GatedPanel } from "./GatedPanel";
import { LoadingState } from "./LoadingState";
import { SessionErrorState } from "./SessionErrorState";
import { SignedOutState } from "./SignedOutState";
import { StatusPill } from "./StatusPill";
import { SystemPanel } from "./SystemPanel";
import { emptyWorkspace } from "./emptyWorkspace";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { getCapability } from "./getCapability";
import { hasAnyPlatformPermission } from "./hasAnyPlatformPermission";
import { normalizeRouteError } from "./normalizeRouteError";
import { useAdminSession } from "./useAdminSession";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function AdminSystemRoute() {
  const route = useAdminSession();
  const workspace = useMemo(
    () => (route.session ? buildPlatformAdminWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const allowed = route.session ? hasPlatformAdminAccess(route.session) : false;
  const canView = hasAnyPlatformPermission(workspace, ["VIEW_REPORT", "VIEW_AUDIT_LOGS", "VIEW_ANALYTICS_DASHBOARD"]);

  const [status, setStatus] = useState<PlatformSystemStatus | null>(null);
  const [error, setError] = useState<RouteError | null>(null);
  const [state, setState] = useState<SectionState>("idle");

  const loadStatus = useCallback(async () => {
    setState("loading");
    setError(null);
    try {
      const next = await fetchPlatformSystemStatus();
      setStatus(next);
      setState("success");
    } catch (err) {
      setStatus(null);
      setError(normalizeRouteError(err, "System status could not be loaded."));
      setState("error");
    }
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadStatus(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadStatus]);

  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "System" },
      ]}
      description="API liveness and dependency readiness from the runtime health endpoints."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : canView ? "System access" : "System gated"} />
          <StatusPill label={status?.liveness.status ? formatUnderscoreLabel(status.liveness.status) : "Checking"} />
        </>
      }
      title="System status"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !allowed ? <AdminDeniedState workspace={workspace} /> : null}

      {route.session && allowed ? (
        canView ? (
          <SystemPanel error={error} onRetry={loadStatus} state={state} status={status} />
        ) : (
          <GatedPanel
            capability={getCapability(workspace, "system")}
            icon={<Database size={20} aria-hidden />}
            title="System status unavailable"
          />
        )
      ) : null}
    </ProductShell>
  );
}
