"use client";

import {
  AlertTriangle,
  BookOpen,
  BriefcaseBusiness,
  Building2,
  CreditCard,
  Loader2,
  LogIn,
  ShieldCheck,
  Trophy,
} from "lucide-react";
import Link from "next/link";
import { type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { accessSummary, countDelegatedPermissions, hasOrganizationAccess, hasPlatformAdminAccess, hasTeacherApplicationAccess, hasTeacherCourseAccess } from "@/lib/access";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  type PlatformSessionScope,
  SessionRequestError,
} from "@/lib/session";
import { ProductShell } from "./product-shell";
import { type ShellNotice } from "./product-shell";
import styles from "./workspace-route.module.css";

type LoadState = "idle" | "loading" | "success" | "error";
type WorkspaceKind = "learn" | "teach" | "organizations" | "admin";

type WorkspaceConfig = {
  activeNav: WorkspaceKind;
  deniedSignals: string[];
  description: string;
  eyebrow: string;
  title: string;
};

const workspaceConfig: Record<WorkspaceKind, WorkspaceConfig> = {
  learn: {
    activeNav: "learn",
    deniedSignals: ["A verified product session"],
    description: "Courses, learning access, reward progress, and wallet readiness.",
    eyebrow: "Learner",
    title: "Learner workspace",
  },
  teach: {
    activeNav: "teach",
    deniedSignals: [
      "An approved course teaching permission",
      "A delegated course teaching permission",
      "Teacher application capability",
    ],
    description: "Teaching scopes, course permissions, and application status signals.",
    eyebrow: "Teacher",
    title: "Teaching workspace",
  },
  organizations: {
    activeNav: "organizations",
    deniedSignals: [
      "Organization membership",
      "Organization-scoped view or management permission",
      "Delegated organization permission",
    ],
    description: "Organization memberships, scoped permissions, reports, and wallet readiness.",
    eyebrow: "Organization",
    title: "Organization workspace",
  },
  admin: {
    activeNav: "admin",
    deniedSignals: [
      "Platform review permission",
      "Platform reward, fraud, export, wallet, or settings permission",
      "Active delegated platform permission",
    ],
    description: "Platform review, reward, fraud, export, wallet, and audit scopes.",
    eyebrow: "Platform",
    title: "Admin workspace",
  },
};

export function WorkspaceRoute({ kind }: { kind: WorkspaceKind }) {
  const config = workspaceConfig[kind];
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<{ code: string; message: string } | null>(null);

  const loadSession = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setError(null);
      setLoadState("idle");
      return;
    }

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
          : new SessionRequestError("Workspace could not be loaded.", 0, "network_error");
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
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  const access = useMemo(() => accessSummary(session), [session]);
  const allowed = Boolean(session && isWorkspaceAllowed(kind, access));

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setError(null);
    setLoadState("idle");
  }

  const notice = workspaceNotice(error, kind);

  return (
    <ProductShell
      activeNav={config.activeNav}
      breadcrumbs={[{ href: "/session", label: "Workspace" }, { label: config.title }]}
      description={config.description}
      eyebrow={config.eyebrow}
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={
        <>
          <StatusPill
            icon={<ShieldCheck size={16} aria-hidden />}
            label={allowed ? "Access available" : loadState === "loading" ? "Resolving access" : "Access gated"}
            tone={allowed ? "good" : loadState === "error" ? "warn" : "neutral"}
          />
          <StatusPill
            icon={<BriefcaseBusiness size={16} aria-hidden />}
            label={`${countDelegatedPermissions(session)} delegations`}
            tone="neutral"
          />
        </>
      }
      title={config.title}
    >
      {loadState === "idle" && !session ? <SignedOutState redirect={`/${kind}`} /> : null}
      {loadState === "loading" ? <LoadingState /> : null}
      {error ? <ErrorState error={error} redirect={`/${kind}`} /> : null}
      {session && !allowed ? <DeniedState config={config} /> : null}
      {session && allowed ? <WorkspaceContent kind={kind} session={session} /> : null}
    </ProductShell>
  );
}

