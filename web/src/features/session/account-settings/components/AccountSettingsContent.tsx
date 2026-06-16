"use client";

import { CreditCard } from "lucide-react";
import styles from "@/features/session/account-settings/page.module.css";
import { AccountNextStep } from "./AccountNextStep";
import { NotificationPreferencesPanel } from "./NotificationPreferencesPanel";
import { ProfilePanel } from "./ProfilePanel";
import { VerificationPanel } from "./VerificationPanel";
import { WalletAccountStatus } from "./WalletAccountStatus";
import { WorkspaceAccessPanel } from "./WorkspaceAccessPanel";
import { type AccountSettingsRouteController } from "../route/useAccountSettingsRoute";

export function AccountSettingsContent({ route }: { route: AccountSettingsRouteController }) {
  if (!route.session) return null;

  return (
    <>
      <AccountNextStep session={route.session} wallet={route.wallet} walletError={route.walletError} />
      <section className={styles.summaryGrid}>
        <ProfilePanel onRefresh={() => void route.loadAccount()} session={route.session} />
        <VerificationPanel session={route.session} token={route.activeToken} />
        <article className={styles.panel}>
          <div className={styles.panelHeader}>
            <CreditCard size={22} aria-hidden />
            <h2>Wallet</h2>
          </div>
          <WalletAccountStatus error={route.walletError} state={route.walletState} wallet={route.wallet} />
        </article>
        <WorkspaceAccessPanel session={route.session} workspaceSummary={route.workspaceSummary} />
      </section>
      <NotificationPreferencesPanel
        emailEnabled={route.prefsEmail}
        message={route.prefsMessage}
        onEmailChange={route.setPrefsEmail}
        onPushChange={route.setPrefsPush}
        onSave={() => void route.savePreferences()}
        prefsLoaded={Boolean(route.prefs)}
        pushEnabled={route.prefsPush}
        saveState={route.prefsSaveState}
      />
    </>
  );
}
