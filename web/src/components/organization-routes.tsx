"use client";

import {
  AlertTriangle,
  ArrowLeft,
  BookOpen,
  Building2,
  CheckCircle2,
  CreditCard,
  Download,
  ExternalLink,
  FileText,
  Filter,
  Loader2,
  LogIn,
  RefreshCw,
  Search,
  Send,
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
  deleteOrganization,
  downloadOrganizationRewardDashboardCsv,
  enabledOrganizationCapabilities,
  fetchOrganization,
  fetchOrganizationCourses,
  fetchOrganizationDashboard,
  fetchOrganizationWalletAudit,
  fetchOrganizationMembers,
  fetchOrganizationRewardDashboard,
  fetchOrganizationTeacherApplications,
  filterOrganizationWorkspace,
  findOrganizationWorkspaceItem,
  linkOrganizationWallet,
  missingOrganizationPermissions,
  OrganizationRequestError,
  updateOrganization,
  type OrganizationCapability,
  type OrganizationCapabilityKey,
  type OrganizationCourseList,
  type OrganizationCourseListItem,
  type OrganizationDashboardAlert,
  type OrganizationDashboardSummary,
  type OrganizationDetail,
  type OrganizationMemberList,
  type OrganizationMemberListItem,
  type OrganizationRewardDashboard,
  type OrganizationTeacherApplicationItem,
  type OrganizationTeacherApplicationList,
  type OrganizationWalletAudit,
  type OrganizationWalletRewardRecordAudit,
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
type CsvState = "idle" | "downloading" | "success" | "error";
type ReportLoadState = "idle" | "loading" | "success" | "error";
type CourseLoadState = "idle" | "loading" | "success" | "error";
type MemberLoadState = "idle" | "loading" | "success" | "error";
type TeacherApplicationLoadState = "idle" | "loading" | "success" | "error";
type DashboardLoadState = "idle" | "loading" | "success" | "error";
type WalletLoadState = "idle" | "loading" | "success" | "missing" | "error";
type WalletLinkState = "idle" | "linking" | "success" | "error";
type SettingsLoadState = "idle" | "loading" | "success" | "error";
type SettingsSaveState = "idle" | "saving" | "success" | "error";

const capabilityFilters: Array<{ label: string; value: CapabilityFilter }> = [
  { label: "All access", value: "all" },
  { label: "Courses", value: "courses" },
  { label: "Reports", value: "reports" },
  { label: "Members", value: "members" },
  { label: "Wallet", value: "wallet" },
  { label: "Teacher nominations", value: "teacher_applications" },
  { label: "Course rewards", value: "course_rewards" },
  { label: "Settings", value: "settings" },
  { label: "Delegated", value: "delegated" },
];

const actionIcons: Record<OrganizationCapabilityKey, ReactNode> = {
  courses: <BookOpen size={19} aria-hidden />,
  course_rewards: <Trophy size={19} aria-hidden />,
  members: <Users size={19} aria-hidden />,
  reports: <FileText size={19} aria-hidden />,
  settings: <Settings size={19} aria-hidden />,
  teacher_applications: <UserPlus size={19} aria-hidden />,
  wallet: <CreditCard size={19} aria-hidden />,
};

const actionDescriptions: Record<OrganizationCapabilityKey, string> = {
  courses: "Inspect sponsored courses, lifecycle, teacher coverage, enrollment pressure, and reward policy status.",
  course_rewards: "Submit organization-backed course reward events when the route contract is added.",
  members: "Inspect organization members, role labels, scoped permissions, and management readiness.",
  reports: "Inspect reward volume, sponsored applications, wallet balances, and CSV exports.",
  settings: "Manage organization name, website, profile URL, and destructive actions scoped to operator permissions.",
  teacher_applications: "Track sponsored teacher applications, decisions, applicant context, and audit hints.",
  wallet: "Review wallet balance, budget readiness, reward credits, token links, and audit rows.",
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

const organizationCourseLifecycleOptions = [
  { label: "All lifecycles", value: "" },
  { label: "Draft", value: "draft" },
  { label: "Submitted", value: "submitted" },
  { label: "Needs changes", value: "needs_changes" },
  { label: "Approved", value: "approved" },
  { label: "Published", value: "published" },
  { label: "Suspended", value: "suspended" },
  { label: "Archived", value: "archived" },
];

const ORGANIZATION_COURSE_PAGE_SIZE = 6;
const ORGANIZATION_MEMBER_PAGE_SIZE = 8;
const ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE = 6;

const organizationTeacherApplicationStatusOptions = [
  { label: "All statuses", value: "" },
  { label: "Submitted", value: "submitted" },
  { label: "Needs changes", value: "needs_changes" },
  { label: "Approved", value: "approved" },
  { label: "Rejected", value: "rejected" },
];

const organizationMemberRoleOptions = [
  { label: "All roles", value: "" },
  { label: "Super admin", value: "SUPER_ADMIN" },
  { label: "Admin", value: "ADMIN" },
  { label: "Moderator", value: "MODERATOR" },
  { label: "Teacher", value: "TEACHER" },
  { label: "Student", value: "STUDENT" },
];

const organizationMemberPermissionOptions = [
  { label: "All permissions", value: "" },
  { label: "Can view organization", value: "VIEW_ORGANIZATION" },
  { label: "Can invite members", value: "INVITE_USER_TO_ORGANIZATION" },
  { label: "Can manage members", value: "MANAGE_ORG_MEMBERS" },
  { label: "Can assign roles", value: "ASSIGN_ROLES_TO_ORG_USERS" },
  { label: "Can view reward reports", value: "VIEW_ORG_REWARD_REPORTS" },
  { label: "Can manage wallets", value: "MANAGE_ORG_WALLETS" },
];

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
  const [dashboard, setDashboard] = useState<OrganizationDashboardSummary | null>(null);
  const [dashboardError, setDashboardError] = useState<RouteError | null>(null);
  const [dashboardLoadState, setDashboardLoadState] = useState<DashboardLoadState>("idle");
  const notice = organizationNotice(route.error || dashboardError);

  const loadDashboard = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization) {
      return;
    }

    setDashboardError(null);
    setDashboardLoadState("loading");

    try {
      const nextDashboard = await fetchOrganizationDashboard({
        organizationId: organization.id,
        token,
      });
      setDashboard(nextDashboard);
      setDashboardLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
      }
      setDashboard(null);
      setDashboardError(routeError);
      setDashboardLoadState("error");
    }
  }, [invalidOrganizationId, organization]);

  useEffect(() => {
    if (route.session && organization) {
      const timeout = window.setTimeout(() => {
        setDashboard(null);
        setDashboardError(null);
        setDashboardLoadState("idle");
        void loadDashboard();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [loadDashboard, organization, route.session]);

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
      {route.session && organization ? (
        <OrganizationDashboard
          dashboard={dashboard}
          dashboardError={dashboardError}
          dashboardLoadState={dashboardLoadState}
          onRefreshDashboard={loadDashboard}
          organization={organization}
        />
      ) : null}
    </ProductShell>
  );
}

export function OrganizationMembersRoute({ organizationId }: { organizationId: string }) {
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
  const membersCapability = organization?.capabilities.find((capability) => capability.key === "members");
  const canViewMembers = Boolean(membersCapability?.enabled);
  const [draftSearch, setDraftSearch] = useState("");
  const [memberError, setMemberError] = useState<RouteError | null>(null);
  const [memberLoadState, setMemberLoadState] = useState<MemberLoadState>("idle");
  const [members, setMembers] = useState<OrganizationMemberList | null>(null);
  const [page, setPage] = useState(0);
  const [permissionFilter, setPermissionFilter] = useState("");
  const [roleFilter, setRoleFilter] = useState("");
  const [search, setSearch] = useState("");
  const notice = organizationNotice(route.error || memberError);

  const loadMembers = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canViewMembers) {
      return;
    }

    setMemberError(null);
    setMemberLoadState("loading");

    try {
      const nextMembers = await fetchOrganizationMembers({
        limit: ORGANIZATION_MEMBER_PAGE_SIZE,
        offset: page * ORGANIZATION_MEMBER_PAGE_SIZE,
        organizationId: organization.id,
        permission: permissionFilter,
        role: roleFilter,
        search,
        token,
      });
      setMembers(nextMembers);
      setMemberLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
      }
      setMembers(null);
      setMemberError(routeError);
      setMemberLoadState("error");
    }
  }, [canViewMembers, invalidOrganizationId, organization, page, permissionFilter, roleFilter, search]);

  useEffect(() => {
    if (route.session && organization && canViewMembers) {
      const timeout = window.setTimeout(() => {
        setMemberError(null);
        setMemberLoadState("idle");
        setMembers(null);
        void loadMembers();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canViewMembers, loadMembers, organization, route.session]);

  function applyFilters() {
    setPage(0);
    setSearch(draftSearch);
  }

  function resetFilters() {
    setDraftSearch("");
    setPage(0);
    setPermissionFilter("");
    setRoleFilter("");
    setSearch("");
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: organization ? `/organizations/${organization.id}` : undefined,
          label: organization?.name || "Organization",
        },
        { label: "Members" },
      ]}
      description="Organization members, role labels, direct and delegated scoped permissions, and operator action readiness."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} members` : "Organization members"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/members`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/members`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canViewMembers ? (
        <MembersDeniedState capability={membersCapability} organizationName={organization.name} />
      ) : null}
      {route.session && organization && canViewMembers ? (
        <OrganizationMembersContent
          draftSearch={draftSearch}
          loadState={memberLoadState}
          members={members}
          onApplyFilters={applyFilters}
          onDraftSearchChange={setDraftSearch}
          onPageChange={setPage}
          onPermissionFilterChange={(nextPermission) => {
            setPermissionFilter(nextPermission);
            setPage(0);
          }}
          onRefresh={loadMembers}
          onResetFilters={resetFilters}
          onRoleFilterChange={(nextRole) => {
            setRoleFilter(nextRole);
            setPage(0);
          }}
          organization={organization}
          page={page}
          permissionFilter={permissionFilter}
          roleFilter={roleFilter}
          routeError={memberError}
        />
      ) : null}
    </ProductShell>
  );
}

export function OrganizationCoursesRoute({ organizationId }: { organizationId: string }) {
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
  const coursesCapability = organization?.capabilities.find((capability) => capability.key === "courses");
  const canViewCourses = Boolean(coursesCapability?.enabled);
  const [courseError, setCourseError] = useState<RouteError | null>(null);
  const [courseLoadState, setCourseLoadState] = useState<CourseLoadState>("idle");
  const [courses, setCourses] = useState<OrganizationCourseList | null>(null);
  const [draftSearch, setDraftSearch] = useState("");
  const [lifecycleStatus, setLifecycleStatus] = useState("");
  const [page, setPage] = useState(0);
  const [rewardFilter, setRewardFilter] = useState<"all" | "rewarded" | "unrewarded">("all");
  const [search, setSearch] = useState("");
  const notice = organizationNotice(route.error || courseError);

  const loadCourses = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canViewCourses) {
      return;
    }

    setCourseError(null);
    setCourseLoadState("loading");

    try {
      const nextCourses = await fetchOrganizationCourses({
        lifecycleStatus,
        limit: ORGANIZATION_COURSE_PAGE_SIZE,
        offset: page * ORGANIZATION_COURSE_PAGE_SIZE,
        organizationId: organization.id,
        rewardAvailable:
          rewardFilter === "all" ? null : rewardFilter === "rewarded",
        search,
        token,
      });
      setCourses(nextCourses);
      setCourseLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
      }
      setCourses(null);
      setCourseError(routeError);
      setCourseLoadState("error");
    }
  }, [canViewCourses, invalidOrganizationId, lifecycleStatus, organization, page, rewardFilter, search]);

  useEffect(() => {
    if (route.session && organization && canViewCourses) {
      const timeout = window.setTimeout(() => {
        setCourseError(null);
        setCourseLoadState("idle");
        setCourses(null);
        void loadCourses();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canViewCourses, loadCourses, organization, route.session]);

  function applyFilters() {
    setPage(0);
    setSearch(draftSearch);
  }

  function resetFilters() {
    setDraftSearch("");
    setLifecycleStatus("");
    setPage(0);
    setRewardFilter("all");
    setSearch("");
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: organization ? `/organizations/${organization.id}` : undefined,
          label: organization?.name || "Organization",
        },
        { label: "Courses" },
      ]}
      description="Organization-sponsored courses, lifecycle state, teacher coverage, enrollment pressure, content readiness, and reward policy status."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} courses` : "Organization courses"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/courses`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/courses`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canViewCourses ? (
        <CoursesDeniedState capability={coursesCapability} organizationName={organization.name} />
      ) : null}
      {route.session && organization && canViewCourses ? (
        <OrganizationCoursesContent
          courses={courses}
          draftSearch={draftSearch}
          lifecycleStatus={lifecycleStatus}
          loadState={courseLoadState}
          onApplyFilters={applyFilters}
          onDraftSearchChange={setDraftSearch}
          onLifecycleStatusChange={(nextStatus) => {
            setLifecycleStatus(nextStatus);
            setPage(0);
          }}
          onPageChange={setPage}
          onRefresh={loadCourses}
          onResetFilters={resetFilters}
          onRewardFilterChange={(nextFilter) => {
            setRewardFilter(nextFilter);
            setPage(0);
          }}
          organization={organization}
          page={page}
          routeError={courseError}
          rewardFilter={rewardFilter}
        />
      ) : null}
    </ProductShell>
  );
}