function workspaceNotice(
  error: { code: string; message: string } | null,
  kind: WorkspaceKind,
): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user") {
    return {
      actionHref: `/login?redirect=/${kind}`,
      actionLabel: "Sign in",
      message: "Your stored session is no longer valid. Sign in again to continue.",
      title: "Session expired",
      tone: "error",
    };
  }

  return {
    message: error.message,
    title: "Workspace status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}

function SignedOutState({ redirect }: { redirect: string }) {
  return (
    <section className={styles.statePanel}>
      <div className={styles.panelHeader}>
        <LogIn size={20} aria-hidden />
        <h2>Sign in required</h2>
      </div>
      <p className={styles.muted}>
        This workspace is shown after RustLearn resolves your current account and scoped
        permissions.
      </p>
      <Link className={styles.primaryLink} href={`/login?redirect=${redirect}`}>
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
        <h2>Resolving workspace</h2>
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
  redirect,
}: {
  error: { code: string; message: string };
  redirect: string;
}) {
  return (
    <section className={styles.errorPanel} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>{error.code}</h2>
      </div>
      <p>{error.message}</p>
      <Link className={styles.secondaryLink} href={`/login?redirect=${redirect}`}>
        Return to login
      </Link>
    </section>
  );
}

function DeniedState({ config }: { config: WorkspaceConfig }) {
  return (
    <section className={styles.deniedPanel} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Access not available</h2>
      </div>
      <p>
        This workspace is hidden from primary navigation until the current session includes one
        of these resolved access signals.
      </p>
      <ul className={styles.deniedList}>
        {config.deniedSignals.map((signal) => (
          <li key={signal}>{signal}</li>
        ))}
      </ul>
      <Link className={styles.secondaryLink} href="/session">
        Review current session
      </Link>
    </section>
  );
}

function WorkspaceContent({ kind, session }: { kind: WorkspaceKind; session: CurrentSession }) {
  switch (kind) {
    case "learn":
      return <LearnerContent session={session} />;
    case "teach":
      return <TeacherContent session={session} />;
    case "organizations":
      return <OrganizationContent session={session} />;
    case "admin":
      return <AdminContent session={session} />;
  }
}

function LearnerContent({ session }: { session: CurrentSession }) {
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Courses" value={session.courses.length} />
        <SummaryCard
          icon={<ShieldCheck size={20} aria-hidden />}
          label="Reward scopes"
          value={session.courses.filter((course) => course.effective_permissions.includes("VIEW_COURSE_REWARD_STATUS")).length}
        />
        <SummaryCard icon={<BriefcaseBusiness size={20} aria-hidden />} label="Organizations" value={session.organizations.length} />
      </section>
      <section className={styles.statePanel}>
        <div className={styles.panelHeader}>
          <BookOpen size={20} aria-hidden />
          <h2>Continue as a learner</h2>
        </div>
        <p className={styles.muted}>
          Open course access, reward history, or wallet readiness without leaving the product
          workspace.
        </p>
        <div className={styles.actionRow}>
          <Link className={styles.primaryLink} href="/courses">
            <BookOpen size={18} aria-hidden />
            Courses
          </Link>
          <Link className={styles.secondaryLink} href="/rewards">
            <Trophy size={18} aria-hidden />
            Rewards
          </Link>
          <Link className={styles.secondaryLink} href="/wallet">
            <CreditCard size={18} aria-hidden />
            Wallet
          </Link>
        </div>
      </section>
      <ScopeList
        empty="Course enrollments and learning scopes will appear here."
        scopes={session.courses}
        title="Courses"
      />
    </>
  );
}

