"use client";

import {
  AlertTriangle,
  ArrowLeft,
  Building2,
  CheckCircle2,
  CreditCard,
  FileText,
  Filter,
  Loader2,
  LogIn,
  RefreshCw,
  Search,
  Settings,
  ShieldCheck,
  Trophy,
  UserPlus,
  Users,
} from "lucide-react";
import Link from "next/link";
import { type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import {
  buildOrganizationWorkspace,
  enabledOrganizationCapabilities,
  filterOrganizationWorkspace,
  findOrganizationWorkspaceItem,
  missingOrganizationPermissions,
  type OrganizationCapability,
  type OrganizationCapabilityKey,
  type OrganizationWorkspaceItem,
  type OrganizationWorkspaceSummary,
} from "@/lib/organization";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  SessionRequestError,
} from "@/lib/session";
import styles from "./organization-routes.module.css";

type LoadState = "idle" | "loading" | "success" | "error";

type RouteError = {
  code: string;
  message: string;
  status: number;
};

type CapabilityFilter = OrganizationCapabilityKey | "all" | "delegated";

const capabilityFilters: Array<{ label: string; value: CapabilityFilter }> = [
  { label: "All access", value: "all" },
  { label: "Reports", value: "reports" },
  { label: "Members", value: "members" },
  { label: "Wallet", value: "wallet" },
  { label: "Teacher nominations", value: "teacher_applications" },
  { label: "Course rewards", value: "course_rewards" },
  { label: "Settings", value: "settings" },
  { label: "Delegated", value: "delegated" },
];

const actionIcons: Record<OrganizationCapabilityKey, ReactNode> = {
  course_rewards: <Trophy size={19} aria-hidden />,
  members: <Users size={19} aria-hidden />,
  reports: <FileText size={19} aria-hidden />,
  settings: <Settings size={19} aria-hidden />,
  teacher_applications: <UserPlus size={19} aria-hidden />,
  wallet: <CreditCard size={19} aria-hidden />,
};

const actionDescriptions: Record<OrganizationCapabilityKey, string> = {
  course_rewards: "Submit organization-backed course reward events when the route contract is added.",
  members: "Invite, review, and update organization members when member contracts are available.",
  reports: "Inspect reward reports and exports when report contracts are available.",
  settings: "Review scoped organization settings when settings contracts are available.",
  teacher_applications: "Nominate teachers and track sponsored applications when nomination contracts are available.",
  wallet: "Review budget, wallet state, and audit rows when wallet contracts are available.",
};

const emptyWorkspace: OrganizationWorkspaceSummary = {
  courseRewardScopeCount: 0,
  delegatedOrganizationCount: 0,
  managementScopeCount: 0,
  organizations: [],
  reportScopeCount: 0,
  teacherNominationScopeCount: 0,
  total: 0,
  walletScopeCount: 0,
};

export function OrganizationIndexRoute() {
  const route = useOrganizationSession();
  const [capability, setCapability] = useState<CapabilityFilter>("all");
  const [search, setSearch] = useState("");
  const workspace = useMemo(
    () => (route.session ? buildOrganizationWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const visibleOrganizations = useMemo(
    () => filterOrganizationWorkspace(workspace.organizations, { capability, search }),
    [capability, search, workspace.organizations],
  );
  const notice = organizationNotice(route.error);

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[{ href: "/session", label: "Workspace" }, { label: "Organizations" }]}
      description="Choose an organization workspace, inspect scoped permissions, and see which operator actions are ready for your session."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title="Organizations"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect="/organizations" /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect="/organizations" /> : null}
      {route.session && workspace.total === 0 ? <DeniedState /> : null}
      {route.session && workspace.total > 0 ? (
        <>
          <section className={styles.summaryGrid}>
            <SummaryCard icon={<Building2 size={20} aria-hidden />} label="Organizations" value={workspace.total} />
            <SummaryCard icon={<FileText size={20} aria-hidden />} label="Report scopes" value={workspace.reportScopeCount} />
            <SummaryCard icon={<Users size={20} aria-hidden />} label="Management scopes" value={workspace.managementScopeCount} />
            <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Delegated org access" value={workspace.delegatedOrganizationCount} />
          </section>
          <section className={styles.filterPanel} aria-label="Organization filters">
            <label>
              <span>Search organizations</span>
              <span className={styles.inputWithIcon}>
                <Search size={17} aria-hidden />
                <input
                  onChange={(event) => setSearch(event.target.value)}
                  placeholder="Name, role, or permission"
                  type="search"
                  value={search}
                />
              </span>
            </label>
            <label>
              <span>Capability</span>
              <select
                aria-label="Organization capability filter"
                onChange={(event) => setCapability(event.target.value as CapabilityFilter)}
                value={capability}
              >
                {capabilityFilters.map((filter) => (
                  <option key={filter.value} value={filter.value}>
                    {filter.label}
                  </option>
                ))}
              </select>
            </label>
            <button
              className={styles.secondaryButton}
              onClick={() => {
                setCapability("all");
                setSearch("");
              }}
              type="button"
            >
              <RefreshCw size={17} aria-hidden />
              Reset
            </button>
          </section>
          <OrganizationList organizations={visibleOrganizations} total={workspace.organizations.length} />
        </>
      ) : null}
    </ProductShell>
  );
}