export function OrganizationTeacherApplicationsRoute({ organizationId }: { organizationId: string }) {
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
  const teacherApplicationCapability = organization?.capabilities.find(
    (capability) => capability.key === "teacher_applications",
  );
  const canTrackTeacherApplications = Boolean(teacherApplicationCapability?.enabled);
  const [applicationError, setApplicationError] = useState<RouteError | null>(null);
  const [applicationLoadState, setApplicationLoadState] = useState<TeacherApplicationLoadState>("idle");
  const [applications, setApplications] = useState<OrganizationTeacherApplicationList | null>(null);
  const [draftSearch, setDraftSearch] = useState("");
  const [page, setPage] = useState(0);
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const notice = organizationNotice(route.error || applicationError);

  const loadApplications = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canTrackTeacherApplications) {
      return;
    }

    setApplicationError(null);
    setApplicationLoadState("loading");

    try {
      const nextApplications = await fetchOrganizationTeacherApplications({
        limit: ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE,
        offset: page * ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE,
        organizationId: organization.id,
        search,
        status: statusFilter,
        token,
      });
      setApplications(nextApplications);
      setApplicationLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
      }
      setApplications(null);
      setApplicationError(routeError);
      setApplicationLoadState("error");
    }
  }, [canTrackTeacherApplications, invalidOrganizationId, organization, page, search, statusFilter]);

  useEffect(() => {
    if (route.session && organization && canTrackTeacherApplications) {
      const timeout = window.setTimeout(() => {
        setApplicationError(null);
        setApplicationLoadState("idle");
        setApplications(null);
        void loadApplications();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canTrackTeacherApplications, loadApplications, organization, route.session]);

  function applyFilters() {
    setPage(0);
    setSearch(draftSearch);
  }

  function resetFilters() {
    setDraftSearch("");
    setPage(0);
    setSearch("");
    setStatusFilter("");
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: organization ? `/organizations/${organization.id}` : undefined,
          label: organization?.name || "Organization",
        },
        { label: "Teacher nominations" },
      ]}
      description="Organization-sponsored teacher application tracking with applicant context, status filters, and audit hints."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} teacher nominations` : "Teacher nominations"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/teacher-applications`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/teacher-applications`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canTrackTeacherApplications ? (
        <TeacherApplicationsDeniedState
          capability={teacherApplicationCapability}
          organizationName={organization.name}
        />
      ) : null}
      {route.session && organization && canTrackTeacherApplications ? (
        <OrganizationTeacherApplicationsContent
          applications={applications}
          draftSearch={draftSearch}
          loadState={applicationLoadState}
          onApplyFilters={applyFilters}
          onDraftSearchChange={setDraftSearch}
          onPageChange={setPage}
          onRefresh={loadApplications}
          onResetFilters={resetFilters}
          onStatusFilterChange={(nextStatus) => {
            setStatusFilter(nextStatus);
            setPage(0);
          }}
          organization={organization}
          page={page}
          routeError={applicationError}
          statusFilter={statusFilter}
        />
      ) : null}
    </ProductShell>
  );
}

export function OrganizationReportsRoute({ organizationId }: { organizationId: string }) {
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
  const reportCapability = organization?.capabilities.find((capability) => capability.key === "reports");
  const canViewReports = Boolean(reportCapability?.enabled);
  const [csvError, setCsvError] = useState<RouteError | null>(null);
  const [csvState, setCsvState] = useState<CsvState>("idle");
  const [report, setReport] = useState<OrganizationRewardDashboard | null>(null);
  const [reportError, setReportError] = useState<RouteError | null>(null);
  const [reportLoadState, setReportLoadState] = useState<ReportLoadState>("idle");
  const notice = organizationNotice(route.error || reportError || csvError);

  const loadReport = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canViewReports) {
      return;
    }

    setReportError(null);
    setReportLoadState("loading");

    try {
      const nextReport = await fetchOrganizationRewardDashboard({
        organizationId: organization.id,
        token,
      });
      setReport(nextReport);
      setReportLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
      }
      setReport(null);
      setReportError(routeError);
      setReportLoadState("error");
    }
  }, [canViewReports, invalidOrganizationId, organization]);

  useEffect(() => {
    if (route.session && organization && canViewReports) {
      const timeout = window.setTimeout(() => {
        setCsvError(null);
        setCsvState("idle");
        setReport(null);
        setReportError(null);
        setReportLoadState("idle");
        void loadReport();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canViewReports, loadReport, organization, route.session]);

  async function downloadCsv() {
    const token = readStoredSessionToken();
    if (!token || !organization) {
      setCsvError({ code: "missing_token", message: "Sign in again before exporting reports.", status: 401 });
      setCsvState("error");
      return;
    }

    setCsvError(null);
    setCsvState("downloading");

    try {
      const csv = await downloadOrganizationRewardDashboardCsv({
        organizationId: organization.id,
        token,
      });
      triggerCsvDownload(csv.body, csv.filename);
      setCsvState("success");
    } catch (nextError) {
      setCsvError(normalizeRouteError(nextError));
      setCsvState("error");
    }
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: organization ? `/organizations/${organization.id}` : undefined,
          label: organization?.name || "Organization",
        },
        { label: "Reports" },
      ]}
      description="Organization reward volume, sponsored teacher application summary, wallet balances, and export state."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} reports` : "Organization reports"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/reports`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/reports`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canViewReports ? (
        <ReportDeniedState capability={reportCapability} organizationName={organization.name} />
      ) : null}
      {route.session && organization && canViewReports ? (
        <OrganizationReportsContent
          csvError={csvError}
          csvState={csvState}
          onDownloadCsv={downloadCsv}
          onRefresh={loadReport}
          organization={organization}
          report={report}
          reportError={reportError}
          reportLoadState={reportLoadState}
        />
      ) : null}
    </ProductShell>
  );
}

