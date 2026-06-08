"use client";

import {
  AlertTriangle,
  BookOpen,
  CreditCard,
  Filter,
  Loader2,
  LogIn,
  RefreshCw,
  Search,
  Trophy,
} from "lucide-react";
import Link from "next/link";
import { type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import {
  fetchMyWallet,
  fetchRewardHistory,
  LearnerRequestError,
  linkMyWallet,
  type RewardHistoryEntry,
  type WalletSummary,
} from "@/lib/learner";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CourseSessionScope,
  type CurrentSession,
  SessionRequestError,
} from "@/lib/session";
import styles from "./learner-routes.module.css";

type LoadState = "idle" | "loading" | "success" | "error";
type LearnerRouteKind = "courses" | "rewards" | "wallet";
type RewardStatusFilter =
  | "all"
  | "pending_teacher_approval"
  | "amount_approved"
  | "token_pending"
  | "token_confirmed"
  | "wallet_credited"
  | "needs_reconciliation"
  | "failed";

type RouteError = {
  code: string;
  message: string;
  status: number;
};

export function LearnerProductRoute({ kind }: { kind: LearnerRouteKind }) {
  const config = routeConfig[kind];
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<RouteError | null>(null);
  const [actionNotice, setActionNotice] = useState<ShellNotice | null>(null);
  const [rewards, setRewards] = useState<RewardHistoryEntry[]>([]);
  const [rewardStatus, setRewardStatus] = useState<RewardStatusFilter>("all");
  const [wallet, setWallet] = useState<WalletSummary | null>(null);
  const [walletLinking, setWalletLinking] = useState(false);

  const loadRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setError(null);
      setRewards([]);
      setWallet(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const sessionPromise = fetchCurrentSession({ token });
      const rewardsPromise =
        kind === "rewards"
          ? fetchRewardHistory({
              status: rewardStatus === "all" ? undefined : rewardStatus,
              token,
            })
          : Promise.resolve([]);
      const walletPromise = kind === "wallet" ? fetchMyWallet({ token }) : Promise.resolve(null);

      const [nextSession, nextRewards, nextWallet] = await Promise.all([
        sessionPromise,
        rewardsPromise,
        walletPromise,
      ]);

      setSession(nextSession);
      setRewards(nextRewards);
      setWallet(nextWallet);
      setLoadState("success");
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setError({
        code: requestError.code,
        message: requestError.message,
        status: requestError.status,
      });
      setLoadState("error");
    }
  }, [kind, rewardStatus]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setError(null);
    setActionNotice(null);
    setRewards([]);
    setWallet(null);
    setLoadState("idle");
  }

  async function linkWallet() {
    const token = readStoredSessionToken();
    if (!token) {
      setLoadState("idle");
      return;
    }

    setWalletLinking(true);
    setActionNotice(null);
    try {
      const result = await linkMyWallet({ token });
      setWallet(result.wallet);
      setActionNotice({
        message: result.created
          ? "Approved rewards can now be credited to this wallet."
          : "This account already had a RustLearn wallet.",
        title: result.created ? "Wallet linked" : "Wallet ready",
        tone: "success",
      });
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setActionNotice({
        message: requestError.message,
        title: requestError.code,
        tone: requestError.code === "timeout" || requestError.code === "network_error" ? "warn" : "error",
      });
    } finally {
      setWalletLinking(false);
    }
  }

  const notice = actionNotice || learnerNotice(error, kind);
  const statusLabel = loadState === "loading" ? "Loading" : session ? "Learner access" : "Sign in required";

  return (
    <ProductShell
      activeNav="learn"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/learn", label: "Learner" },
        { label: config.title },
      ]}
      description={config.description}
      eyebrow="Learner"
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={<StatusPill label={statusLabel} tone={session ? "good" : "neutral"} />}
      title={config.title}
    >
      {loadState === "idle" && !session ? <SignedOutState redirect={config.href} /> : null}
      {loadState === "loading" ? <LoadingState /> : null}
      {error ? <ErrorState error={error} onRetry={loadRoute} redirect={config.href} /> : null}
      {loadState === "success" && session && kind === "courses" ? <CoursesContent session={session} /> : null}
      {loadState === "success" && session && kind === "rewards" ? (
        <RewardsContent
          filter={rewardStatus}
          onChangeFilter={(nextFilter) => setRewardStatus(nextFilter)}
          rewards={rewards}
        />
      ) : null}
      {loadState === "success" && session && kind === "wallet" ? (
        <WalletContent
          linking={walletLinking}
          onLinkWallet={linkWallet}
          onRefresh={loadRoute}
          wallet={wallet}
        />
      ) : null}
    </ProductShell>
  );
}