export function OrganizationDashboardRoute({ organizationId }: { organizationId: string }) {
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
  const notice = organizationNotice(route.error);

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        { label: organization?.name || "Organization" },
      ]}
      description="Organization permissions, delegated access, and action readiness for the selected workspace."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name || "Organization workspace"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect="/organizations" /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect="/organizations" /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization ? <OrganizationDashboard organization={organization} /> : null}
    </ProductShell>
  );
}

function OrganizationDashboard({ organization }: { organization: OrganizationWorkspaceItem }) {
  const enabledCapabilities = enabledOrganizationCapabilities(organization);
  const deniedCapabilities = organization.capabilities.filter((capability) => !capability.enabled);

  return (
    <>
      <section className={styles.workspaceHero}>
        <div className={styles.workspaceTitleBlock}>
          <Link className={styles.backLink} href="/organizations">
            <ArrowLeft size={17} aria-hidden />
            Organizations
          </Link>
          <p className={styles.eyebrow}>Selected workspace</p>
          <h2>{organization.name}</h2>
          <p className={styles.muted}>
            This view is resolved from your current session. Member lists, report rows, wallet audit,
            and nomination history remain backend-contract work before those screens can become
            full workflows.
          </p>
        </div>
        <div className={styles.permissionRow}>
          {organization.roles.length ? (
            organization.roles.map((role) => <span className={styles.permissionChip} key={role}>{role}</span>)
          ) : (
            <span className={styles.permissionChip}>No role label</span>
          )}
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Effective permissions" value={organization.effectivePermissionCount} />
        <SummaryCard icon={<Building2 size={20} aria-hidden />} label="Direct permissions" value={organization.directPermissionCount} />
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Available actions" value={enabledCapabilities.length} />
        <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Delegated permissions" value={organization.delegatedPermissionCount} />
      </section>

      <section className={styles.twoColumn}>
        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Available quick actions</h2>
            <StatusPill label={`${enabledCapabilities.length} enabled`} tone={enabledCapabilities.length ? "good" : "neutral"} />
          </div>
          {enabledCapabilities.length ? (
            <div className={styles.actionGrid}>
              {enabledCapabilities.map((capability) => (
                <ActionCard capability={capability} enabled key={capability.key} />
              ))}
            </div>
          ) : (
            <p className={styles.muted}>
              Your session can see this organization, but no management capability is enabled.
            </p>
          )}
        </section>

        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Permission preview</h2>
            <StatusPill label={`${organization.effectivePermissionCount} effective`} tone="neutral" />
          </div>
          <PermissionPreview organization={organization} />
        </section>
      </section>

      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Denied actions</h2>
          <StatusPill label={`${deniedCapabilities.length} gated`} tone={deniedCapabilities.length ? "warn" : "good"} />
        </div>
        <div className={styles.actionGrid}>
          {deniedCapabilities.map((capability) => (
            <ActionCard capability={capability} enabled={false} key={capability.key} />
          ))}
        </div>
      </section>
    </>
  );
}