export function OrganizationWalletRoute({ organizationId }: { organizationId: string }) {
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
  const canManageWallets = Boolean(organization?.effectivePermissions.includes("MANAGE_ORG_WALLETS"));
  const canManageBudget = Boolean(organization?.effectivePermissions.includes("MANAGE_ORG_REWARD_BUDGET"));
  const canViewReports = Boolean(organization?.effectivePermissions.includes("VIEW_ORG_REWARD_REPORTS"));
  const canViewWallet = canManageWallets || canManageBudget || canViewReports;
  const walletCapability = organization?.capabilities.find((capability) => capability.key === "wallet");
  const [audit, setAudit] = useState<OrganizationWalletAudit | null>(null);
  const [walletError, setWalletError] = useState<RouteError | null>(null);
  const [walletLoadState, setWalletLoadState] = useState<WalletLoadState>("idle");
  const [linkError, setLinkError] = useState<RouteError | null>(null);
  const [linkState, setLinkState] = useState<WalletLinkState>("idle");
  const notice = organizationNotice(route.error || walletError || linkError);

  const loadWallet = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canViewWallet) {
      return;
    }

    setWalletError(null);
    setWalletLoadState("loading");

    try {
      const nextAudit = await fetchOrganizationWalletAudit({
        organizationId: organization.id,
        token,
      });
      setAudit(nextAudit);
      setWalletLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
      }

      if (routeError.status === 404 && /wallet not linked/i.test(routeError.message)) {
        setAudit(null);
        setWalletError(null);
        setWalletLoadState("missing");
        return;
      }

      setAudit(null);
      setWalletError(routeError);
      setWalletLoadState("error");
    }
  }, [canViewWallet, invalidOrganizationId, organization]);

  useEffect(() => {
    if (route.session && organization && canViewWallet) {
      const timeout = window.setTimeout(() => {
        setAudit(null);
        setLinkError(null);
        setLinkState("idle");
        setWalletError(null);
        setWalletLoadState("idle");
        void loadWallet();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canViewWallet, loadWallet, organization, route.session]);

  async function linkWallet() {
    const token = readStoredSessionToken();
    if (!token || !organization) {
      setLinkError({ code: "missing_token", message: "Sign in again before linking this wallet.", status: 401 });
      setLinkState("error");
      return;
    }

    setLinkError(null);
    setLinkState("linking");

    try {
      await linkOrganizationWallet({
        organizationId: organization.id,
        token,
      });
      setLinkState("success");
      await loadWallet();
    } catch (nextError) {
      setLinkError(normalizeRouteError(nextError));
      setLinkState("error");
    }
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: organization ? `/organizations/${organization.id}` : undefined,
          label: organization?.name || "Organization",
        },
        { label: "Wallet" },
      ]}
      description="Organization wallet balance, budget readiness, reward-credit audit, token links, and permission-aware wallet actions."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} wallet` : "Organization wallet"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/wallet`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/wallet`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canViewWallet ? (
        <WalletDeniedState capability={walletCapability} organizationName={organization.name} />
      ) : null}
      {route.session && organization && canViewWallet ? (
        <OrganizationWalletContent
          audit={audit}
          canManageBudget={canManageBudget}
          canManageWallets={canManageWallets}
          linkError={linkError}
          linkState={linkState}
          loadState={walletLoadState}
          onLinkWallet={linkWallet}
          onRefresh={loadWallet}
          organization={organization}
          walletError={walletError}
        />
      ) : null}
    </ProductShell>
  );
}

export function OrganizationSettingsRoute({ organizationId }: { organizationId: string }) {
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
  const canManageSettings = Boolean(organization?.effectivePermissions.includes("MANAGE_ORG_SETTINGS"));
  const settingsCapability = organization?.capabilities.find((c) => c.key === "settings");
  const [detail, setDetail] = useState<OrganizationDetail | null>(null);
  const [settingsError, setSettingsError] = useState<RouteError | null>(null);
  const [settingsLoadState, setSettingsLoadState] = useState<SettingsLoadState>("idle");
  const [nameDraft, setNameDraft] = useState("");
  const [websiteDraft, setWebsiteDraft] = useState("");
  const [profileUrlDraft, setProfileUrlDraft] = useState("");
  const [saveState, setSaveState] = useState<SettingsSaveState>("idle");
  const [saveMessage, setSaveMessage] = useState<string | null>(null);
  const [deleteConfirm, setDeleteConfirm] = useState(false);
  const [deleteState, setDeleteState] = useState<SettingsSaveState>("idle");
  const [deleteMessage, setDeleteMessage] = useState<string | null>(null);
  const notice = organizationNotice(route.error || settingsError || (saveState === "error" ? { code: "error", message: saveMessage ?? "", status: 0 } : null) || (deleteState === "error" ? { code: "error", message: deleteMessage ?? "", status: 0 } : null));

  const loadSettings = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization) return;

    setSettingsError(null);
    setSettingsLoadState("loading");
    try {
      const nextDetail = await fetchOrganization({
        organizationId: organization.id,
        token,
      });
      setDetail(nextDetail);
      setNameDraft(nextDetail.name);
      setWebsiteDraft(nextDetail.website_link ?? "");
      setProfileUrlDraft(nextDetail.profile_url ?? "");
      setSettingsLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) clearStoredSessionToken();
      setDetail(null);
      setSettingsError(routeError);
      setSettingsLoadState("error");
    }
  }, [invalidOrganizationId, organization]);

  useEffect(() => {
    if (route.session && organization) {
      const timeout = window.setTimeout(() => {
        setDetail(null);
        setSettingsError(null);
        setSettingsLoadState("idle");
        void loadSettings();
      }, 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [loadSettings, organization, route.session]);

  async function handleSave(event: React.FormEvent) {
    event.preventDefault();
    const token = readStoredSessionToken();
    if (!token || !organization) return;

    setSaveState("saving");
    setSaveMessage(null);
    try {
      const updated = await updateOrganization({
        organizationId: organization.id,
        payload: {
          name: nameDraft.trim() || undefined,
          profile_url: profileUrlDraft.trim() || null,
          website_link: websiteDraft.trim() || null,
        },
        token,
      });
      setDetail(updated);
      setNameDraft(updated.name);
      setWebsiteDraft(updated.website_link ?? "");
      setProfileUrlDraft(updated.profile_url ?? "");
      setSaveState("success");
      setSaveMessage("Settings saved.");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setSaveMessage(routeError.message);
      setSaveState("error");
    }
  }

  async function handleDelete() {
    const token = readStoredSessionToken();
    if (!token || !organization) return;

    setDeleteState("saving");
    setDeleteMessage(null);
    try {
      await deleteOrganization({ organizationId: organization.id, token });
      setDeleteState("success");
      setDeleteMessage("Organization deleted. Return to the organization list.");
      setDetail(null);
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setDeleteMessage(routeError.message);
      setDeleteState("error");
    }
  }

  function resetDeleteConfirm() {
    setDeleteConfirm(false);
    setDeleteState("idle");
    setDeleteMessage(null);
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        { href: organization ? `/organizations/${organization.id}` : undefined, label: organization?.name || "Organization" },
        { label: "Settings" },
      ]}
      description="Organization name, profile links, and destructive actions scoped to operator permissions."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} settings` : "Organization settings"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/settings`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/settings`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canManageSettings ? (
        <SettingsDeniedState capability={settingsCapability} organizationName={organization.name} />
      ) : null}
      {route.session && organization && canManageSettings ? (
        <OrganizationSettingsPanel
          deleteConfirm={deleteConfirm}
          deleteMessage={deleteMessage}
          deleteState={deleteState}
          detail={detail}
          nameDraft={nameDraft}
          onDelete={handleDelete}
          onDeleteConfirmChange={setDeleteConfirm}
          onDeleteConfirmReset={resetDeleteConfirm}
          onNameDraftChange={setNameDraft}
          onProfileUrlDraftChange={setProfileUrlDraft}
          onRefresh={loadSettings}
          onSave={handleSave}
          onWebsiteDraftChange={setWebsiteDraft}
          organization={organization}
          profileUrlDraft={profileUrlDraft}
          saveMessage={saveMessage}
          saveState={saveState}
          settingsError={settingsError}
          settingsLoadState={settingsLoadState}
          websiteDraft={websiteDraft}
        />
      ) : null}
    </ProductShell>
  );
}

