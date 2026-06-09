"use client";

import {
  AlertCircle,
  Bell,
  BookOpen,
  Building2,
  CheckCircle2,
  CreditCard,
  Loader2,
  LogIn,
  Mail,
  RefreshCw,
  ShieldCheck,
  UserRound,
} from "lucide-react";
import Link from "next/link";
import { type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import { fetchMyWallet, LearnerRequestError, type WalletSummary } from "@/lib/learner";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  SessionRequestError,
} from "@/lib/session";
import styles from "./page.module.css";

type LoadState = "idle" | "loading" | "success" | "error";
type WalletLoadState = "idle" | "loading" | "success" | "error";

type RouteError = {
  code: string;
  message: string;
  status: number;
};

type WalletResult = {
  error: RouteError | null;
  wallet: WalletSummary | null;
};

const notificationPreferences = [
  {
    checked: true,
    detail: "Enrollment, course access, and role changes are delivered from backend events.",
    label: "Account and access updates",
  },
  {
    checked: true,
    detail: "Teacher review, token processing, and wallet-credit events stay enabled.",
    label: "Reward status updates",
  },
  {
    checked: true,
    detail: "Upload processing, failed media work, and course-content notices stay enabled.",
    label: "Course activity notices",
  },
];

export default function AccountSettingsPage() {
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [error, setError] = useState<RouteError | null>(null);
  const [wallet, setWallet] = useState<WalletSummary | null>(null);
  const [walletError, setWalletError] = useState<RouteError | null>(null);
  const [walletState, setWalletState] = useState<WalletLoadState>("idle");

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
    if (!session) {
      return "No workspace loaded";
    }

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

  const notice = accountNotice(error);

  return (
    <ProductShell
      activeNav="account"
      breadcrumbs={[{ href: "/session", label: "Workspace" }, { label: "Account" }]}
      description="Profile, verification readiness, wallet connection, and notification defaults."
      eyebrow="Settings"
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={
        <>
          <StatusLine
            icon={<ShieldCheck size={16} aria-hidden />}
            label={
              loadState === "loading"
                ? "Resolving account"
                : session
                  ? session.user.email_verified
                    ? "Email verified"
                    : "Email pending"
                  : "No active session"
            }
            tone={session?.user.email_verified ? "good" : loadState === "error" ? "warn" : "neutral"}
          />
          <StatusLine
            icon={<CreditCard size={16} aria-hidden />}
            label={walletStatus.label}
            tone={walletStatus.tone}
          />
        </>
      }
      title="Account"
    >
      {loadState === "loading" ? (
        <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
          <div className={styles.panelHeader}>
            <Loader2 className={styles.spin} size={22} aria-hidden />
            <h2>Loading account</h2>
          </div>
          <p className={styles.muted}>Resolving profile, verification state, wallet readiness, and notification defaults.</p>
        </section>
      ) : null}

      {loadState === "idle" ? (
        <section className={`${styles.panel} ${styles.singlePanel}`}>
          <div className={styles.panelHeader}>
            <LogIn size={22} aria-hidden />
            <h2>Sign in required</h2>
          </div>
          <p className={styles.muted}>Account settings load after RustLearn resolves your current session.</p>
          <Link className={styles.primaryLink} href="/login?redirect=/settings/account">
            <LogIn size={18} aria-hidden />
            Sign in
          </Link>
        </section>
      ) : null}

      {error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <Link className={styles.secondaryLink} href="/login?redirect=/settings/account">
            Return to login
          </Link>
        </section>
      ) : null}

      {session ? (
        <>
          <AccountNextStep session={session} wallet={wallet} walletError={walletError} />

          <section className={styles.summaryGrid}>
            <article className={`${styles.panel} ${styles.profilePanel}`}>
              <div className={styles.panelHeader}>
                <UserRound size={22} aria-hidden />
                <h2>Profile</h2>
              </div>
              <div className={styles.profileBlock}>
                <div>
                  <p className={styles.profileName}>{session.user.name}</p>
                  <p className={styles.muted}>{session.user.email}</p>
                </div>
                <div className={styles.badgeRow}>
                  <StatusLine
                    label={session.user.email_verified ? "Email verified" : "Email pending"}
                    tone={session.user.email_verified ? "good" : "warn"}
                  />
                  <StatusLine
                    label={session.user.kyc_verified ? "KYC verified" : "KYC pending"}
                    tone={session.user.kyc_verified ? "good" : "neutral"}
                  />
                </div>
                <div className={styles.actionRow}>
                  <button className={styles.secondaryButton} type="button" onClick={() => void loadAccount()}>
                    <RefreshCw size={18} aria-hidden />
                    Refresh
                  </button>
                </div>
              </div>
            </article>

            <article className={styles.panel}>
              <div className={styles.panelHeader}>
                <Mail size={22} aria-hidden />
                <h2>Verification</h2>
              </div>
              <div className={styles.checkList}>
                <ReadinessItem
                  detail={
                    session.user.email_verified
                      ? "Login, learner routes, and workspace APIs can load normally."
                      : "Verify this address before using protected product routes."
                  }
                  label={session.user.email_verified ? "Email is verified" : "Email verification needed"}
                  tone={session.user.email_verified ? "good" : "warn"}
                />
                <ReadinessItem
                  detail={
                    session.user.kyc_verified
                      ? "Identity checks are complete for flows that require them."
                      : "KYC is not required for the current learner screens, but future wallet operations may need it."
                  }
                  label={session.user.kyc_verified ? "KYC is verified" : "KYC not verified"}
                  tone={session.user.kyc_verified ? "good" : "neutral"}
                />
              </div>
              {!session.user.email_verified ? (
                <Link className={styles.secondaryLink} href="/verify-email">
                  Open verification
                </Link>
              ) : null}
            </article>

            <article className={styles.panel}>
              <div className={styles.panelHeader}>
                <CreditCard size={22} aria-hidden />
                <h2>Wallet</h2>
              </div>
              <WalletAccountStatus error={walletError} state={walletState} wallet={wallet} />
            </article>

            <article className={styles.panel}>
              <div className={styles.panelHeader}>
                <Building2 size={22} aria-hidden />
                <h2>Workspace access</h2>
              </div>
              <div className={styles.metricGrid}>
                <MetricCard icon={<Building2 size={18} aria-hidden />} label="Organizations" value={session.organizations.length} />
                <MetricCard icon={<BookOpen size={18} aria-hidden />} label="Courses" value={session.courses.length} />
                <MetricCard
                  icon={<ShieldCheck size={18} aria-hidden />}
                  label="Delegations"
                  value={session.delegated_permissions.length}
                />
              </div>
              <p className={styles.muted}>{workspaceSummary} available from your current session.</p>
              <Link className={styles.secondaryLink} href="/session">
                View access details
              </Link>
            </article>
          </section>

          <section className={styles.preferencePanel}>
            <div className={styles.panelHeader}>
              <Bell size={22} aria-hidden />
              <h2>Notification preferences</h2>
            </div>
            <p className={styles.muted}>
              RustLearn currently sends required account, access, reward, and course-event notices. Preference editing is disabled until the backend exposes a save contract.
            </p>
            <div className={styles.preferenceList}>
              {notificationPreferences.map((preference) => (
                <PreferenceRow key={preference.label} {...preference} />
              ))}
            </div>
          </section>
        </>
      ) : null}
    </ProductShell>
  );
}

