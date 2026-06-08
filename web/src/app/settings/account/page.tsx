"use client";

import {
  Bell,
  CheckCircle2,
  CreditCard,
  Loader2,
  LogIn,
  Mail,
  ShieldCheck,
  UserRound,
} from "lucide-react";
import Link from "next/link";
import { useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  SessionRequestError,
} from "@/lib/session";
import styles from "./page.module.css";

type LoadState = "idle" | "loading" | "success" | "error";

export default function AccountSettingsPage() {
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [error, setError] = useState<{ code: string; message: string } | null>(null);

  const loadAccount = useCallback(async (token: string) => {
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
          : new SessionRequestError("Account could not be loaded.", 0, "network_error");
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
    const timeout = window.setTimeout(() => {
      const token = readStoredSessionToken();
      if (!token) {
        setHasToken(false);
        setSession(null);
        setError(null);
        setLoadState("idle");
        return;
      }

      void loadAccount(token);
    }, 0);
    return () => window.clearTimeout(timeout);
  }, [loadAccount]);

  const workspaceSummary = useMemo(() => {
    if (!session) {
      return "No workspace loaded";
    }

    const count = session.organizations.length + session.courses.length;
    return `${count} workspace${count === 1 ? "" : "s"}`;
  }, [session]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setError(null);
    setLoadState("idle");
  }

  const notice = accountNotice(error);

  return (
    <ProductShell
      activeNav="account"
      breadcrumbs={[{ href: "/session", label: "Workspace" }, { label: "Account" }]}
      description="Profile, email status, wallet readiness, and notification preferences."
      eyebrow="Settings"
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={
        <span className={styles.statusPill}>
          <ShieldCheck size={16} aria-hidden />
          {workspaceSummary}
        </span>
      }
      title="Account"
    >

      {loadState === "loading" ? (
        <section className={styles.panel}>
          <Loader2 className={styles.spin} size={24} aria-hidden />
          <h2>Loading account</h2>
        </section>
      ) : null}

      {loadState === "idle" ? (
        <section className={styles.panel}>
          <div className={styles.panelHeader}>
            <LogIn size={22} aria-hidden />
            <h2>Sign in required</h2>
          </div>
          <p className={styles.muted}>
            Account settings load after the current session is resolved.
          </p>
          <Link className={styles.primaryLink} href="/login?redirect=/settings/account">
            Sign in
          </Link>
        </section>
      ) : null}

      {error ? (
        <section className={styles.errorBox} role="status">
          <strong>{error.code}</strong>
          <span>{error.message}</span>
          <Link className={styles.secondaryLink} href="/login?redirect=/settings/account">
            Return to login
          </Link>
        </section>
      ) : null}

      {session ? (
        <section className={styles.grid}>
          <article className={styles.panel}>
            <div className={styles.panelHeader}>
              <UserRound size={22} aria-hidden />
              <h2>Profile</h2>
            </div>
            <dl className={styles.detailList}>
              <div>
                <dt>Name</dt>
                <dd>{session.user.name}</dd>
              </div>
              <div>
                <dt>Email</dt>
                <dd>{session.user.email}</dd>
              </div>
              <div>
                <dt>User ID</dt>
                <dd>{session.user.id}</dd>
              </div>
            </dl>
          </article>

          <article className={styles.panel}>
            <div className={styles.panelHeader}>
              <Mail size={22} aria-hidden />
              <h2>Email</h2>
            </div>
            <StatusLine
              label={session.user.email_verified ? "Verified" : "Verification required"}
              tone={session.user.email_verified ? "good" : "warn"}
            />
            <p className={styles.muted}>
              Login and workspace access require a verified email address.
            </p>
          </article>

          <article className={styles.panel}>
            <div className={styles.panelHeader}>
              <CreditCard size={22} aria-hidden />
              <h2>Wallet</h2>
            </div>
            <StatusLine label="Wallet status placeholder" tone="neutral" />
            <p className={styles.muted}>
              Wallet linking, balances, and audit state will use the wallet API in a later
              account-settings slice.
            </p>
          </article>

          <article className={styles.panel}>
            <div className={styles.panelHeader}>
              <Bell size={22} aria-hidden />
              <h2>Notifications</h2>
            </div>
            <StatusLine label="Preferences placeholder" tone="neutral" />
            <p className={styles.muted}>
              Email and in-app notification controls are reserved until preference APIs are
              defined.
            </p>
          </article>
        </section>
      ) : null}
    </ProductShell>
  );
}

function StatusLine({ label, tone }: { label: string; tone: "good" | "neutral" | "warn" }) {
  return (
    <span className={`${styles.statusLine} ${styles[tone]}`}>
      <CheckCircle2 size={16} aria-hidden />
      {label}
    </span>
  );
}

function accountNotice(error: { code: string; message: string } | null): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user") {
    return {
      actionHref: "/login?redirect=/settings/account",
      actionLabel: "Sign in",
      message: "Your stored session is no longer valid. Sign in again to continue.",
      title: "Session expired",
      tone: "error",
    };
  }

  return {
    message: error.message,
    title: "Account status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}