function OrganizationDashboard({
  dashboard,
  dashboardError,
  dashboardLoadState,
  onRefreshDashboard,
  organization,
}: {
  dashboard: OrganizationDashboardSummary | null;
  dashboardError: RouteError | null;
  dashboardLoadState: DashboardLoadState;
  onRefreshDashboard: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  const enabledCapabilities = enabledOrganizationCapabilities(organization);
  const deniedCapabilities = organization.capabilities.filter((capability) => !capability.enabled);
  const pendingTeacherApplications = dashboard
    ? dashboard.teacher_applications.submitted + dashboard.teacher_applications.needs_changes
    : 0;
  const walletBalance = dashboard?.wallet.available ? formatTokenAmount(dashboard.wallet.balance_total) : "Gated";

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
            This dashboard combines the resolved session scope with the organization dashboard
            contract so operators can see attention items before opening member, course, report,
            wallet, or nomination workspaces.
          </p>
        </div>
        <div className={styles.actionRow}>
          <button className={styles.secondaryButton} onClick={onRefreshDashboard} type="button">
            <RefreshCw size={17} aria-hidden />
            Refresh
          </button>
        </div>
        <div className={styles.permissionRow}>
          {organization.roles.length ? (
            organization.roles.map((role) => <span className={styles.permissionChip} key={role}>{role}</span>)
          ) : (
            <span className={styles.permissionChip}>No role label</span>
          )}
          {dashboard ? (
            <StatusPill label={formatUnderscoreLabel(dashboard.health.status)} tone={dashboard.health.status === "attention" ? "warn" : "good"} />
          ) : null}
        </div>
      </section>

      {dashboardLoadState === "loading" || dashboardLoadState === "idle" ? (
        <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
          <div className={styles.panelHeader}>
            <Loader2 className={styles.spin} size={20} aria-hidden />
            <h2>Loading dashboard summary</h2>
          </div>
          <div className={styles.skeletonGrid} aria-hidden>
            <div className={styles.skeleton} />
            <div className={styles.skeleton} />
            <div className={styles.skeleton} />
          </div>
        </section>
      ) : null}

      {dashboardLoadState === "error" ? (
        <DashboardErrorState error={dashboardError} onRetry={onRefreshDashboard} />
      ) : null}

      {dashboard ? (
        <>
          <section className={styles.summaryGrid}>
            <SummaryCard icon={<Users size={20} aria-hidden />} label="Members" value={dashboard.members.available ? dashboard.members.total : "Gated"} />
            <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Courses" value={dashboard.courses.available ? dashboard.courses.total : "Gated"} />
            <SummaryCard icon={<UserPlus size={20} aria-hidden />} label="Pending teacher apps" value={dashboard.teacher_applications.available ? pendingTeacherApplications : "Gated"} />
            <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet balance" value={walletBalance} />
          </section>

          <section className={styles.twoColumn}>
            <section className={styles.panel}>
              <div className={styles.sectionHeader}>
                <h2>Attention items</h2>
                <StatusPill label={`${dashboard.health.alert_count} visible`} tone={dashboard.health.status === "attention" ? "warn" : "good"} />
              </div>
              <div className={styles.compactList}>
                {dashboard.alerts.map((alert) => (
                  <DashboardAlertCard alert={alert} key={`${alert.kind}-${alert.message}`} />
                ))}
              </div>
            </section>

            <section className={styles.panel}>
              <div className={styles.sectionHeader}>
                <h2>Operational signals</h2>
                <StatusPill label={dashboard.rewards.available ? "Reports visible" : "Reports gated"} tone={dashboard.rewards.available ? "good" : "warn"} />
              </div>
              <div className={styles.metricGrid}>
                <Metric label="Published courses" value={dashboard.courses.available ? dashboard.courses.published : "Gated"} />
                <Metric label="Reward candidates" value={dashboard.rewards.available ? dashboard.rewards.reward_candidate_count : "Gated"} />
                <Metric label="Approved amount" value={dashboard.rewards.available ? formatTokenAmount(dashboard.rewards.approved_amount_total) : "Gated"} />
              </div>
              <div className={styles.metricGrid}>
                <Metric label="Verified members" value={dashboard.members.available ? dashboard.members.verified_email_count : "Gated"} />
                <Metric label="Needs changes" value={dashboard.courses.available ? dashboard.courses.needs_changes : "Gated"} />
                <Metric label="Wallets" value={dashboard.wallet.available ? dashboard.wallet.wallet_count : "Gated"} />
              </div>
              <PermissionGateList dashboard={dashboard} />
            </section>
          </section>
        </>
      ) : null}

      <section className={styles.twoColumn}>
        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Available quick actions</h2>
            <StatusPill label={`${enabledCapabilities.length} enabled`} tone={enabledCapabilities.length ? "good" : "neutral"} />
          </div>
          {enabledCapabilities.length ? (
            <div className={styles.actionGrid}>
              {enabledCapabilities.map((capability) => (
                <ActionCard capability={capability} enabled key={capability.key} organizationId={organization.id} />
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
            <ActionCard capability={capability} enabled={false} key={capability.key} organizationId={organization.id} />
          ))}
        </div>
      </section>
    </>
  );
}

function DashboardAlertCard({ alert }: { alert: OrganizationDashboardAlert }) {
  const tone = alert.severity === "warning" ? "warn" : "good";
  return (
    <article className={styles.compactRow}>
      <span>
        <StatusPill label={formatUnderscoreLabel(alert.kind)} tone={tone} />
        {alert.message}
      </span>
      {alert.action_href && alert.action_label ? (
        <Link className={styles.secondaryLink} href={alert.action_href}>
          {alert.action_label}
        </Link>
      ) : null}
    </article>
  );
}

function PermissionGateList({ dashboard }: { dashboard: OrganizationDashboardSummary }) {
  const gatedSections = [
    { label: "Members", section: dashboard.members },
    { label: "Courses", section: dashboard.courses },
    { label: "Teacher applications", section: dashboard.teacher_applications },
    { label: "Reports", section: dashboard.rewards },
    { label: "Wallet", section: dashboard.wallet },
  ].filter((item) => !item.section.available);

  if (!gatedSections.length) {
    return (
      <div className={styles.permissionRow}>
        <span className={styles.permissionChip}>All dashboard sections visible</span>
      </div>
    );
  }

  return (
    <div className={styles.missingList}>
      <strong>Gated dashboard sections</strong>
      {gatedSections.map((item) => (
        <span key={item.label}>
          {item.label}: {item.section.missing_permissions.join(", ")}
        </span>
      ))}
    </div>
  );
}

function OrganizationMembersContent({
  draftSearch,
  loadState,
  members,
  onApplyFilters,
  onDraftSearchChange,
  onPageChange,
  onPermissionFilterChange,
  onRefresh,
  onResetFilters,
  onRoleFilterChange,
  organization,
  page,
  permissionFilter,
  roleFilter,
  routeError,
}: {
  draftSearch: string;
  loadState: MemberLoadState;
  members: OrganizationMemberList | null;
  onApplyFilters: () => void;
  onDraftSearchChange: (value: string) => void;
  onPageChange: (page: number) => void;
  onPermissionFilterChange: (value: string) => void;
  onRefresh: () => void;
  onResetFilters: () => void;
  onRoleFilterChange: (value: string) => void;
  organization: OrganizationWorkspaceItem;
  page: number;
  permissionFilter: string;
  roleFilter: string;
  routeError: RouteError | null;
}) {
  if (loadState === "loading" || loadState === "idle") {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
        <div className={styles.panelHeader}>
          <Loader2 className={styles.spin} size={20} aria-hidden />
          <h2>Loading organization members</h2>
        </div>
        <div className={styles.skeletonGrid} aria-hidden>
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
        </div>
      </section>
    );
  }

  if (loadState === "error") {
    return <MemberErrorState error={routeError} onRetry={onRefresh} />;
  }

  if (!members) {
    return null;
  }

  const delegatedCount = members.members.filter((member) => member.delegated_permission_count > 0).length;
  const verifiedEmailCount = members.members.filter((member) => member.email_verified).length;
  const kycReadyCount = members.members.filter((member) => member.kyc_verified).length;
  const totalPages = Math.max(1, Math.ceil(members.total / members.limit));
  const canGoBack = members.offset > 0;
  const canGoForward = members.offset + members.limit < members.total;

  return (
    <>
      <section className={styles.workspaceHero}>
        <div className={styles.workspaceTitleBlock}>
          <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
            <ArrowLeft size={17} aria-hidden />
            {organization.name}
          </Link>
          <p className={styles.eyebrow}>Organization members</p>
          <h2>Member directory</h2>
          <p className={styles.muted}>
            Members load from the organization-scoped directory contract. Invite, role-change,
            removal, and audit-history actions remain separate route work.
          </p>
        </div>
        <div className={styles.actionRow}>
          <button className={styles.secondaryButton} onClick={onRefresh} type="button">
            <RefreshCw size={17} aria-hidden />
            Refresh
          </button>
        </div>
        <div className={styles.permissionRow}>
          {members.operator_permissions.can_invite_members ? <span className={styles.permissionChip}>Can invite</span> : null}
          {members.operator_permissions.can_manage_members ? <span className={styles.permissionChip}>Can manage members</span> : null}
          {members.operator_permissions.can_assign_roles ? <span className={styles.permissionChip}>Can assign roles</span> : null}
          {!members.operator_permissions.can_invite_members &&
          !members.operator_permissions.can_manage_members &&
          !members.operator_permissions.can_assign_roles ? (
            <span className={styles.permissionChip}>View-only directory</span>
          ) : null}
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Matching members" value={members.total} />
        <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Delegated access" value={delegatedCount} />
        <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Verified emails" value={verifiedEmailCount} />
        <SummaryCard icon={<Building2 size={20} aria-hidden />} label="KYC ready" value={kycReadyCount} />
      </section>

      <section className={styles.memberFilterPanel} aria-label="Organization member filters">
        <label>
          <span>Search members</span>
          <span className={styles.inputWithIcon}>
            <Search size={17} aria-hidden />
            <input
              onChange={(event) => onDraftSearchChange(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") {
                  event.preventDefault();
                  onApplyFilters();
                }
              }}
              placeholder="Name, email, role, or permission"
              type="search"
              value={draftSearch}
            />
          </span>
        </label>
        <label>
          <span>Role</span>
          <select
            aria-label="Organization member role filter"
            onChange={(event) => onRoleFilterChange(event.target.value)}
            value={roleFilter}
          >
            {organizationMemberRoleOptions.map((option) => (
              <option key={option.label} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <label>
          <span>Permission</span>
          <select
            aria-label="Organization member permission filter"
            onChange={(event) => onPermissionFilterChange(event.target.value)}
            value={permissionFilter}
          >
            {organizationMemberPermissionOptions.map((option) => (
              <option key={option.label} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <div className={styles.filterActions}>
          <button className={styles.primaryButton} onClick={onApplyFilters} type="button">
            <Search size={17} aria-hidden />
            Apply
          </button>
          <button className={styles.secondaryButton} onClick={onResetFilters} type="button">
            <RefreshCw size={17} aria-hidden />
            Reset
          </button>
        </div>
      </section>

      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Member list</h2>
          <StatusPill label={`Page ${page + 1} of ${totalPages}`} tone="neutral" />
        </div>
        {members.members.length ? (
          <div className={styles.memberGrid}>
            {members.members.map((member) => (
              <OrganizationMemberCard member={member} key={member.id} />
            ))}
          </div>
        ) : (
          <p className={styles.muted}>
            No organization members match these filters. Reset filters or check whether the member
            still has a role in this organization.
          </p>
        )}
        <div className={styles.paginationRow}>
          <button
            className={styles.secondaryButton}
            disabled={!canGoBack}
            onClick={() => onPageChange(Math.max(0, page - 1))}
            type="button"
          >
            Previous
          </button>
          <span>
            {members.total === 0
              ? "0 members"
              : `${members.offset + 1}-${Math.min(members.offset + members.limit, members.total)} of ${members.total}`}
          </span>
          <button
            className={styles.secondaryButton}
            disabled={!canGoForward}
            onClick={() => onPageChange(page + 1)}
            type="button"
          >
            Next
          </button>
        </div>
      </section>
    </>
  );
}

function OrganizationMemberCard({ member }: { member: OrganizationMemberListItem }) {
  const previewPermissions = member.effective_permissions.slice(0, 4);
  const remainingPermissions = member.effective_permissions.length - previewPermissions.length;

  return (
    <article className={styles.memberCard}>
      <div className={styles.sectionHeader}>
        <div>
          <h3>{member.name}</h3>
          <p className={styles.muted}>{member.email}</p>
        </div>
        <StatusPill label={member.roles[0] ? formatUnderscoreLabel(member.roles[0]) : "No role"} tone="neutral" />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Direct" value={member.direct_permission_count} />
        <Metric label="Delegated" value={member.delegated_permission_count} />
        <Metric label="Effective" value={member.effective_permission_count} />
      </div>
      <div className={styles.compactList}>
        <div className={styles.compactRow}>
          <span>Email</span>
          <strong>{member.email_verified ? "Verified" : "Pending"}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>KYC</span>
          <strong>{member.kyc_verified ? "Ready" : "Not verified"}</strong>
        </div>
      </div>
      <div className={styles.permissionRow}>
        {member.roles.length ? (
          member.roles.map((role) => <span className={styles.permissionChip} key={role}>{role}</span>)
        ) : (
          <span className={styles.permissionChip}>No role label</span>
        )}
      </div>
      <div className={styles.permissionList}>
        {previewPermissions.length ? (
          previewPermissions.map((permission) => <span key={permission}>{permission}</span>)
        ) : (
          <span>No effective permissions</span>
        )}
        {remainingPermissions > 0 ? <span>+{remainingPermissions} more</span> : null}
      </div>
    </article>
  );
}

function OrganizationCoursesContent({
  courses,
  draftSearch,
  lifecycleStatus,
  loadState,
  onApplyFilters,
  onDraftSearchChange,
  onLifecycleStatusChange,
  onPageChange,
  onRefresh,
  onResetFilters,
  onRewardFilterChange,
  organization,
  page,
  routeError,
  rewardFilter,
}: {
  courses: OrganizationCourseList | null;
  draftSearch: string;
  lifecycleStatus: string;
  loadState: CourseLoadState;
  onApplyFilters: () => void;
  onDraftSearchChange: (value: string) => void;
  onLifecycleStatusChange: (value: string) => void;
  onPageChange: (page: number) => void;
  onRefresh: () => void;
  onResetFilters: () => void;
  onRewardFilterChange: (value: "all" | "rewarded" | "unrewarded") => void;
  organization: OrganizationWorkspaceItem;
  page: number;
  routeError: RouteError | null;
  rewardFilter: "all" | "rewarded" | "unrewarded";
}) {
  if (loadState === "loading" || loadState === "idle") {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
        <div className={styles.panelHeader}>
          <Loader2 className={styles.spin} size={20} aria-hidden />
          <h2>Loading organization courses</h2>
        </div>
        <div className={styles.skeletonGrid} aria-hidden>
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
        </div>
      </section>
    );
  }

  if (loadState === "error") {
    return <CourseErrorState error={routeError} onRetry={onRefresh} />;
  }

  if (!courses) {
    return null;
  }

  const activePolicyCount = courses.courses.reduce(
    (sum, course) => sum + course.rewards.active_policy_count,
    0,
  );
  const pendingJoinCount = courses.courses.reduce(
    (sum, course) => sum + course.roster.pending_join_request_count,
    0,
  );
  const pendingRewardCount = courses.courses.reduce(
    (sum, course) => sum + course.reward_queue.pending_teacher_count,
    0,
  );
  const totalPages = Math.max(1, Math.ceil(courses.total / courses.limit));
  const canGoBack = courses.offset > 0;
  const canGoForward = courses.offset + courses.limit < courses.total;

  return (
    <>
      <section className={styles.workspaceHero}>
        <div className={styles.workspaceTitleBlock}>
          <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
            <ArrowLeft size={17} aria-hidden />
            {organization.name}
          </Link>
          <p className={styles.eyebrow}>Organization courses</p>
          <h2>Sponsored course workspace</h2>
          <p className={styles.muted}>
            Courses load from the organization-scoped course contract. Editing, publishing, and
            organization-course ownership changes remain separate route work.
          </p>
        </div>
        <div className={styles.actionRow}>
          <button className={styles.secondaryButton} onClick={onRefresh} type="button">
            <RefreshCw size={17} aria-hidden />
            Refresh
          </button>
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Matching courses" value={courses.total} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Reward policies" value={activePolicyCount} />
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Pending joins" value={pendingJoinCount} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward queue" value={pendingRewardCount} />
      </section>

      <section className={styles.courseFilterPanel} aria-label="Organization course filters">
        <label>
          <span>Search courses</span>
          <span className={styles.inputWithIcon}>
            <Search size={17} aria-hidden />
            <input
              onChange={(event) => onDraftSearchChange(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") {
                  event.preventDefault();
                  onApplyFilters();
                }
              }}
              placeholder="Title"
              type="search"
              value={draftSearch}
            />
          </span>
        </label>
        <label>
          <span>Lifecycle</span>
          <select
            aria-label="Course lifecycle filter"
            onChange={(event) => onLifecycleStatusChange(event.target.value)}
            value={lifecycleStatus}
          >
            {organizationCourseLifecycleOptions.map((option) => (
              <option key={option.label} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <label>
          <span>Rewards</span>
          <select
            aria-label="Course reward filter"
            onChange={(event) =>
              onRewardFilterChange(event.target.value as "all" | "rewarded" | "unrewarded")
            }
            value={rewardFilter}
          >
            <option value="all">All reward states</option>
            <option value="rewarded">Reward policy active</option>
            <option value="unrewarded">No active policy</option>
          </select>
        </label>
        <div className={styles.filterActions}>
          <button className={styles.primaryButton} onClick={onApplyFilters} type="button">
            <Search size={17} aria-hidden />
            Apply
          </button>
          <button className={styles.secondaryButton} onClick={onResetFilters} type="button">
            <RefreshCw size={17} aria-hidden />
            Reset
          </button>
        </div>
      </section>

      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Course list</h2>
          <StatusPill label={`Page ${page + 1} of ${totalPages}`} tone="neutral" />
        </div>
        {courses.courses.length ? (
          <div className={styles.courseGrid}>
            {courses.courses.map((course) => (
              <OrganizationCourseCard course={course} key={course.id} />
            ))}
          </div>
        ) : (
          <p className={styles.muted}>
            No organization courses match these filters. Reset filters or check whether the course is
            attached to this organization.
          </p>
        )}
        <div className={styles.paginationRow}>
          <button
            className={styles.secondaryButton}
            disabled={!canGoBack}
            onClick={() => onPageChange(Math.max(0, page - 1))}
            type="button"
          >
            Previous
          </button>
          <span>
            {courses.total === 0
              ? "0 courses"
              : `${courses.offset + 1}-${Math.min(courses.offset + courses.limit, courses.total)} of ${courses.total}`}
          </span>
          <button
            className={styles.secondaryButton}
            disabled={!canGoForward}
            onClick={() => onPageChange(page + 1)}
            type="button"
          >
            Next
          </button>
        </div>
      </section>
    </>
  );
}

function OrganizationCourseCard({ course }: { course: OrganizationCourseListItem }) {
  const teacherNames = course.teachers.map((teacher) => teacher.name).join(", ");
  const rewardEvents = course.rewards.event_types.length
    ? course.rewards.event_types.map(formatUnderscoreLabel).join(", ")
    : "No active reward policy";

  return (
    <article className={styles.courseCard}>
      <div className={styles.sectionHeader}>
        <div>
          <h3>{course.title}</h3>
          <p className={styles.muted}>{teacherNames || "No teacher assigned"}</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(course.lifecycle_status)} tone={course.lifecycle_status === "published" ? "good" : "neutral"} />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Chapters" value={course.content.chapter_count} />
        <Metric label="Lessons" value={course.content.content_count} />
        <Metric label="Students" value={course.roster.enrolled_student_count} />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Pending joins" value={course.roster.pending_join_request_count} />
        <Metric label="Reward review" value={course.reward_queue.pending_teacher_count} />
        <Metric label="Approved queue" value={course.reward_queue.teacher_approved_count} />
      </div>
      <div className={styles.compactList}>
        <div className={styles.compactRow}>
          <span>Reward status</span>
          <strong>{rewardEvents}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Content types</span>
          <strong>{course.content.content_types.length ? course.content.content_types.join(", ") : "None yet"}</strong>
        </div>
      </div>
      <div className={styles.permissionRow}>
        {course.permissions.can_manage_enrollments ? <span className={styles.permissionChip}>Can review joins</span> : null}
        {course.permissions.can_submit_reward_events ? <span className={styles.permissionChip}>Can submit rewards</span> : null}
        {course.permissions.can_create_courses ? <span className={styles.permissionChip}>Can create courses</span> : null}
        {!course.permissions.can_manage_enrollments &&
        !course.permissions.can_submit_reward_events &&
        !course.permissions.can_create_courses ? (
          <span className={styles.permissionChip}>View only</span>
        ) : null}
      </div>
    </article>
  );
}

function OrganizationTeacherApplicationsContent({
  applications,
  draftSearch,
  loadState,
  onApplyFilters,
  onDraftSearchChange,
  onPageChange,
  onRefresh,
  onResetFilters,
  onStatusFilterChange,
  organization,
  page,
  routeError,
  statusFilter,
}: {
  applications: OrganizationTeacherApplicationList | null;
  draftSearch: string;
  loadState: TeacherApplicationLoadState;
  onApplyFilters: () => void;
  onDraftSearchChange: (value: string) => void;
  onPageChange: (page: number) => void;
  onRefresh: () => void;
  onResetFilters: () => void;
  onStatusFilterChange: (value: string) => void;
  organization: OrganizationWorkspaceItem;
  page: number;
  routeError: RouteError | null;
  statusFilter: string;
}) {
  if (loadState === "loading" || loadState === "idle") {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
        <div className={styles.panelHeader}>
          <Loader2 className={styles.spin} size={20} aria-hidden />
          <h2>Loading teacher applications</h2>
        </div>
        <div className={styles.skeletonGrid} aria-hidden>
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
        </div>
      </section>
    );
  }

  if (loadState === "error") {
    return <TeacherApplicationErrorState error={routeError} onRetry={onRefresh} />;
  }

  if (!applications) {
    return null;
  }

  const totalPages = Math.max(1, Math.ceil(applications.total / applications.limit));
  const canGoBack = applications.offset > 0;
  const canGoForward = applications.offset + applications.limit < applications.total;

  return (
    <>
      <section className={styles.workspaceHero}>
        <div className={styles.workspaceTitleBlock}>
          <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
            <ArrowLeft size={17} aria-hidden />
            {organization.name}
          </Link>
          <p className={styles.eyebrow}>Teacher nominations</p>
          <h2>Sponsored application tracking</h2>
          <p className={styles.muted}>
            Applications load from the organization-scoped tracking contract. Nomination submission
            still needs a searchable applicant picker before it should become a normal operator form.
          </p>
        </div>
        <div className={styles.actionRow}>
          <button className={styles.secondaryButton} onClick={onRefresh} type="button">
            <RefreshCw size={17} aria-hidden />
            Refresh
          </button>
        </div>
        <div className={styles.permissionRow}>
          {applications.operator_permissions.can_view_applications ? <span className={styles.permissionChip}>Can view tracking</span> : null}
          {applications.operator_permissions.can_nominate_teachers ? <span className={styles.permissionChip}>Can nominate</span> : null}
          {!applications.operator_permissions.can_view_applications &&
          !applications.operator_permissions.can_nominate_teachers ? (
            <span className={styles.permissionChip}>Tracking unavailable</span>
          ) : null}
          <StatusPill label={`${applications.summary.rejected} rejected`} tone={applications.summary.rejected ? "warn" : "neutral"} />
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<UserPlus size={20} aria-hidden />} label="All applications" value={applications.summary.total} />
        <SummaryCard icon={<Search size={20} aria-hidden />} label="Matching rows" value={applications.total} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Submitted" value={applications.summary.submitted} />
        <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Approved" value={applications.summary.approved} />
      </section>

      <section className={styles.applicationFilterPanel} aria-label="Teacher application filters">
        <label>
          <span>Search applications</span>
          <span className={styles.inputWithIcon}>
            <Search size={17} aria-hidden />
            <input
              onChange={(event) => onDraftSearchChange(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") {
                  event.preventDefault();
                  onApplyFilters();
                }
              }}
              placeholder="Applicant, email, scope, course, or status"
              type="search"
              value={draftSearch}
            />
          </span>
        </label>
        <label>
          <span>Status</span>
          <select
            aria-label="Teacher application status filter"
            onChange={(event) => onStatusFilterChange(event.target.value)}
            value={statusFilter}
          >
            {organizationTeacherApplicationStatusOptions.map((option) => (
              <option key={option.label} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <div className={styles.filterActions}>
          <button className={styles.primaryButton} onClick={onApplyFilters} type="button">
            <Search size={17} aria-hidden />
            Apply
          </button>
          <button className={styles.secondaryButton} onClick={onResetFilters} type="button">
            <RefreshCw size={17} aria-hidden />
            Reset
          </button>
        </div>
      </section>

      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Application list</h2>
          <StatusPill label={`Page ${page + 1} of ${totalPages}`} tone="neutral" />
        </div>
        {applications.applications.length ? (
          <div className={styles.applicationGrid}>
            {applications.applications.map((application) => (
              <OrganizationTeacherApplicationCard application={application} key={application.id} />
            ))}
          </div>
        ) : (
          <p className={styles.muted}>
            No sponsored teacher applications match these filters. Reset filters or check whether
            the application was sponsored by another organization.
          </p>
        )}
        <div className={styles.paginationRow}>
          <button
            className={styles.secondaryButton}
            disabled={!canGoBack}
            onClick={() => onPageChange(Math.max(0, page - 1))}
            type="button"
          >
            Previous
          </button>
          <span>
            {applications.total === 0
              ? "0 applications"
              : `${applications.offset + 1}-${Math.min(applications.offset + applications.limit, applications.total)} of ${applications.total}`}
          </span>
          <button
            className={styles.secondaryButton}
            disabled={!canGoForward}
            onClick={() => onPageChange(page + 1)}
            type="button"
          >
            Next
          </button>
        </div>
      </section>
    </>
  );
}

function OrganizationTeacherApplicationCard({
  application,
}: {
  application: OrganizationTeacherApplicationItem;
}) {
  const requestedScope =
    application.requested_course?.title ||
    application.requested_organization?.name ||
    formatUnderscoreLabel(application.requested_scope);
  const latestAudit = application.audit.latest_event_type
    ? `${formatUnderscoreLabel(application.audit.latest_event_type)}${
        application.audit.latest_event_at ? ` at ${formatDateTime(application.audit.latest_event_at)}` : ""
      }`
    : "No audit event";

  return (
    <article className={styles.applicationCard}>
      <div className={styles.sectionHeader}>
        <div>
          <h3>{application.applicant.name}</h3>
          <p className={styles.muted}>{application.applicant.email}</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(application.status)} tone={teacherApplicationStatusTone(application.status)} />
      </div>

      <div className={styles.metricGrid}>
        <Metric label="Audit events" value={application.audit.event_count} />
        <Metric label="Portfolio links" value={application.portfolio_links.length} />
        <Metric label="Decided" value={application.decided_at ? "Yes" : "No"} />
      </div>

      <div className={styles.compactList}>
        <div className={styles.compactRow}>
          <span>Requested scope</span>
          <strong>{requestedScope}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Requested organization</span>
          <strong>{application.requested_organization?.name || "Platform-wide"}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Latest audit</span>
          <strong>{latestAudit}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Updated</span>
          <strong>{formatDateTime(application.updated_at)}</strong>
        </div>
      </div>

      <p className={styles.muted}>{application.experience_summary}</p>

      {application.decision_reason || application.reviewer ? (
        <div className={styles.compactList}>
          {application.reviewer ? (
            <div className={styles.compactRow}>
              <span>Reviewer</span>
              <strong>{application.reviewer.name}</strong>
            </div>
          ) : null}
          {application.decision_reason ? (
            <div className={styles.compactRow}>
              <span>Decision reason</span>
              <strong>{application.decision_reason}</strong>
            </div>
          ) : null}
        </div>
      ) : null}

      <div className={styles.permissionRow}>
        {application.sponsored_by_this_organization ? <span className={styles.permissionChip}>Sponsored here</span> : null}
        {application.requested_for_this_organization ? <span className={styles.permissionChip}>Requested here</span> : null}
        {application.audit.latest_reason ? <span className={styles.permissionChip}>Audit reason attached</span> : null}
      </div>

      {application.portfolio_links.length ? (
        <div className={styles.portfolioList}>
          {application.portfolio_links.slice(0, 3).map((link, index) => {
            const href = safeExternalHref(link);
            return href ? (
              <a
                className={styles.secondaryLink}
                href={href}
                key={`${link}-${index}`}
                rel="noreferrer"
                target="_blank"
                title={link}
              >
                <ExternalLink size={16} aria-hidden />
                Portfolio {index + 1}
              </a>
            ) : (
              <span className={styles.permissionChip} key={`${link}-${index}`}>
                Portfolio link unavailable
              </span>
            );
          })}
          {application.portfolio_links.length > 3 ? (
            <span className={styles.permissionChip}>+{application.portfolio_links.length - 3} more</span>
          ) : null}
        </div>
      ) : null}
    </article>
  );
}

function OrganizationReportsContent({
  csvError,
  csvState,
  onDownloadCsv,
  onRefresh,
  organization,
  report,
  reportError,
  reportLoadState,
}: {
  csvError: RouteError | null;
  csvState: CsvState;
  onDownloadCsv: () => void;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
  report: OrganizationRewardDashboard | null;
  reportError: RouteError | null;
  reportLoadState: ReportLoadState;
}) {
  if (reportLoadState === "loading" || reportLoadState === "idle") {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
        <div className={styles.panelHeader}>
          <Loader2 className={styles.spin} size={20} aria-hidden />
          <h2>Loading reward reports</h2>
        </div>
        <div className={styles.skeletonGrid} aria-hidden>
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
        </div>
      </section>
    );
  }

  if (reportLoadState === "error") {
    return <ReportErrorState error={reportError} onRetry={onRefresh} />;
  }

  if (!report) {
    return null;
  }

  const hasCourseRows = report.courses.length > 0;
  const hasWalletRows = report.wallets.length > 0;

  return (
    <>
      <section className={styles.workspaceHero}>
        <div className={styles.workspaceTitleBlock}>
          <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
            <ArrowLeft size={17} aria-hidden />
            {organization.name}
          </Link>
          <p className={styles.eyebrow}>Reward reports</p>
          <h2>All-time reward dashboard</h2>
          <p className={styles.muted}>
            Current backend reports are all-time snapshots. Date filters, pagination, payout-failure
            drill-downs, and reconciliation rows remain open report-contract work.
          </p>
        </div>
        <div className={styles.actionRow}>
          <button className={styles.secondaryButton} onClick={onRefresh} type="button">
            <RefreshCw size={17} aria-hidden />
            Refresh
          </button>
          <button
            className={styles.primaryButton}
            disabled={csvState === "downloading"}
            onClick={onDownloadCsv}
            type="button"
          >
            {csvState === "downloading" ? (
              <Loader2 className={styles.spin} size={17} aria-hidden />
            ) : (
              <Download size={17} aria-hidden />
            )}
            Export CSV
          </button>
        </div>
        {csvState === "success" ? (
          <StatusPill icon={<CheckCircle2 size={16} aria-hidden />} label="CSV ready" tone="good" />
        ) : null}
        {csvError ? (
          <section className={styles.inlineError} role="status">
            <AlertTriangle size={17} aria-hidden />
            <span>{csvError.message}</span>
          </section>
        ) : null}
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward candidates" value={report.course_reward_count} />
        <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Approved rewards" value={report.approved_reward_count} />
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Approved amount" value={formatTokenAmount(report.approved_amount_total)} />
        <SummaryCard icon={<Building2 size={20} aria-hidden />} label="Wallet balance" value={formatTokenAmount(report.wallet_balance_total)} />
      </section>

      <section className={styles.twoColumn}>
        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Sponsored teacher applications</h2>
            <StatusPill label={`${report.sponsored_teacher_applications.total} total`} tone="neutral" />
          </div>
          <div className={styles.metricGrid}>
            <Metric label="Submitted" value={report.sponsored_teacher_applications.submitted} />
            <Metric label="Needs changes" value={report.sponsored_teacher_applications.needs_changes} />
            <Metric label="Approved" value={report.sponsored_teacher_applications.approved} />
          </div>
          <p className={styles.muted}>
            Sponsored application rows and decision history are not part of this report contract yet.
          </p>
        </section>

        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Wallet balances</h2>
            <StatusPill label={`${report.wallets.length} wallet${report.wallets.length === 1 ? "" : "s"}`} tone={hasWalletRows ? "good" : "neutral"} />
          </div>
          {hasWalletRows ? (
            <div className={styles.compactList}>
              {report.wallets.map((wallet, index) => (
                <article className={styles.compactRow} key={wallet.wallet_id}>
                  <span>Organization wallet {index + 1}</span>
                  <strong>{formatTokenAmount(wallet.balance)}</strong>
                </article>
              ))}
            </div>
          ) : (
            <p className={styles.muted}>No organization wallet balances are attached to this report.</p>
          )}
        </section>
      </section>

      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Course reward volume</h2>
          <StatusPill label={`${report.courses.length} course${report.courses.length === 1 ? "" : "s"}`} tone={hasCourseRows ? "good" : "neutral"} />
        </div>
        {hasCourseRows ? (
          <div className={styles.reportGrid}>
            {report.courses.map((course) => (
              <article className={styles.reportCard} key={course.course_id}>
                <h3>{course.course_title}</h3>
                <div className={styles.metricGrid}>
                  <Metric label="Candidates" value={course.reward_candidate_count} />
                  <Metric label="Approved" value={course.approved_reward_count} />
                  <Metric label="Amount" value={formatTokenAmount(course.approved_amount_total)} />
                </div>
              </article>
            ))}
          </div>
        ) : (
          <p className={styles.muted}>
            No course reward rows are available yet. The CSV export will still include the summary
            headers for this organization.
          </p>
        )}
      </section>
    </>
  );
}

function OrganizationWalletContent({
  audit,
  canManageBudget,
  canManageWallets,
  linkError,
  linkState,
  loadState,
  onLinkWallet,
  onRefresh,
  organization,
  walletError,
}: {
  audit: OrganizationWalletAudit | null;
  canManageBudget: boolean;
  canManageWallets: boolean;
  linkError: RouteError | null;
  linkState: WalletLinkState;
  loadState: WalletLoadState;
  onLinkWallet: () => void;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
  walletError: RouteError | null;
}) {
  if (loadState === "loading" || loadState === "idle") {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
        <div className={styles.panelHeader}>
          <Loader2 className={styles.spin} size={20} aria-hidden />
          <h2>Loading wallet audit</h2>
        </div>
        <div className={styles.skeletonGrid} aria-hidden>
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
          <div className={styles.skeleton} />
        </div>
      </section>
    );
  }

  if (loadState === "error") {
    return <WalletErrorState error={walletError} onRetry={onRefresh} />;
  }

  if (loadState === "missing") {
    return (
      <MissingOrganizationWalletState
        canManageWallets={canManageWallets}
        linkError={linkError}
        linkState={linkState}
        onLinkWallet={onLinkWallet}
        onRefresh={onRefresh}
        organization={organization}
      />
    );
  }

  if (!audit) {
    return null;
  }

  const walletBalance = numericAmount(audit.wallet.value);
  const approvedAmountTotal = sumAmounts(audit.reward_records.map((record) => record.approved_amount));
  const uncreditedAmountTotal = sumAmounts(
    audit.reward_records
      .filter((record) => record.wallet_credit_record_id === null)
      .map((record) => record.approved_amount),
  );
  const attentionRecords = audit.reward_records.filter((record) =>
    walletRewardNeedsAttention(record.reconciliation_status),
  );
  const budgetTone = uncreditedAmountTotal > walletBalance ? "warn" : "good";
  const hasAuditRows =
    audit.internal_transactions.length > 0 ||
    audit.external_transactions.length > 0 ||
    audit.reward_records.length > 0 ||
    audit.compensation_records.length > 0;

  return (
    <>
      <section className={styles.workspaceHero}>
        <div className={styles.workspaceTitleBlock}>
          <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
            <ArrowLeft size={17} aria-hidden />
            {organization.name}
          </Link>
          <p className={styles.eyebrow}>Wallet and budget</p>
          <h2>Organization wallet audit</h2>
          <p className={styles.muted}>
            Wallet audit rows combine internal ledger entries, token payout references, source
            organization reward records, and compensation adjustments from the current backend
            contract.
          </p>
        </div>
        <div className={styles.actionRow}>
          <button className={styles.secondaryButton} onClick={onRefresh} type="button">
            <RefreshCw size={17} aria-hidden />
            Refresh
          </button>
          {canManageWallets ? (
            <button
              className={styles.secondaryButton}
              disabled={linkState === "linking"}
              onClick={onLinkWallet}
              type="button"
            >
              {linkState === "linking" ? (
                <Loader2 className={styles.spin} size={17} aria-hidden />
              ) : (
                <CreditCard size={17} aria-hidden />
              )}
              Relink wallet
            </button>
          ) : null}
        </div>
        <div className={styles.permissionRow}>
          {canManageWallets ? <span className={styles.permissionChip}>Can manage wallets</span> : null}
          {canManageBudget ? <span className={styles.permissionChip}>Can manage reward budget</span> : null}
          {!canManageWallets && !canManageBudget ? <span className={styles.permissionChip}>Read-only wallet audit</span> : null}
          {linkState === "success" ? (
            <StatusPill icon={<CheckCircle2 size={16} aria-hidden />} label="Wallet linked" tone="good" />
          ) : null}
        </div>
        {linkError ? (
          <section className={styles.inlineError} role="status">
            <AlertTriangle size={17} aria-hidden />
            <span>{linkError.message}</span>
          </section>
        ) : null}
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet balance" value={formatTokenAmount(audit.wallet.value)} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward audit rows" value={audit.reward_records.length} />
        <SummaryCard icon={<AlertTriangle size={20} aria-hidden />} label="Needs attention" value={attentionRecords.length} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Ledger rows" value={audit.internal_transactions.length} />
      </section>

      <section className={styles.twoColumn}>
        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Budget constraints</h2>
            <StatusPill label={budgetTone === "warn" ? "Review balance" : "Balance visible"} tone={budgetTone} />
          </div>
          <div className={styles.metricGrid}>
            <Metric label="Approved amount" value={formatTokenAmount(approvedAmountTotal)} />
            <Metric label="Uncredited approved" value={formatTokenAmount(uncreditedAmountTotal)} />
            <Metric label="Balance" value={formatTokenAmount(walletBalance)} />
          </div>
          <p className={styles.muted}>
            Compare visible balance with uncredited approved rewards before reward-budget actions.
            This view does not reserve funds or execute payouts; it gives operators the current
            audit context.
          </p>
        </section>

        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Audit coverage</h2>
            <StatusPill label={hasAuditRows ? "Rows available" : "Empty audit"} tone={hasAuditRows ? "good" : "neutral"} />
          </div>
          <div className={styles.metricGrid}>
            <Metric label="Internal ledger" value={audit.internal_transactions.length} />
            <Metric label="Token links" value={audit.external_transactions.length} />
            <Metric label="Compensations" value={audit.compensation_records.length} />
          </div>
          <p className={styles.muted}>
            Empty audit rows are valid for a newly linked wallet. Reward-credit and payout rows
            appear after reward execution records are created.
          </p>
        </section>
      </section>

      <section className={styles.panel}>
        <div className={styles.sectionHeader}>
          <h2>Reward credit audit</h2>
          <StatusPill label={`${attentionRecords.length} attention`} tone={attentionRecords.length ? "warn" : "good"} />
        </div>
        {audit.reward_records.length ? (
          <div className={styles.reportGrid}>
            {audit.reward_records.slice(0, 8).map((record) => (
              <OrganizationWalletRewardCard
                externalTransaction={audit.external_transactions.find(
                  (external) => external.external_transaction_id === record.external_transaction_id,
                )}
                key={record.reward_candidate_id}
                record={record}
              />
            ))}
          </div>
        ) : (
          <p className={styles.muted}>No reward credit rows are associated with this organization wallet yet.</p>
        )}
      </section>

      <section className={styles.twoColumn}>
        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Internal ledger</h2>
            <StatusPill label={`${audit.internal_transactions.length} rows`} tone={audit.internal_transactions.length ? "good" : "neutral"} />
          </div>
          {audit.internal_transactions.length ? (
            <div className={styles.compactList}>
              {audit.internal_transactions.slice(0, 8).map((transaction) => (
                <article className={styles.compactRow} key={transaction.internal_transaction_id}>
                  <span>
                    <StatusPill label={formatUnderscoreLabel(transaction.transaction_type)} tone="neutral" />
                    {formatDateTime(transaction.created_at)}
                  </span>
                  <strong>{formatTokenAmount(transaction.amount)}</strong>
                </article>
              ))}
            </div>
          ) : (
            <p className={styles.muted}>No internal ledger rows are available for this wallet yet.</p>
          )}
        </section>

        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Token transaction links</h2>
            <StatusPill label={`${audit.external_transactions.length} links`} tone={audit.external_transactions.length ? "good" : "neutral"} />
          </div>
          {audit.external_transactions.length ? (
            <div className={styles.compactList}>
              {audit.external_transactions.slice(0, 8).map((transaction) => (
                <article className={styles.compactRow} key={transaction.external_transaction_id}>
                  <span>
                    <StatusPill label={transaction.event_type || "External transaction"} tone="neutral" />
                    {transaction.transaction_hash ? shortHash(transaction.transaction_hash) : transaction.blockchain_address}
                  </span>
                  <strong>{formatTokenAmount(transaction.amount)}</strong>
                </article>
              ))}
            </div>
          ) : (
            <p className={styles.muted}>No token payout or deposit transaction links are attached yet.</p>
          )}
        </section>
      </section>

      {audit.compensation_records.length ? (
        <section className={styles.panel}>
          <div className={styles.sectionHeader}>
            <h2>Compensation adjustments</h2>
            <StatusPill label={`${audit.compensation_records.length} adjustments`} tone="warn" />
          </div>
          <div className={styles.reportGrid}>
            {audit.compensation_records.slice(0, 6).map((record) => (
              <article className={styles.reportCard} key={record.id}>
                <h3>{record.reason}</h3>
                <div className={styles.metricGrid}>
                  <Metric label="Amount" value={formatTokenAmount(record.amount)} />
                  <Metric label="Created" value={formatDateTime(record.created_at)} />
                  <Metric label="Reward" value="Adjustment" />
                </div>
              </article>
            ))}
          </div>
        </section>
      ) : null}
    </>
  );
}

function OrganizationWalletRewardCard({
  externalTransaction,
  record,
}: {
  externalTransaction?: OrganizationWalletAudit["external_transactions"][number];
  record: OrganizationWalletRewardRecordAudit;
}) {
  const needsAttention = walletRewardNeedsAttention(record.reconciliation_status);
  const explorerUrl = externalTransaction ? tokenExplorerUrl(externalTransaction) : null;
  return (
    <article className={styles.reportCard}>
      <div className={styles.sectionHeader}>
        <h3>{formatUnderscoreLabel(record.candidate_status)}</h3>
        <StatusPill label={formatUnderscoreLabel(record.reconciliation_status)} tone={needsAttention ? "warn" : "good"} />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Approved amount" value={formatTokenAmount(record.approved_amount)} />
        <Metric label="Wallet credit" value={record.wallet_credit_record_id ? "Recorded" : "Missing"} />
        <Metric label="Updated" value={formatDateTime(record.updated_at)} />
      </div>
      {explorerUrl && externalTransaction?.transaction_hash ? (
        <a
          className={styles.secondaryLink}
          href={explorerUrl}
          rel="noreferrer"
          target="_blank"
        >
          <ExternalLink size={17} aria-hidden />
          Token tx {shortHash(externalTransaction.transaction_hash)}
        </a>
      ) : externalTransaction?.transaction_hash ? (
        <span className={styles.permissionChip}>
          Token tx {shortHash(externalTransaction.transaction_hash)}
        </span>
      ) : externalTransaction ? (
        <span className={styles.permissionChip}>Token transfer recorded</span>
      ) : (
        <span className={styles.permissionChip}>No token link yet</span>
      )}
    </article>
  );
}

function MissingOrganizationWalletState({
  canManageWallets,
  linkError,
  linkState,
  onLinkWallet,
  onRefresh,
  organization,
}: {
  canManageWallets: boolean;
  linkError: RouteError | null;
  linkState: WalletLinkState;
  onLinkWallet: () => void;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <CreditCard size={20} aria-hidden />
        <h2>Wallet not linked</h2>
      </div>
      <p className={styles.muted}>
        {organization.name} does not have an organization wallet attached yet. Link the wallet
        before relying on balance, reward-credit, or token reconciliation views.
      </p>
      <div className={styles.actionRow}>
        {canManageWallets ? (
          <button
            className={styles.primaryButton}
            disabled={linkState === "linking"}
            onClick={onLinkWallet}
            type="button"
          >
            {linkState === "linking" ? (
              <Loader2 className={styles.spin} size={17} aria-hidden />
            ) : (
              <CreditCard size={17} aria-hidden />
            )}
            Link organization wallet
          </button>
        ) : (
          <span className={styles.permissionChip}>Wallet manager required</span>
        )}
        <button className={styles.secondaryButton} onClick={onRefresh} type="button">
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </div>
      {!canManageWallets ? (
        <div className={styles.missingList}>
          <strong>Missing scoped permission</strong>
          <span>MANAGE_ORG_WALLETS</span>
        </div>
      ) : null}
      {linkError ? (
        <section className={styles.inlineError} role="status">
          <AlertTriangle size={17} aria-hidden />
          <span>{linkError.message}</span>
        </section>
      ) : null}
    </section>
  );
}

function WalletDeniedState({
  capability,
  organizationName,
}: {
  capability?: OrganizationCapability;
  organizationName: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Organization wallet unavailable</h2>
      </div>
      <p className={styles.muted}>
        Your current session can open {organizationName}, but it cannot view wallet balance,
        budget, or audit rows.
      </p>
      <div className={styles.missingList}>
        <strong>Missing scoped permission</strong>
        {(capability
          ? missingOrganizationPermissions(capability)
          : ["MANAGE_ORG_WALLETS", "MANAGE_ORG_REWARD_BUDGET", "VIEW_ORG_REWARD_REPORTS"]
        ).map((permission) => (
          <span key={permission}>{permission}</span>
        ))}
      </div>
      <Link className={styles.secondaryLink} href="/organizations">
        Back to organizations
      </Link>
    </section>
  );
}

function WalletErrorState({
  error,
  onRetry,
}: {
  error: RouteError | null;
  onRetry: () => void;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error?.code || "wallet_error"}</strong>
        <span>{error?.message || "Organization wallet audit could not be loaded."}</span>
      </span>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function MembersDeniedState({
  capability,
  organizationName,
}: {
  capability?: OrganizationCapability;
  organizationName: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Organization members unavailable</h2>
      </div>
      <p className={styles.muted}>
        Your current session can see that {organizationName} exists, but it cannot open the
        organization member directory.
      </p>
      <div className={styles.missingList}>
        <strong>Missing scoped permission</strong>
        {(capability ? missingOrganizationPermissions(capability) : ["VIEW_ORGANIZATION"]).map(
          (permission) => (
            <span key={permission}>{permission}</span>
          ),
        )}
      </div>
      <Link className={styles.secondaryLink} href="/organizations">
        Back to organizations
      </Link>
    </section>
  );
}

function MemberErrorState({
  error,
  onRetry,
}: {
  error: RouteError | null;
  onRetry: () => void;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error?.code || "member_error"}</strong>
        <span>{error?.message || "Organization members could not be loaded."}</span>
      </span>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function DashboardErrorState({
  error,
  onRetry,
}: {
  error: RouteError | null;
  onRetry: () => void;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error?.code || "dashboard_error"}</strong>
        <span>{error?.message || "Organization dashboard summary could not be loaded."}</span>
      </span>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function CoursesDeniedState({
  capability,
  organizationName,
}: {
  capability?: OrganizationCapability;
  organizationName: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Organization courses unavailable</h2>
      </div>
      <p className={styles.muted}>
        Your current session can see that {organizationName} exists, but it cannot open the
        organization course list.
      </p>
      <div className={styles.missingList}>
        <strong>Missing scoped permission</strong>
        {(capability ? missingOrganizationPermissions(capability) : ["VIEW_ORGANIZATION"]).map(
          (permission) => (
            <span key={permission}>{permission}</span>
          ),
        )}
      </div>
      <Link className={styles.secondaryLink} href="/organizations">
        Back to organizations
      </Link>
    </section>
  );
}

function CourseErrorState({
  error,
  onRetry,
}: {
  error: RouteError | null;
  onRetry: () => void;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error?.code || "course_error"}</strong>
        <span>{error?.message || "Organization courses could not be loaded."}</span>
      </span>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function TeacherApplicationsDeniedState({
  capability,
  organizationName,
}: {
  capability?: OrganizationCapability;
  organizationName: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Teacher applications unavailable</h2>
      </div>
      <p className={styles.muted}>
        Your current session can see that {organizationName} exists, but it cannot open sponsored
        teacher application tracking.
      </p>
      <div className={styles.missingList}>
        <strong>Missing scoped permission</strong>
        {(capability
          ? missingOrganizationPermissions(capability)
          : ["VIEW_ORG_TEACHER_APPLICATIONS", "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW"]
        ).map((permission) => (
          <span key={permission}>{permission}</span>
        ))}
      </div>
      <Link className={styles.secondaryLink} href="/organizations">
        Back to organizations
      </Link>
    </section>
  );
}

function TeacherApplicationErrorState({
  error,
  onRetry,
}: {
  error: RouteError | null;
  onRetry: () => void;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error?.code || "teacher_application_error"}</strong>
        <span>{error?.message || "Organization teacher applications could not be loaded."}</span>
      </span>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function ReportDeniedState({
  capability,
  organizationName,
}: {
  capability?: OrganizationCapability;
  organizationName: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Reward reports unavailable</h2>
      </div>
      <p className={styles.muted}>
        Your current session can open {organizationName}, but it cannot view organization reward
        reports.
      </p>
      <div className={styles.missingList}>
        <strong>Missing scoped permission</strong>
        {(capability ? missingOrganizationPermissions(capability) : ["VIEW_ORG_REWARD_REPORTS"]).map(
          (permission) => (
            <span key={permission}>{permission}</span>
          ),
        )}
      </div>
      <Link className={styles.secondaryLink} href="/organizations">
        Back to organizations
      </Link>
    </section>
  );
}

function ReportErrorState({
  error,
  onRetry,
}: {
  error: RouteError | null;
  onRetry: () => void;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error?.code || "report_error"}</strong>
        <span>{error?.message || "Organization reward report could not be loaded."}</span>
      </span>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
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
  organizationId,
}: {
  capability: OrganizationCapability;
  enabled: boolean;
  organizationId: number;
}) {
  const coursesHref = `/organizations/${organizationId}/courses`;
  const membersHref = `/organizations/${organizationId}/members`;
  const reportHref = `/organizations/${organizationId}/reports`;
  const settingsHref = `/organizations/${organizationId}/settings`;
  const teacherApplicationsHref = `/organizations/${organizationId}/teacher-applications`;
  const walletHref = `/organizations/${organizationId}/wallet`;

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
      {enabled && capability.key === "courses" ? (
        <Link className={styles.primaryLink} href={coursesHref}>
          <BookOpen size={17} aria-hidden />
          Open courses
        </Link>
      ) : enabled && capability.key === "members" ? (
        <Link className={styles.primaryLink} href={membersHref}>
          <Users size={17} aria-hidden />
          Open members
        </Link>
      ) : enabled && capability.key === "reports" ? (
        <Link className={styles.primaryLink} href={reportHref}>
          <FileText size={17} aria-hidden />
          Open reports
        </Link>
      ) : enabled && capability.key === "teacher_applications" ? (
        <Link className={styles.primaryLink} href={teacherApplicationsHref}>
          <UserPlus size={17} aria-hidden />
          Open teacher nominations
        </Link>
      ) : enabled && capability.key === "wallet" ? (
        <Link className={styles.primaryLink} href={walletHref}>
          <CreditCard size={17} aria-hidden />
          Open wallet
        </Link>
      ) : enabled && capability.key === "settings" ? (
        <Link className={styles.primaryLink} href={settingsHref}>
          <Settings size={17} aria-hidden />
          Open settings
        </Link>
      ) : (
        <button className={styles.secondaryButton} disabled type="button">
          Contract pending
        </button>
      )}
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

function formatUnderscoreLabel(value: string) {
  return value.replace(/_/g, " ").replace(/\b\w/g, (letter) => letter.toUpperCase());
}

function teacherApplicationStatusTone(status: string): "good" | "neutral" | "warn" {
  if (status === "approved") {
    return "good";
  }

  if (status === "needs_changes" || status === "rejected") {
    return "warn";
  }

  return "neutral";
}

function formatDateTime(value: string | null | undefined) {
  if (!value) {
    return "Unknown";
  }

  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return "Unknown";
  }

  return new Intl.DateTimeFormat("en-US", {
    day: "numeric",
    month: "short",
    year: "numeric",
  }).format(date);
}

function safeExternalHref(value: string) {
  try {
    const url = new URL(value);
    return url.protocol === "http:" || url.protocol === "https:" ? url.toString() : null;
  } catch {
    return null;
  }
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

function SettingsDeniedState({
  capability,
  organizationName,
}: {
  capability?: OrganizationCapability;
  organizationName: string;
}) {
  const missingPermissions = capability ? missingOrganizationPermissions(capability) : [];
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <h2>Settings unavailable</h2>
      </div>
      <p className={styles.muted}>
        Your current session cannot manage settings for {organizationName}.
        {missingPermissions.length ? ` Missing permission${missingPermissions.length === 1 ? "" : "s"}: ${missingPermissions.join(", ")}.` : ""}
      </p>
    </section>
  );
}

function OrganizationSettingsPanel({
  deleteConfirm,
  deleteMessage,
  deleteState,
  detail,
  nameDraft,
  onDelete,
  onDeleteConfirmChange,
  onDeleteConfirmReset,
  onNameDraftChange,
  onProfileUrlDraftChange,
  onRefresh,
  onSave,
  onWebsiteDraftChange,
  organization,
  profileUrlDraft,
  saveMessage,
  saveState,
  settingsError,
  settingsLoadState,
  websiteDraft,
}: {
  deleteConfirm: boolean;
  deleteMessage: string | null;
  deleteState: SettingsSaveState;
  detail: OrganizationDetail | null;
  nameDraft: string;
  onDelete: () => void;
  onDeleteConfirmChange: (value: boolean) => void;
  onDeleteConfirmReset: () => void;
  onNameDraftChange: (value: string) => void;
  onProfileUrlDraftChange: (value: string) => void;
  onRefresh: () => void;
  onSave: (event: React.FormEvent) => void;
  onWebsiteDraftChange: (value: string) => void;
  organization: OrganizationWorkspaceItem;
  profileUrlDraft: string;
  saveMessage: string | null;
  saveState: SettingsSaveState;
  settingsError: RouteError | null;
  settingsLoadState: SettingsLoadState;
  websiteDraft: string;
}) {
  const isSaving = saveState === "saving";
  const isDeleting = deleteState === "saving";
  const roles = organization.roles.join(", ") || "None";
  const roleSummary = organization.roles.length ? roles : "Delegated access only";

  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/organizations/${organization.id}`}>
          <ArrowLeft size={17} aria-hidden />
          {organization.name}
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{roleSummary}</p>
          <h2>Organization settings</h2>
          <p className={styles.muted}>
            Update the public identity fields for {organization.name}. These values appear in learner-facing course discovery and teacher application contexts.
          </p>
        </div>
      </section>

      {saveMessage ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
          <div className={styles.panelHeader}>
            {saveState === "success" ? <CheckCircle2 size={18} aria-hidden /> : <AlertTriangle size={18} aria-hidden />}
            <h2>{saveState === "success" ? "Saved" : "Update error"}</h2>
          </div>
          <p>{saveMessage}</p>
        </section>
      ) : null}

      {deleteMessage ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
          <div className={styles.panelHeader}>
            {deleteState === "success" ? <CheckCircle2 size={18} aria-hidden /> : <AlertTriangle size={18} aria-hidden />}
            <h2>{deleteState === "success" ? "Deleted" : "Deletion error"}</h2>
          </div>
          <p>{deleteMessage}</p>
        </section>
      ) : null}

      {settingsLoadState === "loading" ? (
        <section className={`${styles.panel} ${styles.singlePanel}`}>
          <div className={styles.panelHeader}>
            <Loader2 className={styles.spin} size={20} aria-hidden />
            <h2>Loading settings</h2>
          </div>
          <p className={styles.muted}>Fetching current organization values from the backend.</p>
        </section>
      ) : null}

      {settingsLoadState === "error" && settingsError ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`}>
          <AlertTriangle size={18} aria-hidden />
          <span>
            <strong>{settingsError.code}</strong>
            {settingsError.message}
          </span>
          <button className={styles.secondaryButton} onClick={onRefresh} type="button">
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {settingsLoadState === "success" && detail ? (
        <>
          <form className={`${styles.panel} ${styles.singlePanel}`} onSubmit={onSave}>
            <div className={styles.panelHeader}>
              <Settings size={20} aria-hidden />
              <h2>Organization identity</h2>
            </div>
            <div className={styles.authoringForm}>
              <label>
                <span>Name</span>
                <input
                  disabled={isSaving}
                  maxLength={120}
                  onChange={(event) => onNameDraftChange(event.target.value)}
                  placeholder="Organization name"
                  required
                  value={nameDraft}
                />
              </label>
              <label>
                <span>Website</span>
                <input
                  disabled={isSaving}
                  maxLength={240}
                  onChange={(event) => onWebsiteDraftChange(event.target.value)}
                  placeholder="https://example.com"
                  value={websiteDraft}
                />
              </label>
              <label>
                <span>Profile image URL</span>
                <input
                  disabled={isSaving}
                  maxLength={240}
                  onChange={(event) => onProfileUrlDraftChange(event.target.value)}
                  placeholder="https://example.com/logo.png"
                  value={profileUrlDraft}
                />
              </label>
              <div style={{ display: "flex", gap: "0.75rem", marginTop: "0.5rem" }}>
                <button className={styles.primaryButton} disabled={isSaving} type="submit">
                  <Send size={17} aria-hidden />
                  {isSaving ? "Saving…" : "Save settings"}
                </button>
                <button className={styles.secondaryButton} disabled={isSaving} onClick={onRefresh} type="button">
                  <RefreshCw size={17} aria-hidden />
                  Refresh
                </button>
              </div>
            </div>
          </form>

          <section className={`${styles.panel} ${styles.singlePanel}`}>
            <div className={styles.panelHeader}>
              <AlertTriangle size={20} aria-hidden />
              <h2>Delete organization</h2>
            </div>
            <p className={styles.muted}>
              Permanently remove {organization.name} and its data. Inactive courses and existing wallet state may prevent deletion.
            </p>
            {deleteConfirm ? (
              <div className={styles.actionRow} style={{ marginTop: "0.75rem" }}>
                <button
                  className={styles.primaryButton}
                  disabled={isDeleting}
                  onClick={onDelete}
                  style={{ background: "var(--color-warn, #dc2626)", borderColor: "var(--color-warn, #dc2626)" }}
                  type="button"
                >
                  <Send size={17} aria-hidden />
                  {isDeleting ? "Deleting…" : "Confirm delete"}
                </button>
                <button className={styles.secondaryButton} disabled={isDeleting} onClick={onDeleteConfirmReset} type="button">
                  Cancel
                </button>
              </div>
            ) : (
              <button
                className={styles.secondaryButton}
                onClick={() => onDeleteConfirmChange(true)}
                style={{ color: "var(--color-warn, #dc2626)", marginTop: "0.75rem" }}
                type="button"
              >
                <AlertTriangle size={17} aria-hidden />
                Delete {organization.name}
              </button>
            )}
          </section>
        </>
      ) : null}
    </>
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

  if (error instanceof OrganizationRequestError) {
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

function formatTokenAmount(value: string | number | null | undefined) {
  if (value === null || value === undefined || value === "") {
    return "0";
  }

  const numericValue = Number(value);
  if (!Number.isFinite(numericValue)) {
    return String(value);
  }

  return new Intl.NumberFormat("en-US", {
    maximumFractionDigits: 2,
  }).format(numericValue);
}

function numericAmount(value: string | number | null | undefined): number {
  const numericValue = Number(value);
  return Number.isFinite(numericValue) ? numericValue : 0;
}

function sumAmounts(values: Array<string | number | null | undefined>): number {
  return values.reduce<number>((total, value) => total + numericAmount(value), 0);
}

function walletRewardNeedsAttention(status: string) {
  return !["reconciled", "closed_without_payout"].includes(status);
}

function shortHash(value: string) {
  if (value.length <= 14) {
    return value;
  }

  return `${value.slice(0, 8)}...${value.slice(-6)}`;
}

function tokenExplorerUrl(transaction: OrganizationWalletAudit["external_transactions"][number]) {
  if (!transaction.transaction_hash) {
    return null;
  }

  if (transaction.chain_id === 1 || transaction.chain_id === null) {
    return `https://etherscan.io/tx/${transaction.transaction_hash}`;
  }

  if (transaction.chain_id === 11155111) {
    return `https://sepolia.etherscan.io/tx/${transaction.transaction_hash}`;
  }

  return null;
}

function triggerCsvDownload(body: string, filename: string) {
  if (typeof window === "undefined") {
    return;
  }

  const blob = new Blob([body], { type: "text/csv;charset=utf-8" });
  const url = window.URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  link.remove();
  window.URL.revokeObjectURL(url);
}
