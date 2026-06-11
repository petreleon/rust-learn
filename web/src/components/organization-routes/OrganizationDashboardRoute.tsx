"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { buildOrganizationWorkspace, fetchOrganizationDashboard, findOrganizationWorkspaceItem, type OrganizationDashboardSummary } from "@/lib/organization";
import { clearStoredSessionToken, readStoredSessionToken } from "@/lib/session";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";
import { MissingOrganizationState } from "./MissingOrganizationState";
import { OrganizationDashboard } from "./OrganizationDashboard";
import { OrganizationStatus } from "./OrganizationStatus";
import { SignedOutState } from "./SignedOutState";
import { emptyWorkspace } from "./emptyWorkspace";
import { type DashboardLoadState } from "./DashboardLoadState";
import { type RouteError } from "./RouteError";
import { normalizeRouteError } from "./normalizeRouteError";
import { organizationNotice } from "./organizationNotice";
import { useOrganizationSession } from "./useOrganizationSession";

export function OrganizationDashboardRoute({ organizationId }: { organizationId: string }) {
  const route = useOrganizationSession();
  const numericOrganizationId = Number.parseInt(organizationId, 10);
  const invalidOrganizationId = !/^\d+$/.test(organizationId) || !Number.isFinite(numericOrganizationId);
  const organization = useMemo(
    () =>
      route.session && !invalidOrganizationId
        ? findOrganizationWorkspaceItem(route.session, numericOrganizationId)
        : null,
    [invalidOrganizationId, numericOrganizationId, route.session],
  );
  const workspace = useMemo(
    () => (route.session ? buildOrganizationWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const [dashboard, setDashboard] = useState<OrganizationDashboardSummary | null>(null);
  const [dashboardError, setDashboardError] = useState<RouteError | null>(null);
  const [dashboardLoadState, setDashboardLoadState] = useState<DashboardLoadState>("idle");
  const notice = organizationNotice(route.error || dashboardError);

  const loadDashboard = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization) {
      return;
    }

    setDashboardError(null);
    setDashboardLoadState("loading");

    try {
      const nextDashboard = await fetchOrganizationDashboard({
        organizationId: organization.id,
        token,
      });
      setDashboard(nextDashboard);
      setDashboardLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
      }
      setDashboard(null);
      setDashboardError(routeError);
      setDashboardLoadState("error");
    }
  }, [invalidOrganizationId, organization]);

  useEffect(() => {
    if (route.session && organization) {
      const timeout = window.setTimeout(() => {
        setDashboard(null);
        setDashboardError(null);
        setDashboardLoadState("idle");
        void loadDashboard();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [loadDashboard, organization, route.session]);

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        { label: organization?.name || "Organization" },
      ]}
      description="Organization permissions, delegated access, and action readiness for the selected workspace."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name || "Organization workspace"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect="/organizations" /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect="/organizations" /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization ? (
        <OrganizationDashboard
          dashboard={dashboard}
          dashboardError={dashboardError}
          dashboardLoadState={dashboardLoadState}
          onRefreshDashboard={loadDashboard}
          organization={organization}
        />
      ) : null}
    </ProductShell>
  );
}