function OrganizationList({
  organizations,
  total,
}: {
  organizations: OrganizationWorkspaceItem[];
  total: number;
}) {
  return (
    <section className={styles.organizationSection}>
      <div className={styles.sectionHeader}>
        <h2>Visible organizations</h2>
        <StatusPill label={`${organizations.length} of ${total}`} tone="neutral" />
      </div>
      {organizations.length ? (
        <div className={styles.organizationGrid}>
          {organizations.map((organization) => (
            <article className={styles.organizationCard} key={organization.id}>
              <div className={styles.organizationTop}>
                <span className={styles.smallIcon}>
                  <Building2 size={18} aria-hidden />
                </span>
                <div>
                  <h3>{organization.name}</h3>
                  <p className={styles.muted}>{workspaceSubtitle(organization)}</p>
                </div>
              </div>
              <div className={styles.metricGrid}>
                <Metric label="Permissions" value={organization.effectivePermissionCount} />
                <Metric label="Direct" value={organization.directPermissionCount} />
                <Metric label="Delegated" value={organization.delegatedPermissionCount} />
              </div>
              <CapabilityRow organization={organization} />
              <div className={styles.permissionRow}>
                {organization.roles.length ? (
                  organization.roles.slice(0, 3).map((role) => <span className={styles.permissionChip} key={role}>{role}</span>)
                ) : (
                  <span className={styles.permissionChip}>No role label</span>
                )}
              </div>
              <Link className={styles.primaryLink} href={`/organizations/${organization.id}`}>
                <Building2 size={18} aria-hidden />
                Open dashboard
              </Link>
            </article>
          ))}
        </div>
      ) : (
        <section className={styles.panel}>
          <div className={styles.panelHeader}>
            <Filter size={20} aria-hidden />
            <h2>No matching organizations</h2>
          </div>
          <p className={styles.muted}>
            Adjust the search or capability filter. Your underlying organization access was not
            changed.
          </p>
        </section>
      )}
    </section>
  );
}

function ActionCard({
  capability,
  enabled,
}: {
  capability: OrganizationCapability;
  enabled: boolean;
}) {
  return (
    <article className={`${styles.actionCard} ${enabled ? styles.enabledAction : styles.deniedAction}`}>
      <div className={styles.actionHeader}>
        <span className={styles.smallIcon}>{actionIcons[capability.key]}</span>
        <div>
          <h3>{capability.label}</h3>
          <p>{actionDescriptions[capability.key]}</p>
        </div>
      </div>
      {enabled ? (
        <span className={styles.statusPill}>
          <CheckCircle2 size={16} aria-hidden />
          Permission available
        </span>
      ) : (
        <div className={styles.missingList}>
          <strong>Missing scoped permission</strong>
          {missingOrganizationPermissions(capability).slice(0, 3).map((permission) => (
            <span key={permission}>{permission}</span>
          ))}
        </div>
      )}
      <button className={styles.secondaryButton} disabled type="button">
        Contract pending
      </button>
    </article>
  );
}

function CapabilityRow({ organization }: { organization: OrganizationWorkspaceItem }) {
  const enabledCapabilities = enabledOrganizationCapabilities(organization);

  return (
    <div className={styles.capabilityRow} aria-label={`${organization.name} capabilities`}>
      {enabledCapabilities.length ? (
        enabledCapabilities.slice(0, 4).map((capability) => (
          <span className={styles.statusPill} key={capability.key}>
            {capability.label}
          </span>
        ))
      ) : (
        <span className={styles.statusPill}>View-only membership</span>
      )}
      {enabledCapabilities.length > 4 ? <span className={styles.statusPill}>+{enabledCapabilities.length - 4}</span> : null}
    </div>
  );
}

function PermissionPreview({ organization }: { organization: OrganizationWorkspaceItem }) {
  const permissions = organization.permissionPreview;
  const remaining = organization.effectivePermissionCount - permissions.length;

  return (
    <div className={styles.permissionList}>
      {permissions.length ? (
        permissions.map((permission) => <span key={permission}>{permission}</span>)
      ) : (
        <span>No effective permissions resolved</span>
      )}
      {remaining > 0 ? <span>+{remaining} more</span> : null}
    </div>
  );
}