function AccountNextStep({
  session,
  wallet,
  walletError,
}: {
  session: CurrentSession;
  wallet: WalletSummary | null;
  walletError: RouteError | null;
}) {
  const nextStep = accountNextStep(session, wallet, walletError);
  if (!nextStep) {
    return null;
  }

  return (
    <section className={styles.nextStepPanel}>
      <div className={styles.panelHeader}>
        {nextStep.icon}
        <h2>{nextStep.title}</h2>
      </div>
      <p className={styles.muted}>{nextStep.detail}</p>
      <Link className={styles.primaryLink} href={nextStep.href}>
        {nextStep.actionIcon}
        {nextStep.actionLabel}
      </Link>
    </section>
  );
}

function WalletAccountStatus({
  error,
  state,
  wallet,
}: {
  error: RouteError | null;
  state: WalletLoadState;
  wallet: WalletSummary | null;
}) {
  if (state === "loading") {
    return (
      <div className={styles.inlineStatus}>
        <Loader2 className={styles.spin} size={18} aria-hidden />
        <span>Checking wallet</span>
      </div>
    );
  }

  if (error) {
    return (
      <>
        <StatusLine icon={<AlertCircle size={16} aria-hidden />} label="Wallet unavailable" tone="warn" />
        <p className={styles.muted}>{error.message}</p>
        <Link className={styles.secondaryLink} href="/wallet">
          Open wallet
        </Link>
      </>
    );
  }

  if (!wallet) {
    return (
      <>
        <StatusLine icon={<CreditCard size={16} aria-hidden />} label="Wallet not linked" tone="warn" />
        <p className={styles.muted}>Link a RustLearn wallet before approved learner rewards can be credited.</p>
        <Link className={styles.primaryLink} href="/wallet">
          <CreditCard size={18} aria-hidden />
          Link wallet
        </Link>
      </>
    );
  }

  return (
    <>
      <StatusLine icon={<CreditCard size={16} aria-hidden />} label="Wallet linked" tone="good" />
      <div className={styles.walletValue}>
        <strong>{wallet.value}</strong>
        <span>{wallet.organization_id ? "Organization wallet" : "Personal wallet"}</span>
      </div>
      <p className={styles.muted}>Approved rewards can be credited to this wallet. Deposits and retirements are managed from the wallet route when available.</p>
      <Link className={styles.secondaryLink} href="/wallet">
        Open wallet
      </Link>
    </>
  );
}

