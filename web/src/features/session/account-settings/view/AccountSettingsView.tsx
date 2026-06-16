"use client";

import { ProductShell } from "@/components/product-shell";
import { AccountSettingsContent } from "../components/AccountSettingsContent";
import { AccountStatusItems } from "../components/AccountStatusItems";
import { AccountError, LoadingAccount, SignedOutAccount } from "../components/AccountStatePanels";
import { type AccountSettingsRouteController } from "../route/useAccountSettingsRoute";
import { accountNotice } from "./accountNotice";

export function AccountSettingsView({ route }: { route: AccountSettingsRouteController }) {
  return (
    <ProductShell
      activeNav="account"
      breadcrumbs={[{ href: "/session", label: "Workspace" }, { label: "Account" }]}
      description="Profile, verification readiness, wallet connection, and notification defaults."
      eyebrow="Settings"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={accountNotice(route.error)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <AccountStatusItems
          loadState={route.loadState}
          session={route.session}
          wallet={route.wallet}
          walletError={route.walletError}
          walletState={route.walletState}
        />
      }
      title="Account"
    >
      {route.loadState === "loading" ? <LoadingAccount /> : null}
      {route.loadState === "idle" ? <SignedOutAccount /> : null}
      {route.error ? <AccountError error={route.error} /> : null}
      <AccountSettingsContent route={route} />
    </ProductShell>
  );
}
