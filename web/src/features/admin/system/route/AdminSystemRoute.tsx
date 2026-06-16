"use client";

import { Database } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace, fetchPlatformSystemStatus, type PlatformSystemStatus } from "@/lib/admin";
import { ProductShell } from "@/components/product-shell";
import { AdminDeniedState } from "@/components/admin-routes/AdminDeniedState";
import { GatedPanel } from "@/components/admin-routes/GatedPanel";
import { LoadingState } from "@/components/admin-routes/LoadingState";
import { SessionErrorState } from "@/components/admin-routes/SessionErrorState";
import { SignedOutState } from "@/components/admin-routes/SignedOutState";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { SystemPanel } from "@/components/admin-routes/SystemPanel";
import { emptyWorkspace } from "@/components/admin-routes/emptyWorkspace";
import { formatUnderscoreLabel } from "@/components/admin-routes/formatUnderscoreLabel";
import { getCapability } from "@/components/admin-routes/getCapability";
import { hasPlatformCapability } from "@/components/admin-routes/hasPlatformCapability";
import { normalizeRouteError } from "@/components/admin-routes/normalizeRouteError";
import { useAdminSession } from "@/components/admin-routes/useAdminSession";
import { type RouteError } from "@/components/admin-routes/RouteError";
import { type SectionState } from "@/components/admin-routes/SectionState";

export function AdminSystemRoute() {
  const route = useAdminSession();
  const workspace = useMemo(
    () => (route.session ? buildPlatformAdminWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const allowed = route.session ? hasPlatformAdminAccess(route.session) : false;
  const canView = hasPlatformCapability(workspace, "system");

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