function PreferenceRow({
  checked,
  detail,
  label,
}: {
  checked: boolean;
  detail: string;
  label: string;
}) {
  return (
    <label className={styles.preferenceRow}>
      <input checked={checked} disabled readOnly type="checkbox" />
      <span>
        <strong>{label}</strong>
        <small>{detail}</small>
      </span>
    </label>
  );
}

function ReadinessItem({
  detail,
  label,
  tone,
}: {
  detail: string;
  label: string;
  tone: "good" | "neutral" | "warn";
}) {
  return (
    <div className={styles.readinessItem}>
      <StatusLine label={label} tone={tone} />
      <p className={styles.muted}>{detail}</p>
    </div>
  );
}

function MetricCard({
  icon,
  label,
  value,
}: {
  icon: ReactNode;
  label: string;
  value: number;
}) {
  return (
    <div className={styles.metricCard}>
      {icon}
      <strong>{value}</strong>
      <span>{label}</span>
    </div>
  );
}

function StatusLine({
  icon = <CheckCircle2 size={16} aria-hidden />,
  label,
  tone,
}: {
  icon?: ReactNode;
  label: string;
  tone: "good" | "neutral" | "warn";
}) {
  return (
    <span className={`${styles.statusLine} ${styles[tone]}`}>
      {icon}
      {label}
    </span>
  );
}

async function fetchWalletForAccount(token: string): Promise<WalletResult> {
  try {
    return {
      error: null,
      wallet: await fetchMyWallet({ token }),
    };
  } catch (error) {
    return {
      error: normalizeAccountError(error),
      wallet: null,
    };
  }
}

function walletStatusLabel(
  wallet: WalletSummary | null,
  state: WalletLoadState,
  error: RouteError | null,
): { label: string; tone: "good" | "neutral" | "warn" } {
  if (state === "loading") {
    return { label: "Wallet loading", tone: "neutral" };
  }
  if (state === "idle") {
    return { label: "Wallet not loaded", tone: "neutral" };
  }
  if (error) {
    return { label: "Wallet unavailable", tone: "warn" };
  }
  if (wallet) {
    return { label: "Wallet linked", tone: "good" };
  }
  return { label: "Wallet unlinked", tone: "warn" };
}

function accountNextStep(
  session: CurrentSession,
  wallet: WalletSummary | null,
  walletError: RouteError | null,
): {
  actionIcon: ReactNode;
  actionLabel: string;
  detail: string;
  href: string;
  icon: ReactNode;
  title: string;
} | null {
  if (!session.user.email_verified) {
    return {
      actionIcon: <Mail size={18} aria-hidden />,
      actionLabel: "Open verification",
      detail: "Verify your email address before relying on protected account and learner workflows.",
      href: "/verify-email",
      icon: <Mail size={22} aria-hidden />,
      title: "Verify email",
    };
  }

  if (walletError) {
    return {
      actionIcon: <CreditCard size={18} aria-hidden />,
      actionLabel: "Open wallet",
      detail: "Your profile loaded, but RustLearn could not read wallet status. Open the wallet route to retry or continue from there.",
      href: "/wallet",
      icon: <AlertCircle size={22} aria-hidden />,
      title: "Check wallet status",
    };
  }

  if (!wallet) {
    return {
      actionIcon: <CreditCard size={18} aria-hidden />,
      actionLabel: "Link wallet",
      detail: "Link a RustLearn wallet so approved learner rewards have a destination.",
      href: "/wallet",
      icon: <CreditCard size={22} aria-hidden />,
      title: "Link wallet",
    };
  }

  return null;
}

function normalizeAccountError(error: unknown): RouteError {
  if (error instanceof SessionRequestError || error instanceof LearnerRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "network_error",
    message: "Account settings could not be loaded before the API responded.",
    status: 0,
  };
}

function accountNotice(error: RouteError | null): ShellNotice | null {
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

  if (error.code === "unverified_email") {
    return {
      actionHref: "/verify-email",
      actionLabel: "Verify email",
      message: "Verify this email address before changing account settings.",
      title: "Email verification required",
      tone: "warn",
    };
  }

  return {
    message: error.message,
    title: "Account status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}
