"use client";

import {
  AlertCircle,
  BookOpen,
  Building2,
  Loader2,
  LogIn,
  LogOut,
  RefreshCw,
  ShieldCheck,
  UserCircle,
} from "lucide-react";
import Link from "next/link";
import { type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  type CourseSessionScope,
  type OrganizationSessionScope,
  type PlatformSessionScope,
  SessionRequestError,
} from "@/lib/session";
import styles from "./page.module.css";

type LoadState = "idle" | "loading" | "success" | "error";

const previewLimit = 4;

export default function SessionPage() {
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
          : new SessionRequestError("Session request failed.", 0, "network_error");
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

  const workspaceCount = useMemo(() => {
    if (!session) {
      return 0;
    }

    return session.organizations.length + session.courses.length;
  }, [session]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setError(null);
    setLoadState("idle");
  }

  const notice = sessionNotice(error);

  return (
    <ProductShell
      activeNav="session"
      breadcrumbs={[{ label: "Workspace" }]}
      description="Profile, workspace scopes, and delegated permissions resolved from the API."
      eyebrow="Account"
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={
        <>
          <StatusPill
            icon={<ShieldCheck size={16} aria-hidden />}
            label={
              loadState === "loading"
                ? "Resolving session"
                : session
                  ? "Verified session"
                  : "No active session"
            }
            tone={session ? "good" : loadState === "error" ? "warn" : "neutral"}
          />
          <StatusPill
            icon={<Building2 size={16} aria-hidden />}
            label={`${workspaceCount} workspaces`}
            tone="neutral"
          />
        </>
      }
      title="Current session"
    >
      {loadState === "idle" && !session ? (
        <section className={`${styles.panel} ${styles.singlePanel}`}>
          <div className={styles.panelHeader}>
            <LogIn size={20} aria-hidden />
            <h2>Sign in required</h2>
          </div>
          <p className={styles.muted}>
            The workspace loads from the session created by the login flow. No manual token
            paste is needed on product routes.
          </p>
          <Link className={styles.primaryLink} href="/login?redirect=/session">
            <LogIn size={18} aria-hidden />
            Sign in
          </Link>
        </section>
      ) : null}

      {loadState === "loading" ? (
        <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
          <div className={styles.panelHeader}>
            <Loader2 className={styles.spin} size={20} aria-hidden />
            <h2>Loading workspace</h2>
          </div>
          <p className={styles.muted}>
            Resolving your account, permissions, organizations, courses, and delegated access.
          </p>
        </section>
      ) : null}

      {error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <Link className={styles.secondaryLink} href="/login?redirect=/session">
            Return to login
          </Link>
        </section>
      ) : null}

      {session ? (
        <>
          <section className={styles.grid}>
            <section className={styles.panel} aria-label="Workspace status">
              <div className={styles.panelHeader}>
                <ShieldCheck size={20} aria-hidden />
                <h2>Access status</h2>
              </div>
              <p className={styles.muted}>
                Use this page to inspect the account and workspace scopes that product routes use
                for navigation and permission gates.
              </p>
              <div className={styles.buttonRow}>
                <button
                  className={styles.secondaryButton}
                  type="button"
                  onClick={() => void loadSession()}
                >
                  <RefreshCw size={18} aria-hidden />
                  Refresh
                </button>
                <button className={styles.secondaryButton} type="button" onClick={signOut}>
                  <LogOut size={18} aria-hidden />
                  Sign out
                </button>
              </div>
            </section>

            <section className={styles.panel} aria-label="Current user">
              <div className={styles.panelHeader}>
                <UserCircle size={20} aria-hidden />
                <h2>Profile</h2>
              </div>
              <div className={styles.profileBlock}>
                <div>
                  <p className={styles.profileName}>{session.user.name}</p>
                  <p className={styles.muted}>{session.user.email}</p>
                </div>
                <div className={styles.badgeRow}>
                  <StatusPill
                    label={session.user.email_verified ? "Email verified" : "Email pending"}
                    tone={session.user.email_verified ? "good" : "warn"}
                  />
                  <StatusPill
                    label={session.user.kyc_verified ? "KYC verified" : "KYC pending"}
                    tone={session.user.kyc_verified ? "good" : "neutral"}
                  />
                </div>
                <ScopeSummary title="Platform" scope={session.platform} />
              </div>
            </section>
          </section>

          <section className={styles.workspaces}>
            <SectionTitle icon={<Building2 size={20} aria-hidden />} title="Organizations" />
            {session.organizations.length ? (
              <div className={styles.cardGrid}>
                {session.organizations.map((organization) => (
                  <OrganizationCard key={organization.id} organization={organization} />
                ))}
              </div>
            ) : (
              <EmptyState title="No organization access" detail="Organization scopes will appear here." />
            )}
          </section>

          <section className={styles.workspaces}>
            <SectionTitle icon={<BookOpen size={20} aria-hidden />} title="Courses" />
            {session.courses.length ? (
              <div className={styles.cardGrid}>
                {session.courses.map((course) => (
                  <CourseCard key={course.id} course={course} />
                ))}
              </div>
            ) : (
              <EmptyState title="No course access" detail="Course enrollments and teaching scopes will appear here." />
            )}
          </section>

          <section className={styles.workspaces}>
            <SectionTitle icon={<ShieldCheck size={20} aria-hidden />} title="Delegated permissions" />
            {session.delegated_permissions.length ? (
              <div className={styles.delegationList}>
                {session.delegated_permissions.map((delegation) => (
                  <article className={styles.delegationItem} key={delegation.id}>
                    <div>
                      <strong>{delegation.permission}</strong>
                      <p className={styles.muted}>
                        {delegation.organization_name ||
                          delegation.course_title ||
                          delegation.scope_type}
                      </p>
                    </div>
                    <StatusPill
                      label={delegation.expires_at ? expiryLabel(delegation.expires_at) : "No expiry"}
                      tone="neutral"
                    />
                  </article>
                ))}
              </div>
            ) : (
              <EmptyState title="No active delegations" detail="Temporary permissions will appear with scope context." />
            )}
          </section>
        </>
      ) : null}
    </ProductShell>
  );
}