function CoursesContent({ session }: { session: CurrentSession }) {
  const rewardCourseCount = session.courses.filter((course) =>
    course.effective_permissions.includes("VIEW_COURSE_REWARD_STATUS"),
  ).length;

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Current courses" value={session.courses.length} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward-ready" value={rewardCourseCount} />
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Organizations" value={session.organizations.length} />
      </section>

      <section className={styles.catalogPanel}>
        <div className={styles.panelHeader}>
          <Search size={20} aria-hidden />
          <h2>Catalog opening soon</h2>
        </div>
        <p>
          Your current course access appears below. Search, filters, enrollment state, teacher
          context, and course detail need course catalog data before they can be shown accurately.
        </p>
        <div className={styles.disabledSearch} aria-disabled="true">
          <span>
            <Search size={17} aria-hidden />
            Search courses
          </span>
          <span>
            <Filter size={17} aria-hidden />
            Filters
          </span>
        </div>
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Current course access</h2>
          <StatusPill label={`${session.courses.length} visible`} tone="neutral" />
        </div>
        {session.courses.length ? (
          <div className={styles.itemGrid}>
            {session.courses.map((course) => (
              <CourseAccessCard course={course} key={course.id} />
            ))}
          </div>
        ) : (
          <EmptyState
            actionHref="/learn"
            actionLabel="Back to learner workspace"
            detail="When the course catalog is available, this page should be the starting point for finding and joining courses."
            title="No courses yet"
          />
        )}
      </section>
    </>
  );
}

function CourseAccessCard({ course }: { course: CourseSessionScope }) {
  const rewardVisible = course.effective_permissions.includes("VIEW_COURSE_REWARD_STATUS");
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{course.title}</h3>
        <StatusPill label={humanize(course.lifecycle_status)} tone={course.lifecycle_status === "published" ? "good" : "neutral"} />
      </div>
      <div className={styles.metaRow}>
        <span>{course.effective_permissions.length} permissions</span>
        <span>{course.roles.length ? course.roles.join(", ") : "Direct access"}</span>
        <span>{rewardVisible ? "Rewards visible" : "Rewards hidden"}</span>
      </div>
      <div className={styles.actionRow}>
        <Link className={styles.secondaryLink} href="/rewards">
          View rewards
        </Link>
      </div>
    </article>
  );
}

function RewardsContent({
  filter,
  onChangeFilter,
  rewards,
}: {
  filter: RewardStatusFilter;
  onChangeFilter: (filter: RewardStatusFilter) => void;
  rewards: RewardHistoryEntry[];
}) {
  const summary = useMemo(() => {
    return {
      credited: rewards.filter((reward) => reward.wallet_credit).length,
      needsHelp: rewards.filter((reward) => reward.status === "failed" || reward.status === "needs_reconciliation").length,
      processing: rewards.filter((reward) => reward.status.includes("token") || reward.status === "amount_approved").length,
    };
  }, [rewards]);

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward records" value={rewards.length} />
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet credits" value={summary.credited} />
        <SummaryCard icon={<AlertTriangle size={20} aria-hidden />} label="Need help" value={summary.needsHelp} />
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Reward history</h2>
          <StatusPill label={`${summary.processing} processing`} tone="neutral" />
        </div>
        <div className={styles.filterBar} aria-label="Reward status filters">
          {rewardFilterOptions.map((option) => (
            <button
              aria-pressed={filter === option.value}
              className={`${styles.filterButton} ${filter === option.value ? styles.activeFilter : ""}`}
              key={option.value}
              type="button"
              onClick={() => onChangeFilter(option.value)}
            >
              {option.label}
            </button>
          ))}
        </div>

        {rewards.length ? (
          <div className={styles.itemGrid}>
            {rewards.map((reward) => (
              <RewardCard key={reward.reward_candidate_id} reward={reward} />
            ))}
          </div>
        ) : (
          <EmptyState
            action={
              filter === "all" ? undefined : (
                <button className={styles.secondaryButton} type="button" onClick={() => onChangeFilter("all")}>
                  Clear filter
                </button>
              )
            }
            actionHref={filter === "all" ? "/courses" : undefined}
            actionLabel={filter === "all" ? "Check courses" : undefined}
            detail={
              filter === "all"
                ? "Rewards appear after eligible course activity is submitted and reviewed."
                : "There are no rewards in this status right now."
            }
            title={filter === "all" ? "No reward history yet" : "No matching rewards"}
          />
        )}
      </section>
    </>
  );
}