function TeacherContent({ session }: { session: CurrentSession }) {
  const teacherCourses = session.courses.filter(hasTeacherCourseAccess);
  const canApply = hasTeacherApplicationAccess(session);
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Teaching courses" value={teacherCourses.length} />
        <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Application access" value={canApply ? "Available" : "Not needed"} />
        <SummaryCard icon={<BriefcaseBusiness size={20} aria-hidden />} label="Delegations" value={countDelegatedPermissions(session)} />
      </section>
      <ScopeList
        empty="Approved course teaching scopes will appear here."
        scopes={teacherCourses}
        title="Teaching scopes"
      />
    </>
  );
}

function OrganizationContent({ session }: { session: CurrentSession }) {
  const organizations = session.organizations.filter(hasOrganizationAccess);
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<Building2 size={20} aria-hidden />} label="Organizations" value={organizations.length} />
        <SummaryCard
          icon={<ShieldCheck size={20} aria-hidden />}
          label="Report scopes"
          value={organizations.filter((organization) => organization.effective_permissions.includes("VIEW_ORG_REWARD_REPORTS")).length}
        />
        <SummaryCard icon={<BriefcaseBusiness size={20} aria-hidden />} label="Delegations" value={countDelegatedPermissions(session)} />
      </section>
      <ScopeList
        empty="Organization memberships and delegated organization scopes will appear here."
        scopes={organizations}
        title="Organization scopes"
      />
    </>
  );
}

function AdminContent({ session }: { session: CurrentSession }) {
  const hasAdmin = hasPlatformAdminAccess(session);
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Platform admin" value={hasAdmin ? "Available" : "Unavailable"} />
        <SummaryCard icon={<BriefcaseBusiness size={20} aria-hidden />} label="Platform permissions" value={session.platform.effective_permissions.length} />
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Delegations" value={countDelegatedPermissions(session)} />
      </section>
      <ScopeList
        empty="Platform permissions will appear here after admin access is granted."
        scopes={[session.platform]}
        title="Platform scope"
      />
    </>
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

function ScopeList({
  empty,
  scopes,
  title,
}: {
  empty: string;
  scopes: Array<PlatformSessionScope & { id?: number; name?: string; title?: string; lifecycle_status?: string }>;
  title: string;
}) {
  return (
    <section className={styles.scopeList}>
      <div className={styles.scopeHeader}>
        <h2>{title}</h2>
        <StatusPill label={`${scopes.length} visible`} tone="neutral" />
      </div>
      {scopes.length ? (
        scopes.map((scope, index) => (
          <article className={styles.scopeCard} key={scope.id || title + index}>
            <h3>{scope.name || scope.title || "Platform"}</h3>
            {scope.lifecycle_status ? <p className={styles.muted}>{scope.lifecycle_status}</p> : null}
            <PermissionPreview scope={scope} />
          </article>
        ))
      ) : (
        <section className={styles.statePanel}>
          <p className={styles.muted}>{empty}</p>
        </section>
      )}
    </section>
  );
}

function PermissionPreview({ scope }: { scope: PlatformSessionScope }) {
  const visiblePermissions = scope.effective_permissions.slice(0, 6);
  const remaining = scope.effective_permissions.length - visiblePermissions.length;
  return (
    <div className={styles.permissionList}>
      {visiblePermissions.map((permission) => (
        <span key={permission}>{permission}</span>
      ))}
      {remaining > 0 ? <span>+{remaining}</span> : null}
      {scope.effective_permissions.length === 0 ? <span>No permissions</span> : null}
    </div>
  );
}

function StatusPill({
  icon,
  label,
  tone,
}: {
  icon?: ReactNode;
  label: string;
  tone: "good" | "neutral" | "warn";
}) {
  return (
    <span className={`${styles.statusPill} ${styles[tone]}`}>
      {icon}
      {label}
    </span>
  );
}

function isWorkspaceAllowed(kind: WorkspaceKind, access: ReturnType<typeof accessSummary>) {
  switch (kind) {
    case "learn":
      return access.learner;
    case "teach":
      return access.teacher;
    case "organizations":
      return access.organization;
    case "admin":
      return access.platformAdmin;
  }
}
