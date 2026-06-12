"use client";

import { AlertCircle, CreditCard, Loader2, LogIn, ShieldCheck } from "lucide-react";
import Link from "next/link";
import { useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { type WalletSummary } from "@/lib/learner";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  fetchNotificationPreferences,
  readStoredSessionToken,
  saveNotificationPreferences,
  type CurrentSession,
  type NotificationPreferences,
} from "@/lib/session";
import styles from "../page.module.css";
import { AccountNextStep } from "./AccountNextStep";
import { NotificationPreferencesPanel } from "./NotificationPreferencesPanel";
import { ProfilePanel } from "./ProfilePanel";
import { StatusLine } from "./StatusLine";
import { VerificationPanel } from "./VerificationPanel";
import { WalletAccountStatus } from "./WalletAccountStatus";
import { WorkspaceAccessPanel } from "./WorkspaceAccessPanel";
import { accountNotice } from "./accountNotice";
import { fetchWalletForAccount } from "./fetchWalletForAccount";
import { normalizeAccountError } from "./normalizeAccountError";
import { walletStatusLabel } from "./walletStatusLabel";
import { type LoadState } from "./LoadState";
import { type PrefsSaveState } from "./PrefsSaveState";
import { type RouteError } from "./RouteError";
import { type WalletLoadState } from "./WalletLoadState";

export default function AccountSettingsPage() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [prefs, setPrefs] = useState<NotificationPreferences | null>(null);
  const [prefsEmail, setPrefsEmail] = useState(true);
  const [prefsMessage, setPrefsMessage] = useState<string | null>(null);
  const [prefsPush, setPrefsPush] = useState(false);
  const [prefsSaveState, setPrefsSaveState] = useState<PrefsSaveState>("idle");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [wallet, setWallet] = useState<WalletSummary | null>(null);
  const [walletError, setWalletError] = useState<RouteError | null>(null);
  const [walletState, setWalletState] = useState<WalletLoadState>("idle");

  async function loadPreferences(token: string) {
    try {
      const nextPrefs = await fetchNotificationPreferences({ token });
      setPrefs(nextPrefs);
      setPrefsEmail(nextPrefs.email_enabled);
      setPrefsPush(nextPrefs.push_enabled);
    } catch {
      setPrefs(null);
    }
  }

  const loadAccount = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setError(null);
      setWallet(null);
      setWalletError(null);
      setWalletState("idle");
      setLoadState("idle");
      return;
    }
    setHasToken(true);
    setLoadState("loading");
    setWalletState("loading");
    setError(null);
    setWalletError(null);
    try {
      const [nextSession, walletResult] = await Promise.all([
        fetchCurrentSession({ token }),
        fetchWalletForAccount(token),
      ]);
      setSession(nextSession);
      setWallet(walletResult.wallet);
      setWalletError(walletResult.error);
      setWalletState(walletResult.error ? "error" : "success");
      await loadPreferences(token);
      setLoadState("success");
    } catch (nextError) {
      const requestError = normalizeAccountError(nextError);
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setWallet(null);
      setWalletError(null);
      setWalletState("idle");
      setError(requestError);
      setLoadState("error");
    }
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadAccount(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadAccount]);

  const workspaceSummary = useMemo(() => {
    if (!session) return "No workspace loaded";
    const count = session.organizations.length + session.courses.length;
    return `${count} workspace${count === 1 ? "" : "s"}`;
  }, [session]);
  const walletStatus = walletStatusLabel(wallet, walletState, walletError);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setError(null);
    setWallet(null);
    setWalletError(null);
    setWalletState("idle");
    setLoadState("idle");
  }

  async function handleSavePrefs() {
    const token = readStoredSessionToken();
    if (!token) return;
    setPrefsSaveState("saving");
    setPrefsMessage(null);
    try {
      const updated = await saveNotificationPreferences({
        emailEnabled: prefsEmail,
        pushEnabled: prefsPush,
        token,
      });
      setPrefs(updated);
      setPrefsSaveState("success");
      setPrefsMessage("Preferences saved.");
    } catch (e) {
      setPrefsMessage(e instanceof Error ? e.message : "Failed to save.");
      setPrefsSaveState("error");
    }
  }

  return (
    <ProductShell activeNav="account" breadcrumbs={[{ href: "/session", label: "Workspace" }, { label: "Account" }]} description="Profile, verification readiness, wallet connection, and notification defaults." eyebrow="Settings" isSignedIn={hasToken || Boolean(session)} notice={accountNotice(error)} onSignOut={signOut} session={session} statusItems={<><StatusLine icon={<ShieldCheck size={16} aria-hidden />} label={loadState === "loading" ? "Resolving account" : session ? session.user.email_verified ? "Email verified" : "Email pending" : "No active session"} tone={session?.user.email_verified ? "good" : loadState === "error" ? "warn" : "neutral"} /><StatusLine icon={<CreditCard size={16} aria-hidden />} label={walletStatus.label} tone={walletStatus.tone} /></>} title="Account">
      {loadState === "loading" ? <LoadingAccount /> : null}
      {loadState === "idle" ? <SignedOutAccount /> : null}
      {error ? <AccountError error={error} /> : null}
      {session ? (
        <>
          <AccountNextStep session={session} wallet={wallet} walletError={walletError} />
          <section className={styles.summaryGrid}>
            <ProfilePanel onRefresh={() => void loadAccount()} session={session} />
            <VerificationPanel session={session} />
            <article className={styles.panel}>
              <div className={styles.panelHeader}><CreditCard size={22} aria-hidden /><h2>Wallet</h2></div>
              <WalletAccountStatus error={walletError} state={walletState} wallet={wallet} />
            </article>
            <WorkspaceAccessPanel session={session} workspaceSummary={workspaceSummary} />
          </section>
          <NotificationPreferencesPanel emailEnabled={prefsEmail} message={prefsMessage} onEmailChange={setPrefsEmail} onPushChange={setPrefsPush} onSave={() => void handleSavePrefs()} prefsLoaded={Boolean(prefs)} pushEnabled={prefsPush} saveState={prefsSaveState} />
        </>
      ) : null}
    </ProductShell>
  );
}

function LoadingAccount() {
  return <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite"><div className={styles.panelHeader}><Loader2 className={styles.spin} size={22} aria-hidden /><h2>Loading account</h2></div><p className={styles.muted}>Resolving profile, verification state, wallet readiness, and notification defaults.</p></section>;
}

function SignedOutAccount() {
  return <section className={`${styles.panel} ${styles.singlePanel}`}><div className={styles.panelHeader}><LogIn size={22} aria-hidden /><h2>Sign in required</h2></div><p className={styles.muted}>Account settings load after RustLearn resolves your current session.</p><Link className={styles.primaryLink} href="/login?redirect=/settings/account"><LogIn size={18} aria-hidden />Sign in</Link></section>;
}

function AccountError({ error }: { error: RouteError }) {
  return <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status"><AlertCircle size={18} aria-hidden /><span><strong>{error.code}</strong>{error.message}</span><Link className={styles.secondaryLink} href="/login?redirect=/settings/account">Return to login</Link></section>;
}
