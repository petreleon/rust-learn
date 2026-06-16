"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { buildOrganizationWorkspace, fetchOrganizationWalletAudit, findOrganizationWorkspaceItem, linkOrganizationWallet, organizationPermissionEnabled, type OrganizationWalletAudit } from "@/lib/organization";
import { clearStoredSessionToken, readStoredSessionToken } from "@/lib/session";
import { ErrorState } from "@/components/organization-routes/ErrorState";
import { LoadingState } from "@/components/organization-routes/LoadingState";
import { MissingOrganizationState } from "@/components/organization-routes/MissingOrganizationState";
import { OrganizationStatus } from "@/components/organization-routes/OrganizationStatus";
import { OrganizationWalletContent } from "@/components/organization-routes/OrganizationWalletContent";
import { SignedOutState } from "@/components/organization-routes/SignedOutState";
import { WalletDeniedState } from "@/components/organization-routes/WalletDeniedState";
import { emptyWorkspace } from "@/components/organization-routes/emptyWorkspace";
import { type RouteError } from "@/components/organization-routes/RouteError";
import { type WalletLinkState } from "@/components/organization-routes/WalletLinkState";
import { type WalletLoadState } from "@/components/organization-routes/WalletLoadState";
import { normalizeRouteError } from "@/components/organization-routes/normalizeRouteError";
import { organizationNotice } from "@/components/organization-routes/organizationNotice";
import { useOrganizationSession } from "@/components/organization-routes/useOrganizationSession";

export function OrganizationWalletRoute({ organizationId }: { organizationId: string }) {
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
  const canManageWallets = organizationPermissionEnabled(organization, "MANAGE_ORG_WALLETS");
  const canManageBudget = organizationPermissionEnabled(organization, "MANAGE_ORG_REWARD_BUDGET");
  const canViewReports = organizationPermissionEnabled(organization, "VIEW_ORG_REWARD_REPORTS");
  const canViewWallet = canManageWallets || canManageBudget || canViewReports;
  const walletCapability = organization?.capabilities.find((capability) => capability.key === "wallet");
  const [audit, setAudit] = useState<OrganizationWalletAudit | null>(null);
  const [walletError, setWalletError] = useState<RouteError | null>(null);
  const [walletLoadState, setWalletLoadState] = useState<WalletLoadState>("idle");
  const [linkError, setLinkError] = useState<RouteError | null>(null);
  const [linkState, setLinkState] = useState<WalletLinkState>("idle");
  const notice = organizationNotice(route.error || walletError || linkError);

  const loadWallet = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canViewWallet) {
      return;
    }

    setWalletError(null);
    setWalletLoadState("loading");

    try {
      const nextAudit = await fetchOrganizationWalletAudit({
        organizationId: organization.id,
        token,
      });
      setAudit(nextAudit);
      setWalletLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
      }

      if (routeError.status === 404 && /wallet not linked/i.test(routeError.message)) {
        setAudit(null);
        setWalletError(null);
        setWalletLoadState("missing");
        return;
      }

      setAudit(null);
      setWalletError(routeError);
      setWalletLoadState("error");
    }
  }, [canViewWallet, invalidOrganizationId, organization]);

  useEffect(() => {
    if (route.session && organization && canViewWallet) {
      const timeout = window.setTimeout(() => {
        setAudit(null);
        setLinkError(null);
        setLinkState("idle");
        setWalletError(null);
        setWalletLoadState("idle");
        void loadWallet();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canViewWallet, loadWallet, organization, route.session]);

  async function linkWallet() {
    const token = readStoredSessionToken();
    if (!token || !organization) {
      setLinkError({ code: "missing_token", message: "Sign in again before linking this wallet.", status: 401 });
      setLinkState("error");
      return;
    }
    if (!canManageWallets) {
      setLinkError({
        code: "permission_denied",
        message: "Wallet management is not enabled for this organization session.",
        status: 403,
      });
      setLinkState("error");
      return;
    }

    setLinkError(null);
    setLinkState("linking");

    try {
      await linkOrganizationWallet({
        organizationId: organization.id,
        token,
      });
      setLinkState("success");
      await loadWallet();
    } catch (nextError) {
      setLinkError(normalizeRouteError(nextError));
      setLinkState("error");
    }
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: organization ? `/organizations/${organization.id}` : undefined,
          label: organization?.name || "Organization",
        },
        { label: "Wallet" },
      ]}
      description="Organization wallet balance, budget readiness, reward-credit audit, token links, and permission-aware wallet actions."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} wallet` : "Organization wallet"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/wallet`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/wallet`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canViewWallet ? (
        <WalletDeniedState capability={walletCapability} organizationName={organization.name} />
      ) : null}
      {route.session && organization && canViewWallet ? (
        <OrganizationWalletContent
          audit={audit}
          canManageBudget={canManageBudget}
          canManageWallets={canManageWallets}
          linkError={linkError}
          linkState={linkState}
          loadState={walletLoadState}
          onLinkWallet={linkWallet}
          onRefresh={loadWallet}
          organization={organization}
          walletError={walletError}
        />
      ) : null}
    </ProductShell>
  );
}