function WalletContent({
  linking,
  onLinkWallet,
  onRefresh,
  wallet,
}: {
  linking: boolean;
  onLinkWallet: () => void;
  onRefresh: () => void;
  wallet: WalletSummary | null;
}) {
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet state" value={wallet ? "Linked" : "Unlinked"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Available value" value={wallet?.value || "0"} />
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Owner" value={wallet?.owner_type || "User"} />
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Wallet summary</h2>
          <button className={styles.iconAction} aria-label="Refresh wallet" type="button" onClick={onRefresh}>
            <RefreshCw size={18} aria-hidden />
          </button>
        </div>
        {wallet ? (
          <article className={styles.itemCard}>
            <div className={styles.itemHeader}>
              <h3>Wallet #{wallet.id}</h3>
              <StatusPill label={wallet.owner_type} tone="good" />
            </div>
            <p className={styles.muted}>Available value</p>
            <strong className={styles.walletValue}>{wallet.value}</strong>
            <div className={styles.metaRow}>
              <span>{wallet.user_id ? `User ${wallet.user_id}` : "No user owner"}</span>
              <span>{wallet.organization_id ? `Organization ${wallet.organization_id}` : "Personal wallet"}</span>
            </div>
            <div className={styles.actionRow}>
              <Link className={styles.secondaryLink} href="/rewards">
                View rewards
              </Link>
            </div>
          </article>
        ) : (
          <EmptyState
            action={
              <button className={styles.primaryLink} disabled={linking} onClick={onLinkWallet} type="button">
                {linking ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CreditCard size={18} aria-hidden />}
                Link wallet
              </button>
            }
            detail="A RustLearn wallet is required before approved rewards can be credited."
            title="Wallet not linked"
          />
        )}
      </section>
    </>
  );
}

function RewardCard({ reward }: { reward: RewardHistoryEntry }) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{reward.course_title}</h3>
        <StatusPill label={humanRewardStatus(reward.status)} tone={rewardTone(reward.status)} />
      </div>
      <p className={styles.muted}>{rewardNextStep(reward)}</p>
      <div className={styles.metaRow}>
        <span>{humanize(reward.event_type)}</span>
        <span>{reward.approved_amount ? `${reward.approved_amount} approved` : "Amount pending"}</span>
        <span>{reward.wallet_credit ? "Wallet credited" : "Wallet pending"}</span>
      </div>
      <div className={styles.detailList}>
        <span>Updated {formatDate(reward.updated_at)}</span>
        {reward.wallet_credit ? <span>Credited {reward.wallet_credit.amount}</span> : null}
        {reward.token_transaction?.transaction_hash ? (
          <span>Token tx {reward.token_transaction.transaction_hash.slice(0, 12)}</span>
        ) : null}
      </div>
    </article>
  );
}

function SignedOutState({ redirect }: { redirect: string }) {
  return (
    <section className={styles.statePanel}>
      <div className={styles.panelHeader}>
        <LogIn size={20} aria-hidden />
        <h2>Sign in required</h2>
      </div>
      <p className={styles.muted}>Learner routes load after RustLearn resolves your current session.</p>
      <Link className={styles.primaryLink} href={loginHref(redirect)}>
        <LogIn size={18} aria-hidden />
        Sign in
      </Link>
    </section>
  );
}

function LoadingState() {
  return (
    <section className={styles.statePanel} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Loading learner route</h2>
      </div>
      <div className={styles.skeletonGrid} aria-hidden>
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
      </div>
    </section>
  );
}

function ErrorState({
  error,
  onRetry,
  redirect,
}: {
  error: RouteError;
  onRetry: () => void;
  redirect: string;
}) {
  const needsSignIn = error.code === "unauthorized" || error.code === "missing_token" || error.code === "missing_user";
  const needsVerification = error.code === "unverified_email";

  return (
    <section className={styles.errorPanel} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>{error.code}</h2>
      </div>
      <p>{error.message}</p>
      {needsVerification ? (
        <Link className={styles.secondaryLink} href="/verify-email">
          Verify email
        </Link>
      ) : needsSignIn ? (
        <Link className={styles.secondaryLink} href={loginHref(redirect)}>
          Return to login
        </Link>
      ) : (
        <button className={styles.secondaryButton} type="button" onClick={onRetry}>
          <RefreshCw size={18} aria-hidden />
          Retry
        </button>
      )}
    </section>
  );
}