function OrganizationStatus({ workspace }: { workspace: OrganizationWorkspaceSummary }) {
  return (
    <>
      <StatusPill
        icon={<Building2 size={16} aria-hidden />}
        label={`${workspace.total} organization${workspace.total === 1 ? "" : "s"}`}
        tone={workspace.total ? "good" : "neutral"}
      />
      <StatusPill
        icon={<FileText size={16} aria-hidden />}
        label={`${workspace.reportScopeCount} report scope${workspace.reportScopeCount === 1 ? "" : "s"}`}
        tone={workspace.reportScopeCount ? "good" : "neutral"}
      />
      <StatusPill
        icon={<ShieldCheck size={16} aria-hidden />}
        label={`${workspace.delegatedOrganizationCount} delegated`}
        tone={workspace.delegatedOrganizationCount ? "warn" : "neutral"}
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
      <span className={styles.smallIcon}>{icon}</span>
      <strong>{value}</strong>
      <span>{label}</span>
    </article>
  );
}

function Metric({ label, value }: { label: string; value: number | string }) {
  return (
    <span className={styles.metricCard}>
      <strong>{value}</strong>
      <span>{label}</span>
    </span>
  );
}

function SignedOutState({ redirect }: { redirect: string }) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <LogIn size={20} aria-hidden />
        <h2>Sign in required</h2>
      </div>
      <p className={styles.muted}>
        Organization workspaces are shown after RustLearn resolves your current account, memberships,
        and scoped permissions.
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
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Resolving organization access</h2>
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
  error: RouteError;
  redirect: string;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error.code}</strong>
        <span>{error.message}</span>
      </span>
      <Link className={styles.secondaryLink} href={`/login?redirect=${redirect}`}>
        Return to login
      </Link>
    </section>
  );
}

function DeniedState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>No organization workspace yet</h2>
      </div>
      <p className={styles.muted}>
        This account does not currently include organization roles, direct organization
        permissions, delegated organization permissions, or effective organization permissions.
      </p>
      <Link className={styles.secondaryLink} href="/session">
        Review current session
      </Link>
    </section>
  );
}

function MissingOrganizationState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Organization unavailable</h2>
      </div>
      <p className={styles.muted}>
        This organization is not visible to your current session. It may have been removed,
        renamed, or your scoped access may have changed.
      </p>
      <Link className={styles.secondaryLink} href="/organizations">
        Back to organizations
      </Link>
    </section>
  );
}

function StatusPill({
  icon,
  label,
  tone = "neutral",
}: {
  icon?: ReactNode;
  label: string;
  tone?: "good" | "neutral" | "warn";
}) {
  return (
    <span className={`${styles.statusPill} ${styles[tone]}`}>
      {icon}
      {label}
    </span>
  );
}

function useOrganizationSession() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const loadSession = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setError(null);
      setHasToken(false);
      setLoadState("idle");
      setSession(null);
      return;
    }

    setError(null);
    setHasToken(true);
    setLoadState("loading");

    try {
      const nextSession = await fetchCurrentSession({ token });
      setSession(nextSession);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setError(routeError);
      setLoadState("error");
      setSession(null);
    }
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  function signOut() {
    clearStoredSessionToken();
    setError(null);
    setHasToken(false);
    setLoadState("idle");
    setSession(null);
  }

  return {
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
  };
}

function normalizeRouteError(error: unknown): RouteError {
  if (error instanceof SessionRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "network_error",
    message: "Organization workspace could not be loaded.",
    status: 0,
  };
}

function organizationNotice(error: RouteError | null): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user") {
    return {
      actionHref: "/login?redirect=/organizations",
      actionLabel: "Sign in",
      message: "Your stored session is no longer valid. Sign in again to continue.",
      title: "Session expired",
      tone: "error",
    };
  }

  return {
    message: error.message,
    title: "Organization workspace status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}

function workspaceSubtitle(organization: OrganizationWorkspaceItem) {
  const enabledCount = enabledOrganizationCapabilities(organization).length;
  if (enabledCount > 0) {
    return `${enabledCount} action ${enabledCount === 1 ? "capability" : "capabilities"} available`;
  }

  if (organization.delegatedPermissionCount > 0) {
    return "Delegated organization access";
  }

  return "Organization membership visible";
}
