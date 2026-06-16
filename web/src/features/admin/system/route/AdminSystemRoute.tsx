"use client";

import { Database } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace, fetchPlatformSystemStatus, type PlatformSystemStatus } from "@/lib/admin";
import { ProductShell } from "@/components/product-shell";
import { AdminDeniedState } from "@/features/admin/shared/route-kit/AdminDeniedState";
import { GatedPanel } from "@/features/admin/shared/route-kit/GatedPanel";
import { LoadingState } from "@/features/admin/shared/route-kit/LoadingState";
import { SessionErrorState } from "@/features/admin/shared/route-kit/SessionErrorState";
import { SignedOutState } from "@/features/admin/shared/route-kit/SignedOutState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { SystemPanel } from "@/features/admin/shared/route-kit/SystemPanel";
import { emptyWorkspace } from "@/features/admin/shared/route-kit/emptyWorkspace";
import { formatUnderscoreLabel } from "@/features/admin/shared/route-kit/formatUnderscoreLabel";
import { getCapability } from "@/features/admin/shared/route-kit/getCapability";
import { hasPlatformCapability } from "@/features/admin/shared/route-kit/hasPlatformCapability";
import { normalizeRouteError } from "@/features/admin/shared/route-kit/normalizeRouteError";
import { useAdminSession } from "@/features/admin/shared/route-kit/useAdminSession";
import { type RouteError } from "@/features/admin/shared/route-kit/RouteError";
import { type SectionState } from "@/features/admin/shared/route-kit/SectionState";

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