function EmptyState({
  action,
  actionHref,
  actionLabel,
  detail,
  title,
}: {
  action?: ReactNode;
  actionHref?: string;
  actionLabel?: string;
  detail: string;
  title: string;
}) {
  return (
    <section className={styles.statePanel}>
      <h3>{title}</h3>
      <p className={styles.muted}>{detail}</p>
      {action}
      {actionHref && actionLabel ? (
        <Link className={styles.secondaryLink} href={actionHref}>
          {actionLabel}
        </Link>
      ) : null}
    </section>
  );
}

function SummaryCard({
  icon,
  label,
  value,
}: {
  icon: ReactNode;
  label: string;
  value: number | string;
}) {
  return (
    <article className={styles.summaryCard}>
      {icon}
      <strong>{value}</strong>
      <span className={styles.muted}>{label}</span>
    </article>
  );
}

function StatusPill({
  label,
  tone,
}: {
  label: string;
  tone: "bad" | "good" | "neutral" | "warn";
}) {
  return <span className={`${styles.statusPill} ${styles[tone]}`}>{label}</span>;
}

function learnerNotice(error: RouteError | null, kind: LearnerRouteKind): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user" || error.code === "missing_token") {
    return {
      actionHref: loginHref(routeConfig[kind].href),
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
      message: "Verify this email address before using learner routes.",
      title: "Email verification required",
      tone: "warn",
    };
  }

  return {
    message: error.message,
    title: "Learner route status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}

function normalizeRouteError(error: unknown) {
  if (error instanceof SessionRequestError) {
    return error;
  }

  if (error instanceof LearnerRequestError) {
    return error;
  }

  return new LearnerRequestError("Learner route failed before the API responded.", 0, "network_error");
}

function rewardTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "wallet_credited" || status === "completed" || status === "notified") {
    return "good";
  }
  if (status === "failed") {
    return "bad";
  }
  if (status === "needs_reconciliation" || status.endsWith("_rejected")) {
    return "warn";
  }
  return "neutral";
}

function rewardNextStep(reward: RewardHistoryEntry) {
  switch (reward.status) {
    case "pending_teacher_approval":
      return "Waiting for teacher review.";
    case "teacher_approved":
      return "Teacher approved this activity; amount review is next.";
    case "teacher_rejected":
      return "Teacher rejected this activity. Check the course context before resubmitting.";
    case "amount_approved":
    case "adjusted":
      return "Amount is approved and token processing can continue.";
    case "amount_rejected":
      return "Amount review rejected this reward.";
    case "token_pending":
      return "Token transaction is being processed.";
    case "token_confirmed":
      return "Token transaction is confirmed; wallet credit is next.";
    case "wallet_credited":
    case "notified":
    case "completed":
      return "Reward was credited to the wallet.";
    case "needs_reconciliation":
      return "Reward needs reconciliation before it is final.";
    case "failed":
      return "Reward failed and needs support review.";
    default:
      return "Reward status changed after this page loaded.";
  }
}

function humanRewardStatus(status: string) {
  return humanize(status);
}

function humanize(value: string) {
  return value.replaceAll("_", " ");
}

function formatDate(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(date);
}

function loginHref(redirect: string) {
  return `/login?redirect=${encodeURIComponent(redirect)}`;
}

const rewardFilterOptions: Array<{ label: string; value: RewardStatusFilter }> = [
  { label: "All", value: "all" },
  { label: "Pending", value: "pending_teacher_approval" },
  { label: "Approved", value: "amount_approved" },
  { label: "Processing", value: "token_pending" },
  { label: "Confirmed", value: "token_confirmed" },
  { label: "Credited", value: "wallet_credited" },
  { label: "Needs help", value: "needs_reconciliation" },
  { label: "Failed", value: "failed" },
];

const routeConfig = {
  courses: {
    description: "Current learner course access and catalog readiness.",
    href: "/courses",
    title: "Courses",
  },
  rewards: {
    description: "Reward history with human status, wallet credit, and course context.",
    href: "/rewards",
    title: "Rewards",
  },
  wallet: {
    description: "Wallet link state, balance, and reward-credit readiness.",
    href: "/wallet",
    title: "Wallet",
  },
} as const;
