"use client";

import {
  AlertCircle,
  BookOpen,
  Building2,
  CheckCircle2,
  KeyRound,
  LogOut,
  RefreshCw,
  ShieldCheck,
  UserCircle,
} from "lucide-react";
import Link from "next/link";
import { type FormEvent, type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  type CourseSessionScope,
  type OrganizationSessionScope,
  type PlatformSessionScope,
  SessionRequestError,
  storeSessionToken,
} from "@/lib/session";
import styles from "./page.module.css";

type LoadState = "idle" | "loading" | "success" | "error";

const previewLimit = 4;

export default function SessionPage() {
  const [initialToken] = useState(() => readStoredSessionToken() || "");
  const [tokenInput, setTokenInput] = useState(initialToken);
  const [activeToken, setActiveToken] = useState(initialToken);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("idle");
  const [error, setError] = useState<{ code: string; message: string } | null>(null);

  const loadSession = useCallback(async (token: string) => {
    setLoadState("loading");
    setError(null);

    try {
      const nextSession = await fetchCurrentSession({ token });
      storeSessionToken(token);
      setActiveToken(token);
      setTokenInput(token);
      setSession(nextSession);
      setLoadState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof SessionRequestError
          ? nextError
          : new SessionRequestError("Session request failed.", 0, "network_error");
      setSession(null);
      setError({ code: requestError.code, message: requestError.message });
      setLoadState("error");
    }
  }, []);

  useEffect(() => {
    if (!initialToken) {
      return undefined;
    }

    const timeout = window.setTimeout(() => void loadSession(initialToken), 0);
    return () => window.clearTimeout(timeout);
  }, [initialToken, loadSession]);

  const workspaceCount = useMemo(() => {
    if (!session) {
      return 0;
    }

    return session.organizations.length + session.courses.length;
  }, [session]);

  function submitToken(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void loadSession(tokenInput);
  }

  function signOut() {
    clearStoredSessionToken();
    setActiveToken("");
    setTokenInput("");
    setSession(null);
    setError(null);
    setLoadState("idle");
  }

  return (
    <main className={styles.page}>
      <header className={styles.topbar}>
        <Link className={styles.brand} href="/">
          <span className={styles.brandMark}>RL</span>
          <span>
            <strong>RustLearn</strong>
            <small>Workspace</small>
          </span>
        </Link>
        <nav className={styles.topActions} aria-label="Session actions">
          <Link href="/" className={styles.navLink}>
            Operations
          </Link>
          {activeToken ? (
            <button className={styles.iconButton} type="button" onClick={signOut} aria-label="Sign out">
              <LogOut size={18} aria-hidden />
            </button>
          ) : null}
        </nav>
      </header>

      <section className={styles.hero}>
        <div className={styles.heroText}>
          <p className={styles.eyebrow}>Account</p>
          <h1>Current session</h1>
          <p>
            Profile, workspace scopes, and delegated permissions resolved from the API.
          </p>
        </div>
        <div className={styles.statusStrip} aria-live="polite">
          <StatusPill
            icon={<ShieldCheck size={16} aria-hidden />}
            label={session ? "Verified session" : "No active session"}
            tone={session ? "good" : "neutral"}
          />
          <StatusPill
            icon={<Building2 size={16} aria-hidden />}
            label={`${workspaceCount} workspaces`}
            tone="neutral"
          />
        </div>
      </section>

      <section className={styles.grid}>
        <form className={styles.panel} onSubmit={submitToken}>
          <div className={styles.panelHeader}>
            <KeyRound size={20} aria-hidden />
            <h2>Session token</h2>
          </div>
          <label className={styles.field}>
            <span>Bearer token</span>
            <textarea
              value={tokenInput}
              onChange={(event) => setTokenInput(event.target.value)}
              placeholder="Paste token from /api/auth/login"
              rows={5}
              spellCheck={false}
            />
          </label>
          <div className={styles.buttonRow}>
            <button className={styles.primaryButton} type="submit" disabled={loadState === "loading"}>
              {loadState === "loading" ? (
                <RefreshCw className={styles.spin} size={18} aria-hidden />
              ) : (
                <CheckCircle2 size={18} aria-hidden />
              )}
              Load session
            </button>
            {activeToken ? (
              <button className={styles.secondaryButton} type="button" onClick={() => void loadSession(activeToken)}>
                <RefreshCw size={18} aria-hidden />
                Refresh
              </button>
            ) : null}
          </div>
          {error ? (
            <div className={styles.errorBox} role="status">
              <AlertCircle size={18} aria-hidden />
              <span>
                <strong>{error.code}</strong>
                {error.message}
              </span>
            </div>
          ) : null}
        </form>

        <section className={styles.panel} aria-label="Current user">
          <div className={styles.panelHeader}>
            <UserCircle size={20} aria-hidden />
            <h2>Profile</h2>
          </div>
          {session ? (
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
          ) : (
            <EmptyState title="No profile loaded" detail="Load a session to resolve account access." />
          )}
        </section>
      </section>

      <section className={styles.workspaces}>
        <SectionTitle icon={<Building2 size={20} aria-hidden />} title="Organizations" />
        {session?.organizations.length ? (
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
        {session?.courses.length ? (
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
        {session?.delegated_permissions.length ? (
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
                <StatusPill label={delegation.expires_at ? "Expires" : "No expiry"} tone="neutral" />
              </article>
            ))}
          </div>
        ) : (
          <EmptyState title="No active delegations" detail="Temporary permissions will appear with scope context." />
        )}
      </section>
    </main>
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