function SectionTitle({ icon, title }: { icon: ReactNode; title: string }) {
  return (
    <div className={styles.sectionTitle}>
      {icon}
      <h2>{title}</h2>
    </div>
  );
}

function OrganizationCard({ organization }: { organization: OrganizationSessionScope }) {
  return (
    <article className={styles.scopeCard}>
      <h3>{organization.name}</h3>
      <p className={styles.muted}>Organization #{organization.id}</p>
      <ScopeSummary title="Access" scope={organization} />
    </article>
  );
}

function CourseCard({ course }: { course: CourseSessionScope }) {
  return (
    <article className={styles.scopeCard}>
      <h3>{course.title}</h3>
      <p className={styles.muted}>{course.lifecycle_status}</p>
      <ScopeSummary title="Access" scope={course} />
    </article>
  );
}

function ScopeSummary({ title, scope }: { title: string; scope: PlatformSessionScope }) {
  const visiblePermissions = scope.effective_permissions.slice(0, previewLimit);
  const remainingCount = scope.effective_permissions.length - visiblePermissions.length;

  return (
    <div className={styles.scopeSummary}>
      <p className={styles.summaryTitle}>{title}</p>
      <div className={styles.badgeRow}>
        {scope.roles.map((role) => (
          <span className={styles.roleBadge} key={role}>
            {role}
          </span>
        ))}
        {scope.roles.length === 0 ? <span className={styles.roleBadge}>Delegated</span> : null}
      </div>
      <div className={styles.permissionList}>
        {visiblePermissions.map((permission) => (
          <span key={permission}>{permission}</span>
        ))}
        {remainingCount > 0 ? <span>+{remainingCount}</span> : null}
        {scope.effective_permissions.length === 0 ? <span>No permissions</span> : null}
      </div>
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

function EmptyState({ title, detail }: { title: string; detail: string }) {
  return (
    <div className={styles.emptyState}>
      <p>{title}</p>
      <span>{detail}</span>
    </div>
  );
}

function sessionNotice(error: { code: string; message: string } | null): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user") {
    return {
      actionHref: "/login?redirect=/session",
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

function expiryLabel(expiresAt: string) {
  return `Expires ${expiresAt.replace("T", " ").slice(0, 16)}`;
}
