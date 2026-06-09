"use client";

import {
  AlertTriangle,
  CheckCircle2,
  Clock3,
  Database,
  Download,
  FileSpreadsheet,
  FileText,
  Gauge,
  Landmark,
  Loader2,
  RefreshCw,
  Search,
  Send,
  ShieldAlert,
  ShieldCheck,
  UserCheck,
  Users,
  WalletCards,
  XCircle,
  type LucideIcon,
} from "lucide-react";
import Link from "next/link";
import { type FormEvent, type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import {
  AdminRequestError,
  buildPlatformAdminWorkspace,
  createFraudBlock,
  decideRewardAmount,
  decideTeacherApplication,
  downloadPlatformCsv,
  fetchFraudBlockAudit,
  fetchFraudBlocks,
  fetchPlatformFraudDashboard,
  fetchPlatformRewardCandidates,
  fetchPlatformRewardDashboard,
  fetchPlatformSummary,
  fetchPlatformSystemStatus,
  fetchPlatformTeacherApplications,
  fetchRewardCandidateAudit,
  fetchTeacherApplicationAudit,
  missingPlatformPermissions,
  platformCapabilityDefinitions,
  platformCapabilityEnabled,
  revokeFraudBlock,
  type FraudBlockAuditEvent,
  type FraudBlockCreateOptions,
  type FraudBlockItem,
  type FraudBlockStatus,
  type PlatformAdminWorkspace,
  type PlatformCapability,
  type PlatformCapabilityKey,
  type PlatformCsvDownload,
  type PlatformCsvReport,
  type PlatformFraudDashboard,
  type PlatformReportSummary,
  type PlatformRewardCandidateItem,
  type PlatformRewardCandidatesResponse,
  type PlatformRewardDashboard,
  type PlatformSystemStatus,
  type PlatformTeacherApplicationItem,
  type PlatformTeacherApplicationsResponse,
  type RewardAuditEvent,
  type TeacherApplicationAuditEvent,
  type TeacherApplicationStatus,
} from "@/lib/admin";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  SessionRequestError,
} from "@/lib/session";
import { ProductShell, type ShellNotice } from "./product-shell";
import styles from "./admin-routes.module.css";

type LoadState = "idle" | "loading" | "success" | "error";
type SectionState = "idle" | "loading" | "success" | "error";
type CsvState = "idle" | "downloading" | "success" | "error";

type RouteError = {
  code: string;
  message: string;
  status: number;
};

const emptyWorkspace: PlatformAdminWorkspace = {
  capabilities: platformCapabilityDefinitions.map((capability) => ({
    ...capability,
    enabled: false,
  })),
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

const exportReports: Array<{
  description: string;
  label: string;
  report: PlatformCsvReport;
}> = [
  {
    description: "Top-level platform counts for users, organizations, courses, wallets, and notifications.",
    label: "Platform summary",
    report: "summary",
  },
  {
    description: "Teacher applications, amount approvals, payout failures, and reconciliation mismatches.",
    label: "Reward dashboard",
    report: "reward_dashboard",
  },
  {
    description: "Active fraud blocks grouped by scope and recent block records.",
    label: "Fraud dashboard",
    report: "fraud_dashboard",
  },
  {
    description: "Teacher application rows with requested scope, sponsor, reviewer, reason, and timestamps.",
    label: "Teacher applications",
    report: "teacher_applications",
  },
  {
    description: "Teacher and amount approval decisions for reward candidates.",
    label: "Reward approvals",
    report: "reward_approvals",
  },
  {
    description: "Token payout records and external transaction state.",
    label: "Token payouts",
    report: "token_payouts",
  },
  {
    description: "Wallet credit rows linked to internal transactions and notifications.",
    label: "Wallet credits",
    report: "wallet_credits",
  },
  {
    description: "Delegated permission grants with scope, expiration, revocation, and usage timestamps.",
    label: "Delegations",
    report: "delegated_permissions",
  },
];

const ADMIN_TEACHER_APPLICATION_PAGE_SIZE = 8;
const teacherApplicationStatusOptions: Array<{ label: string; value: TeacherApplicationStatus | "" }> = [
  { label: "All statuses", value: "" },
  { label: "Submitted", value: "submitted" },
  { label: "Needs changes", value: "needs_changes" },
  { label: "Approved", value: "approved" },
  { label: "Rejected", value: "rejected" },
];

const actionIcons: Record<PlatformCapabilityKey, LucideIcon> = {
  delegations: ShieldCheck,
  exports: FileSpreadsheet,
  fraud_blocks: ShieldAlert,
  reward_amount_review: Landmark,
  summary: Gauge,
  system: Database,
  teacher_applications: UserCheck,
  wallets: WalletCards,
};

export function AdminDashboardRoute() {
  const route = useAdminSession();
  const workspace = useMemo(
    () => (route.session ? buildPlatformAdminWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const allowed = route.session ? hasPlatformAdminAccess(route.session) : false;
  const canViewSummary = hasPlatformPermission(workspace, "VIEW_REPORT");
  const canViewRewardAudit = hasPlatformPermission(workspace, "VIEW_REWARD_AUDIT");
  const canApproveRewardAmount = hasPlatformPermission(workspace, "APPROVE_REWARD_AMOUNT");
  const canExportData = hasPlatformPermission(workspace, "EXPORT_DATA");
  const canManageFraud = hasAnyPlatformPermission(workspace, [
    "BLOCK_REWARD_ORGANIZATION",
    "BLOCK_REWARD_TEACHER",
    "MANAGE_REWARD_FRAUD_BLOCKS",
  ]);
  const [summary, setSummary] = useState<PlatformReportSummary | null>(null);
  const [summaryError, setSummaryError] = useState<RouteError | null>(null);
  const [summaryState, setSummaryState] = useState<SectionState>("idle");
  const [rewardDashboard, setRewardDashboard] = useState<PlatformRewardDashboard | null>(null);
  const [rewardError, setRewardError] = useState<RouteError | null>(null);
  const [rewardState, setRewardState] = useState<SectionState>("idle");
  const [fraudDashboard, setFraudDashboard] = useState<PlatformFraudDashboard | null>(null);
  const [fraudError, setFraudError] = useState<RouteError | null>(null);
  const [fraudState, setFraudState] = useState<SectionState>("idle");
  const [systemStatus, setSystemStatus] = useState<PlatformSystemStatus | null>(null);
  const [systemError, setSystemError] = useState<RouteError | null>(null);
  const [systemState, setSystemState] = useState<SectionState>("idle");
  const [csvError, setCsvError] = useState<RouteError | null>(null);
  const [csvFilename, setCsvFilename] = useState<string | null>(null);
  const [csvState, setCsvState] = useState<CsvState>("idle");

  const loadDashboard = useCallback(async () => {
    const token = route.token;
    if (!route.session || !token || !allowed) {
      return;
    }

    if (canViewSummary) {
      setSummaryState("loading");
      setSummaryError(null);
      fetchPlatformSummary({ token })
        .then((nextSummary) => {
          setSummary(nextSummary);
          setSummaryState("success");
        })
        .catch((error) => {
          setSummary(null);
          setSummaryError(normalizeRouteError(error, "Platform summary could not be loaded."));
          setSummaryState("error");
        });
    } else {
      setSummary(null);
      setSummaryError(null);
      setSummaryState("idle");
    }

    if (canViewRewardAudit) {
      setRewardState("loading");
      setRewardError(null);
      fetchPlatformRewardDashboard({ token })
        .then((nextDashboard) => {
          setRewardDashboard(nextDashboard);
          setRewardState("success");
        })
        .catch((error) => {
          setRewardDashboard(null);
          setRewardError(normalizeRouteError(error, "Reward operations could not be loaded."));
          setRewardState("error");
        });

      setFraudState("loading");
      setFraudError(null);
      fetchPlatformFraudDashboard({ token })
        .then((nextDashboard) => {
          setFraudDashboard(nextDashboard);
          setFraudState("success");
        })
        .catch((error) => {
          setFraudDashboard(null);
          setFraudError(normalizeRouteError(error, "Fraud controls could not be loaded."));
          setFraudState("error");
        });
    } else {
      setRewardDashboard(null);
      setRewardError(null);
      setRewardState("idle");
      setFraudDashboard(null);
      setFraudError(null);
      setFraudState("idle");
    }

    setSystemState("loading");
    setSystemError(null);
    fetchPlatformSystemStatus()
      .then((nextStatus) => {
        setSystemStatus(nextStatus);
        setSystemState("success");
      })
      .catch((error) => {
        setSystemStatus(null);
        setSystemError(normalizeRouteError(error, "System status could not be loaded."));
        setSystemState("error");
      });
  }, [allowed, canViewRewardAudit, canViewSummary, route.session, route.token]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadDashboard(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadDashboard]);

  const notice: ShellNotice | null =
    csvState === "success" && csvFilename
      ? {
          message: `${csvFilename} downloaded.`,
          title: "CSV ready",
          tone: "success",
        }
      : csvState === "error" && csvError
        ? {
            message: csvError.message,
            title: "CSV export failed",
            tone: csvError.status === 403 ? "warn" : "error",
          }
        : null;

  async function handleCsvDownload(report: PlatformCsvReport, label: string) {
    const token = route.token;
    if (!token) {
      setCsvError({
        code: "missing_token",
        message: "Sign in again before downloading CSV exports.",
        status: 401,
      });
      setCsvState("error");
      return;
    }

    setCsvError(null);
    setCsvFilename(null);
    setCsvState("downloading");

    try {
      const csv = await downloadPlatformCsv({ report, token });
      startCsvDownload(csv);
      setCsvFilename(csv.filename || label);
      setCsvState("success");
    } catch (error) {
      setCsvError(normalizeRouteError(error, `${label} could not be exported.`));
      setCsvState("error");
    }
  }

  return (
    <ProductShell
      activeNav="admin"
      description="Platform-level review, reward operations, fraud controls, exports, and runtime status."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : allowed ? "Admin access" : "Access gated"} />
          <StatusPill label={`${workspace.effectivePermissionCount} platform permissions`} />
          <StatusPill label={`${workspace.delegatedPermissionCount} delegated`} />
        </>
      }
      title="Platform admin dashboard"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !allowed ? <AdminDeniedState workspace={workspace} /> : null}

      {route.session && allowed ? (
        <div className={styles.stack}>
          <SummaryPanel
            canViewSummary={canViewSummary}
            error={summaryError}
            onRetry={loadDashboard}
            state={summaryState}
            summary={summary}
            workspace={workspace}
          />
          <ActionPanel
            canApproveRewardAmount={canApproveRewardAmount}
            canExportData={canExportData}
            canManageFraud={canManageFraud}
            canViewRewardAudit={canViewRewardAudit}
            rewardDashboard={rewardDashboard}
            fraudDashboard={fraudDashboard}
            systemStatus={systemStatus}
            workspace={workspace}
          />
          <div className={styles.twoColumn}>
            <RewardPanel
              canApproveRewardAmount={canApproveRewardAmount}
              canViewRewardAudit={canViewRewardAudit}
              dashboard={rewardDashboard}
              error={rewardError}
              onRetry={loadDashboard}
              state={rewardState}
              workspace={workspace}
            />
            <FraudPanel
              canManageFraud={canManageFraud}
              canViewRewardAudit={canViewRewardAudit}
              dashboard={fraudDashboard}
              error={fraudError}
              onRetry={loadDashboard}
              state={fraudState}
              workspace={workspace}
            />
          </div>
          <div className={styles.twoColumn}>
            <ExportPanel
              canExportData={canExportData}
              csvState={csvState}
              onDownload={handleCsvDownload}
              workspace={workspace}
            />
            <SystemPanel error={systemError} onRetry={loadDashboard} state={systemState} status={systemStatus} />
          </div>
        </div>
      ) : null}
    </ProductShell>
  );
}

export function AdminTeacherApplicationsRoute() {
  const route = useAdminSession();
  const workspace = useMemo(
    () => (route.session ? buildPlatformAdminWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const allowed = route.session ? hasPlatformAdminAccess(route.session) : false;
  const canReview = hasPlatformPermission(workspace, "REVIEW_TEACHER_APPLICATIONS");
  const canApprove = hasPlatformPermission(workspace, "APPROVE_TEACHER_APPLICATION");
  const canReject = hasPlatformPermission(workspace, "REJECT_TEACHER_APPLICATION");
  const [applications, setApplications] = useState<PlatformTeacherApplicationsResponse | null>(null);
  const [applicationError, setApplicationError] = useState<RouteError | null>(null);
  const [applicationState, setApplicationState] = useState<SectionState>("idle");
  const [searchInput, setSearchInput] = useState("");
  const [appliedSearch, setAppliedSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<TeacherApplicationStatus | "">("submitted");
  const [offset, setOffset] = useState(0);
  const [selectedApplicationId, setSelectedApplicationId] = useState<number | null>(null);
  const [auditEvents, setAuditEvents] = useState<TeacherApplicationAuditEvent[]>([]);
  const [auditError, setAuditError] = useState<RouteError | null>(null);
  const [auditState, setAuditState] = useState<SectionState>("idle");
  const [decisionReason, setDecisionReason] = useState("");
  const [decisionStatus, setDecisionStatus] = useState<Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">>("needs_changes");
  const [decisionError, setDecisionError] = useState<RouteError | null>(null);
  const [decisionState, setDecisionState] = useState<"idle" | "submitting" | "success" | "error">("idle");
  const selectedApplication =
    applications?.applications.find((application) => application.id === selectedApplicationId) ||
    applications?.applications[0] ||
    null;

  const loadApplications = useCallback(async () => {
    const token = route.token;
    if (!route.session || !token || !allowed || !canReview) {
      return;
    }

    setApplicationState("loading");
    setApplicationError(null);

    try {
      const response = await fetchPlatformTeacherApplications({
        limit: ADMIN_TEACHER_APPLICATION_PAGE_SIZE,
        offset,
        search: appliedSearch,
        status: statusFilter,
        token,
      });
      setApplications(response);
      setApplicationState("success");
      setSelectedApplicationId((current) => {
        if (current && response.applications.some((application) => application.id === current)) {
          return current;
        }
        return response.applications[0]?.id || null;
      });
    } catch (error) {
      setApplications(null);
      setApplicationError(normalizeRouteError(error, "Teacher application review queue could not be loaded."));
      setApplicationState("error");
    }
  }, [allowed, appliedSearch, canReview, offset, route.session, route.token, statusFilter]);

  const loadAudit = useCallback(
    async (applicationId: number) => {
      const token = route.token;
      if (!token || !canReview) {
        return;
      }

      setAuditState("loading");
      setAuditError(null);
      try {
        const events = await fetchTeacherApplicationAudit({ applicationId, token });
        setAuditEvents(events);
        setAuditState("success");
      } catch (error) {
        setAuditEvents([]);
        setAuditError(normalizeRouteError(error, "Teacher application audit could not be loaded."));
        setAuditState("error");
      }
    },
    [canReview, route.token],
  );

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadApplications(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadApplications]);

  useEffect(() => {
    if (!selectedApplication?.id) {
      const timeout = window.setTimeout(() => {
        setAuditEvents([]);
        setAuditState("idle");
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    const timeout = window.setTimeout(() => void loadAudit(selectedApplication.id), 0);
    return () => window.clearTimeout(timeout);
  }, [loadAudit, selectedApplication?.id]);

  function applyFilters() {
    setOffset(0);
    setAppliedSearch(searchInput.trim());
  }

  function resetFilters() {
    setSearchInput("");
    setAppliedSearch("");
    setStatusFilter("submitted");
    setOffset(0);
  }

  async function submitDecision(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = route.token;
    if (!token || !selectedApplication) {
      return;
    }

    const trimmedReason = decisionReason.trim();
    if (!trimmedReason) {
      setDecisionError({
        code: "validation_error",
        message: "Add a decision reason before changing an application.",
        status: 400,
      });
      setDecisionState("error");
      return;
    }

    setDecisionError(null);
    setDecisionState("submitting");

    try {
      await decideTeacherApplication({
        applicationId: selectedApplication.id,
        decisionReason: trimmedReason,
        status: decisionStatus,
        token,
      });
      setDecisionReason("");
      setDecisionState("success");
      await loadApplications();
      await loadAudit(selectedApplication.id);
    } catch (error) {
      const routeError = normalizeRouteError(error, "Teacher application decision could not be saved.");
      setDecisionError(routeError);
      setDecisionState("error");
      if (routeError.status === 409) {
        await loadApplications();
        await loadAudit(selectedApplication.id);
      }
    }
  }

  const notice: ShellNotice | null =
    decisionState === "success"
      ? {
          message: "The application was refreshed with the latest decision state.",
          title: "Decision saved",
          tone: "success",
        }
      : decisionState === "error" && decisionError
        ? {
            message: decisionError.message,
            title: decisionError.status === 409 ? "Application changed" : "Decision failed",
            tone: decisionError.status === 403 || decisionError.status === 409 ? "warn" : "error",
          }
        : null;

  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Teacher applications" },
      ]}
      description="Review submitted teacher applications with applicant context, sponsor scope, audit history, and permission-gated decisions."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : canReview ? "Review access" : "Review gated"} />
          <StatusPill label={`${applications?.summary.submitted ?? 0} submitted`} tone={(applications?.summary.submitted ?? 0) ? "warn" : "neutral"} />
          <StatusPill label={canApprove ? "Approve enabled" : "Approve gated"} tone={canApprove ? "good" : "neutral"} />
        </>
      }
      title="Teacher application review"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !allowed ? <AdminDeniedState workspace={workspace} /> : null}

      {route.session && allowed ? (
        canReview ? (
          <div className={styles.stack}>
            <TeacherApplicationSummaryPanel response={applications} state={applicationState} />
            <TeacherApplicationFilterPanel
              onApply={applyFilters}
              onRefresh={loadApplications}
              onReset={resetFilters}
              searchInput={searchInput}
              setSearchInput={setSearchInput}
              setStatusFilter={(value) => {
                setStatusFilter(value);
                setOffset(0);
              }}
              state={applicationState}
              statusFilter={statusFilter}
            />
            {applicationState === "loading" || applicationState === "idle" ? (
              <PanelLoading title="Loading teacher applications" />
            ) : null}
            {applicationState === "error" ? (
              <PanelError error={applicationError} onRetry={loadApplications} title="Teacher application queue failed" />
            ) : null}
            {applicationState === "success" && applications ? (
              <div className={styles.twoColumnWide}>
                <TeacherApplicationQueue
                  applications={applications}
                  offset={offset}
                  onPage={(nextOffset) => setOffset(nextOffset)}
                  onSelect={setSelectedApplicationId}
                  selectedApplicationId={selectedApplication?.id || null}
                />
                <TeacherApplicationDetail
                  application={selectedApplication}
                  auditError={auditError}
                  auditEvents={auditEvents}
                  auditState={auditState}
                  canApprove={canApprove}
                  canReject={canReject}
                  decisionError={decisionError}
                  decisionReason={decisionReason}
                  decisionState={decisionState}
                  decisionStatus={decisionStatus}
                  onDecisionReasonChange={setDecisionReason}
                  onDecisionStatusChange={setDecisionStatus}
                  onRefreshAudit={() => selectedApplication ? void loadAudit(selectedApplication.id) : undefined}
                  onSubmitDecision={submitDecision}
                />
              </div>
            ) : null}
          </div>
        ) : (
          <GatedPanel
            capability={{
              enabled: false,
              key: "teacher_applications",
              label: "Teacher application review",
              permissions: ["REVIEW_TEACHER_APPLICATIONS"],
            }}
            icon={<UserCheck size={20} aria-hidden />}
            title="Teacher application review unavailable"
          />
        )
      ) : null}
    </ProductShell>
  );
}

export function AdminRewardAmountReviewRoute() {
  const route = useAdminSession();
  const workspace = useMemo(
    () => (route.session ? buildPlatformAdminWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const allowed = route.session ? hasPlatformAdminAccess(route.session) : false;
  const canView = hasPlatformPermission(workspace, "VIEW_REWARD_AUDIT");
  const canApprove = hasPlatformPermission(workspace, "APPROVE_REWARD_AMOUNT");
  const [candidates, setCandidates] = useState<PlatformRewardCandidatesResponse | null>(null);
  const [candidateError, setCandidateError] = useState<RouteError | null>(null);
  const [candidateState, setCandidateState] = useState<SectionState>("idle");
  const [searchInput, setSearchInput] = useState("");
  const [appliedSearch, setAppliedSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState("teacher_approved");
  const [offset, setOffset] = useState(0);
  const [selectedCandidateId, setSelectedCandidateId] = useState<number | null>(null);
  const [auditEvents, setAuditEvents] = useState<RewardAuditEvent[]>([]);
  const [auditError, setAuditError] = useState<RouteError | null>(null);
  const [auditState, setAuditState] = useState<SectionState>("idle");
  const [decisionAmount, setDecisionAmount] = useState("");
  const [decisionReason, setDecisionReason] = useState("");
  const [decisionStatus, setDecisionStatus] = useState<"approved" | "rejected">("approved");
  const [decisionError, setDecisionError] = useState<RouteError | null>(null);
  const [decisionState, setDecisionState] = useState<"idle" | "submitting" | "success" | "error">("idle");
  const selectedCandidate =
    candidates?.candidates.find((c) => c.id === selectedCandidateId) || candidates?.candidates[0] || null;

  const loadCandidates = useCallback(async () => {
    const token = route.token;
    if (!route.session || !token || !allowed || !canView) {
      return;
    }
    setCandidateState("loading");
    setCandidateError(null);
    try {
      const response = await fetchPlatformRewardCandidates({
        limit: ADMIN_TEACHER_APPLICATION_PAGE_SIZE,
        offset,
        search: appliedSearch,
        status: statusFilter,
        token,
      });
      setCandidates(response);
      setCandidateState("success");
      setSelectedCandidateId((current) => {
        if (current && response.candidates.some((c) => c.id === current)) {
          return current;
        }
        return response.candidates[0]?.id || null;
      });
    } catch (error) {
      setCandidates(null);
      setCandidateError(normalizeRouteError(error, "Reward candidate review queue could not be loaded."));
      setCandidateState("error");
    }
  }, [allowed, appliedSearch, canView, offset, route.session, route.token, statusFilter]);

  const loadAudit = useCallback(
    async (candidateId: number) => {
      const token = route.token;
      if (!token || !canView) {
        return;
      }
      setAuditState("loading");
      setAuditError(null);
      try {
        const events = await fetchRewardCandidateAudit({ candidateId, token });
        setAuditEvents(events);
        setAuditState("success");
      } catch (error) {
        setAuditEvents([]);
        setAuditError(normalizeRouteError(error, "Reward candidate audit could not be loaded."));
        setAuditState("error");
      }
    },
    [canView, route.token],
  );

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadCandidates(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadCandidates]);

  useEffect(() => {
    if (!selectedCandidate?.id) {
      const timeout = window.setTimeout(() => {
        setAuditEvents([]);
        setAuditState("idle");
      }, 0);
      return () => window.clearTimeout(timeout);
    }
    const timeout = window.setTimeout(() => void loadAudit(selectedCandidate.id), 0);
    return () => window.clearTimeout(timeout);
  }, [loadAudit, selectedCandidate?.id]);

  function applyFilters() {
    setOffset(0);
    setAppliedSearch(searchInput.trim());
  }

  function resetFilters() {
    setSearchInput("");
    setAppliedSearch("");
    setStatusFilter("teacher_approved");
    setOffset(0);
  }

  async function submitDecision(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = route.token;
    if (!token || !selectedCandidate) {
      return;
    }
    const trimmedReason = decisionReason.trim();
    const trimmedAmount = decisionAmount.trim();
    if (!trimmedReason) {
      setDecisionError({
        code: "validation_error",
        message: "Add a decision reason before changing a candidate.",
        status: 400,
      });
      setDecisionState("error");
      return;
    }
    if (decisionStatus === "approved" && (!trimmedAmount || Number.isNaN(Number(trimmedAmount)) || Number(trimmedAmount) < 0)) {
      setDecisionError({
        code: "validation_error",
        message: "Enter a valid non-negative approved amount.",
        status: 400,
      });
      setDecisionState("error");
      return;
    }
    setDecisionError(null);
    setDecisionState("submitting");
    try {
      await decideRewardAmount({
        candidateId: selectedCandidate.id,
        decisionReason: trimmedReason,
        status: decisionStatus,
        approvedAmount: decisionStatus === "approved" ? trimmedAmount : null,
        token,
      });
      setDecisionReason("");
      setDecisionAmount("");
      setDecisionState("success");
      await loadCandidates();
      if (selectedCandidate.id) {
        await loadAudit(selectedCandidate.id);
      }
    } catch (error) {
      const routeError = normalizeRouteError(error, "Reward amount decision could not be saved.");
      setDecisionError(routeError);
      setDecisionState("error");
      if (routeError.status === 409) {
        await loadCandidates();
        if (selectedCandidate.id) {
          await loadAudit(selectedCandidate.id);
        }
      }
    }
  }

  const notice: ShellNotice | null =
    decisionState === "success"
      ? {
          message: "The candidate was refreshed with the latest decision state.",
          title: "Decision saved",
          tone: "success",
        }
      : decisionState === "error" && decisionError
        ? {
            message: decisionError.message,
            title: decisionError.status === 409 ? "Candidate changed" : "Decision failed",
            tone: decisionError.status === 403 || decisionError.status === 409 ? "warn" : "error",
          }
        : null;

  const statusOptions: Array<{ label: string; value: string }> = [
    { label: "All statuses", value: "" },
    { label: "Pending teacher approval", value: "pending_teacher_approval" },
    { label: "Teacher approved", value: "teacher_approved" },
    { label: "Amount approved", value: "amount_approved" },
    { label: "Amount rejected", value: "amount_rejected" },
    { label: "Token pending", value: "token_pending" },
    { label: "Token confirmed", value: "token_confirmed" },
    { label: "Wallet credited", value: "wallet_credited" },
    { label: "Needs reconciliation", value: "needs_reconciliation" },
    { label: "Failed", value: "failed" },
  ];

  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Reward amount review" },
      ]}
      description="Review teacher-approved reward candidates, set approved amounts, and inspect audit history."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : canView ? "Review access" : "Review gated"} />
          <StatusPill label={`${candidates?.total ?? 0} candidates`} />
          <StatusPill label={canApprove ? "Approve enabled" : "Approve gated"} tone={canApprove ? "good" : "neutral"} />
        </>
      }
      title="Reward amount review"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !allowed ? <AdminDeniedState workspace={workspace} /> : null}

      {route.session && allowed ? (
        canView ? (
          <div className={styles.stack}>
            <section className={styles.summaryGrid} aria-label="Reward candidate summary">
              <SummaryCard icon={<UserCheck size={20} aria-hidden />} label="Teacher approved" value={candidates?.candidates.filter((c) => c.status === "teacher_approved").length ?? 0} />
              <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Amount approved" value={candidates?.candidates.filter((c) => c.status === "amount_approved").length ?? 0} />
              <SummaryCard icon={<XCircle size={20} aria-hidden />} label="Amount rejected" value={candidates?.candidates.filter((c) => c.status === "amount_rejected").length ?? 0} />
              <SummaryCard icon={<Clock3 size={20} aria-hidden />} label="Token pending" value={candidates?.candidates.filter((c) => c.status === "token_pending").length ?? 0} />
              <SummaryCard icon={<FileText size={20} aria-hidden />} label={candidateState === "loading" ? "Loading" : "Total visible"} value={candidates?.total ?? 0} />
            </section>
            <section className={styles.filterPanel} aria-label="Reward candidate filters">
              <label>
                <span>Search</span>
                <span className={styles.inputWithIcon}>
                  <Search size={17} aria-hidden />
                  <input
                    onChange={(event) => setSearchInput(event.target.value)}
                    onKeyDown={(event) => {
                      if (event.key === "Enter") {
                        event.preventDefault();
                        applyFilters();
                      }
                    }}
                    placeholder="Student, course, candidate ID"
                    type="search"
                    value={searchInput}
                  />
                </span>
              </label>
              <label>
                <span>Status</span>
                <select onChange={(event) => { setStatusFilter(event.target.value); setOffset(0); }} value={statusFilter}>
                  {statusOptions.map((option) => (
                    <option key={option.label} value={option.value}>
                      {option.label}
                    </option>
                  ))}
                </select>
              </label>
              <div className={styles.filterActions}>
                <button className={styles.primaryButton} onClick={applyFilters} type="button">
                  <Search size={16} aria-hidden />
                  Apply
                </button>
                <button className={styles.secondaryButton} onClick={resetFilters} type="button">
                  Reset
                </button>
                <button className={styles.secondaryButton} disabled={candidateState === "loading"} onClick={loadCandidates} type="button">
                  {candidateState === "loading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <RefreshCw size={16} aria-hidden />}
                  Refresh
                </button>
              </div>
            </section>
            {candidateState === "loading" || candidateState === "idle" ? <PanelLoading title="Loading reward candidates" /> : null}
            {candidateState === "error" ? <PanelError error={candidateError} onRetry={loadCandidates} title="Reward candidate queue failed" /> : null}
            {candidateState === "success" && candidates ? (
              <div className={styles.twoColumnWide}>
                <section className={styles.panel}>
                  <div className={styles.panelHeader}>
                    <Landmark size={20} aria-hidden />
                    <div>
                      <h2>Review queue</h2>
                      <p>{candidates.total} candidate{candidates.total === 1 ? "" : "s"} match the current filters.</p>
                    </div>
                  </div>
                  {candidates.candidates.length ? (
                    <div className={styles.rowList}>
                      {candidates.candidates.map((candidate) => (
                        <article className={`${styles.compactRow} ${selectedCandidateId === candidate.id ? styles.selectedRow : ""}`} key={candidate.id}>
                          <div>
                            <strong>{candidate.student.name}</strong>
                            <span>{candidate.student.email}</span>
                            <small>{candidate.course.title} · {formatUnderscoreLabel(candidate.event_type)}</small>
                          </div>
                          <div className={styles.rowMeta}>
                            <StatusPill label={formatUnderscoreLabel(candidate.status)} tone={candidateStatusTone(candidate.status)} />
                            <span>{formatDate(candidate.updated_at)}</span>
                            <button aria-label={`Review ${candidate.student.name}`} className={styles.secondaryButton} onClick={() => setSelectedCandidateId(candidate.id)} type="button">
                              Review
                            </button>
                          </div>
                        </article>
                      ))}
                    </div>
                  ) : (
                    <EmptyState text="No reward candidates match these filters." />
                  )}
                  <div className={styles.paginationRow}>
                    <button className={styles.secondaryButton} disabled={offset <= 0} onClick={() => setOffset(Math.max(0, offset - candidates.limit))} type="button">
                      Previous
                    </button>
                    <span>
                      Showing {candidates.candidates.length ? offset + 1 : 0}-{Math.min(offset + candidates.candidates.length, candidates.total)} of {candidates.total}
                    </span>
                    <button className={styles.secondaryButton} disabled={offset + candidates.limit >= candidates.total} onClick={() => setOffset(offset + candidates.limit)} type="button">
                      Next
                    </button>
                  </div>
                </section>
                <RewardCandidateDetail
                  candidate={selectedCandidate}
                  auditError={auditError}
                  auditEvents={auditEvents}
                  auditState={auditState}
                  canApprove={canApprove}
                  decisionAmount={decisionAmount}
                  decisionError={decisionError}
                  decisionReason={decisionReason}
                  decisionState={decisionState}
                  decisionStatus={decisionStatus}
                  onDecisionAmountChange={setDecisionAmount}
                  onDecisionReasonChange={setDecisionReason}
                  onDecisionStatusChange={setDecisionStatus}
                  onRefreshAudit={() => selectedCandidate ? void loadAudit(selectedCandidate.id) : undefined}
                  onSubmitDecision={submitDecision}
                />
              </div>
            ) : null}
          </div>
        ) : (
          <GatedPanel
            capability={{
              enabled: false,
              key: "reward_amount_review",
              label: "Reward amount review",
              permissions: ["VIEW_REWARD_AUDIT"],
            }}
            icon={<Landmark size={20} aria-hidden />}
            title="Reward amount review unavailable"
          />
        )
      ) : null}
    </ProductShell>
  );
}

const ADMIN_FRAUD_BLOCK_PAGE_SIZE = 10;

export function AdminFraudBlocksRoute() {
  const route = useAdminSession();
  const workspace = useMemo(
    () => (route.session ? buildPlatformAdminWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const allowed = route.session ? hasPlatformAdminAccess(route.session) : false;
  const canView = hasAnyPlatformPermission(workspace, [
    "VIEW_REWARD_AUDIT",
    "MANAGE_REWARD_FRAUD_BLOCKS",
  ]);
  const canCreate = hasAnyPlatformPermission(workspace, [
    "MANAGE_REWARD_FRAUD_BLOCKS",
    "BLOCK_REWARD_TEACHER",
    "BLOCK_REWARD_ORGANIZATION",
  ]);
  const canRevoke = hasPlatformPermission(workspace, "MANAGE_REWARD_FRAUD_BLOCKS");
  const [blocks, setBlocks] = useState<{ blocks: FraudBlockItem[]; limit: number; offset: number; total: number } | null>(null);
  const [blocksError, setBlocksError] = useState<RouteError | null>(null);
  const [blocksState, setBlocksState] = useState<SectionState>("idle");
  const [searchInput, setSearchInput] = useState("");
  const [appliedSearch, setAppliedSearch] = useState("");
  const [scopeFilter, setScopeFilter] = useState("");
  const [activeFilter, setActiveFilter] = useState<boolean | null>(true);
  const [offset, setOffset] = useState(0);
  const [selectedBlockId, setSelectedBlockId] = useState<number | null>(null);
  const [auditEvents, setAuditEvents] = useState<FraudBlockAuditEvent[]>([]);
  const [auditError, setAuditError] = useState<RouteError | null>(null);
  const [auditState, setAuditState] = useState<SectionState>("idle");
  const [createScopeType, setCreateScopeType] = useState("");
  const [createTargetId, setCreateTargetId] = useState("");
  const [createReason, setCreateReason] = useState("");
  const [createEvidence, setCreateEvidence] = useState("");
  const [createError, setCreateError] = useState<RouteError | null>(null);
  const [createState, setCreateState] = useState<"idle" | "submitting" | "success" | "error">("idle");
  const [revokeError, setRevokeError] = useState<RouteError | null>(null);
  const [revokeState, setRevokeState] = useState<"idle" | "submitting" | "success" | "error">("idle");

  const selectedBlock = blocks?.blocks.find((b) => b.id === selectedBlockId) || blocks?.blocks[0] || null;

  const loadBlocks = useCallback(async () => {
    const token = route.token;
    if (!route.session || !token || !allowed || !canView) {
      return;
    }
    setBlocksState("loading");
    setBlocksError(null);
    try {
      const response = await fetchFraudBlocks({
        active: activeFilter,
        limit: ADMIN_FRAUD_BLOCK_PAGE_SIZE,
        offset,
        scope_type: scopeFilter || null,
        token,
      });
      setBlocks(response);
      setBlocksState("success");
      setSelectedBlockId((current) => {
        if (current && response.blocks.some((b) => b.id === current)) {
          return current;
        }
        return response.blocks[0]?.id || null;
      });
    } catch (error) {
      setBlocks(null);
      setBlocksError(normalizeRouteError(error, "Fraud blocks could not be loaded."));
      setBlocksState("error");
    }
  }, [allowed, activeFilter, canView, offset, route.session, route.token, scopeFilter]);

  const loadAudit = useCallback(
    async (blockId: number) => {
      const token = route.token;
      if (!token || !canView) {
        return;
      }
      setAuditState("loading");
      setAuditError(null);
      try {
        const events = await fetchFraudBlockAudit({ blockId, token });
        setAuditEvents(events);
        setAuditState("success");
      } catch (error) {
        setAuditEvents([]);
        setAuditError(normalizeRouteError(error, "Fraud block audit could not be loaded."));
        setAuditState("error");
      }
    },
    [canView, route.token],
  );

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadBlocks(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadBlocks]);

  useEffect(() => {
    if (!selectedBlock?.id) {
      const timeout = window.setTimeout(() => {
        setAuditEvents([]);
        setAuditState("idle");
      }, 0);
      return () => window.clearTimeout(timeout);
    }
    const timeout = window.setTimeout(() => void loadAudit(selectedBlock.id), 0);
    return () => window.clearTimeout(timeout);
  }, [loadAudit, selectedBlock?.id]);

  function applyFilters() {
    setOffset(0);
    setAppliedSearch(searchInput.trim());
  }

  function resetFilters() {
    setSearchInput("");
    setAppliedSearch("");
    setScopeFilter("");
    setActiveFilter(true);
    setOffset(0);
  }

  function blockStatus(block: FraudBlockItem): FraudBlockStatus {
    if (block.revoked_at) return "revoked";
    if (block.expires_at && new Date(block.expires_at) <= new Date()) return "expired";
    return "active";
  }

  function canSubmitCreate() {
    return (
      createScopeType.trim().length > 0 &&
      createReason.trim().length > 0 &&
      createState !== "submitting"
    );
  }

  async function submitCreate(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = route.token;
    if (!token) return;
    const trimmedReason = createReason.trim();
    if (!trimmedReason) {
      setCreateError({ code: "validation_error", message: "A reason is required.", status: 400 });
      setCreateState("error");
      return;
    }
    if (!createScopeType) {
      setCreateError({ code: "validation_error", message: "Select a scope type.", status: 400 });
      setCreateState("error");
      return;
    }
    const id = createTargetId.trim();
    setCreateError(null);
    setCreateState("submitting");
    try {
      const options: FraudBlockCreateOptions = {
        scope_type: createScopeType,
        reason: trimmedReason,
        token,
      };
      if (id) {
        if (createScopeType === "teacher" && !Number.isNaN(Number(id))) {
          options.teacher_user_id = Number(id);
        } else if (createScopeType === "organization" && !Number.isNaN(Number(id))) {
          options.organization_id = Number(id);
        } else if (createScopeType === "course" && !Number.isNaN(Number(id))) {
          options.course_id = Number(id);
        } else if (createScopeType === "reward_policy" && !Number.isNaN(Number(id))) {
          options.reward_policy_id = Number(id);
        }
      }
      if (createEvidence.trim()) {
        options.evidence_reference = createEvidence.trim();
      }
      await createFraudBlock(options);
      setCreateState("success");
      setCreateReason("");
      setCreateTargetId("");
      setCreateEvidence("");
      setCreateScopeType("");
      await loadBlocks();
    } catch (error) {
      const routeError = normalizeRouteError(error, "Fraud block could not be created.");
      setCreateError(routeError);
      setCreateState("error");
    }
  }

  async function submitRevoke(blockId: number) {
    const token = route.token;
    if (!token) return;
    setRevokeError(null);
    setRevokeState("submitting");
    try {
      await revokeFraudBlock({ blockId, token });
      setRevokeState("success");
      await loadBlocks();
      if (selectedBlock?.id === blockId) {
        await loadAudit(blockId);
      }
    } catch (error) {
      const routeError = normalizeRouteError(error, "Fraud block could not be revoked.");
      setRevokeError(routeError);
      setRevokeState("error");
    }
  }

  const notice: ShellNotice | null =
    createState === "success"
      ? { message: "The fraud block was created.", title: "Block created", tone: "success" }
      : createState === "error" && createError
        ? { message: createError.message, title: "Create failed", tone: createError.status === 403 ? "warn" : "error" }
        : revokeState === "success"
          ? { message: "The fraud block was revoked.", title: "Block revoked", tone: "success" }
          : revokeState === "error" && revokeError
            ? { message: revokeError.message, title: "Revoke failed", tone: revokeError.status === 403 ? "warn" : "error" }
            : null;

  const scopeOptions = [
    { label: "All scopes", value: "" },
    { label: "Teacher", value: "teacher" },
    { label: "Organization", value: "organization" },
    { label: "Course", value: "course" },
    { label: "Reward policy", value: "reward_policy" },
  ];

  const activeOptions = [
    { label: "Active", value: "true" },
    { label: "Inactive", value: "false" },
    { label: "All", value: "" },
  ];

  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Fraud blocks" },
      ]}
      description="Manage reward fraud blocks by scope, create new blocks, revoke existing ones, and inspect audit history."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : canView ? "Fraud access" : "Fraud gated"} />
          <StatusPill label={`${blocks?.total ?? 0} blocks`} />
          <StatusPill label={canCreate ? "Create enabled" : "Create gated"} tone={canCreate ? "good" : "neutral"} />
        </>
      }
      title="Fraud blocks"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !allowed ? <AdminDeniedState workspace={workspace} /> : null}

      {route.session && allowed ? (
        canView ? (
          <div className={styles.stack}>
            <section className={styles.filterPanel} aria-label="Fraud block filters">
              <label>
                <span>Search</span>
                <span className={styles.inputWithIcon}>
                  <Search size={17} aria-hidden />
                  <input
                    onChange={(event) => setSearchInput(event.target.value)}
                    onKeyDown={(event) => {
                      if (event.key === "Enter") {
                        event.preventDefault();
                        applyFilters();
                      }
                    }}
                    placeholder="Block ID, reason"
                    type="search"
                    value={searchInput}
                  />
                </span>
              </label>
              <label>
                <span>Scope</span>
                <select onChange={(event) => { setScopeFilter(event.target.value); setOffset(0); }} value={scopeFilter}>
                  {scopeOptions.map((option) => (
                    <option key={option.label} value={option.value}>
                      {option.label}
                    </option>
                  ))}
                </select>
              </label>
              <label>
                <span>Status</span>
                <select
                  onChange={(event) => {
                    const value = event.target.value;
                    setActiveFilter(value === "" ? null : value === "true");
                    setOffset(0);
                  }}
                  value={activeFilter === true ? "true" : activeFilter === false ? "false" : ""}
                >
                  {activeOptions.map((option) => (
                    <option key={option.label} value={option.value}>
                      {option.label}
                    </option>
                  ))}
                </select>
              </label>
              <div className={styles.filterActions}>
                <button className={styles.primaryButton} onClick={applyFilters} type="button">
                  <Search size={16} aria-hidden />
                  Apply
                </button>
                <button className={styles.secondaryButton} onClick={resetFilters} type="button">
                  Reset
                </button>
                <button className={styles.secondaryButton} disabled={blocksState === "loading"} onClick={loadBlocks} type="button">
                  {blocksState === "loading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <RefreshCw size={16} aria-hidden />}
                  Refresh
                </button>
              </div>
            </section>
            {blocksState === "loading" || blocksState === "idle" ? <PanelLoading title="Loading fraud blocks" /> : null}
            {blocksState === "error" ? <PanelError error={blocksError} onRetry={loadBlocks} title="Fraud blocks failed" /> : null}
            {blocksState === "success" && blocks ? (
              <div className={styles.twoColumnWide}>
                <section className={styles.panel}>
                  <div className={styles.panelHeader}>
                    <ShieldAlert size={20} aria-hidden />
                    <div>
                      <h2>Fraud block list</h2>
                      <p>{blocks.total} block{blocks.total === 1 ? "" : "s"} match the current filters.</p>
                    </div>
                  </div>
                  {blocks.blocks.length ? (
                    <div className={styles.rowList}>
                      {blocks.blocks.map((block) => (
                        <article
                          className={`${styles.compactRow} ${selectedBlockId === block.id ? styles.selectedRow : ""}`}
                          key={block.id}
                        >
                          <div>
                            <strong>Block #{block.id}</strong>
                            <span>{block.reason}</span>
                            <small>
                              {formatUnderscoreLabel(block.scope_type)} · {targetLabel(block)} · {formatDate(block.created_at)}
                            </small>
                          </div>
                          <div className={styles.rowMeta}>
                            <StatusPill label={formatUnderscoreLabel(blockStatus(block))} tone={blockStatus(block) === "active" ? "warn" : "neutral"} />
                            <span>{formatDate(block.updated_at)}</span>
                            <button
                              aria-label={`Inspect block ${block.id}`}
                              className={styles.secondaryButton}
                              onClick={() => setSelectedBlockId(block.id)}
                              type="button"
                            >
                              Inspect
                            </button>
                          </div>
                        </article>
                      ))}
                    </div>
                  ) : (
                    <EmptyState text="No fraud blocks match these filters." />
                  )}
                  <div className={styles.paginationRow}>
                    <button className={styles.secondaryButton} disabled={offset <= 0} onClick={() => setOffset(Math.max(0, offset - blocks.limit))} type="button">
                      Previous
                    </button>
                    <span>
                      Showing {blocks.blocks.length ? offset + 1 : 0}-{Math.min(offset + blocks.blocks.length, blocks.total)} of {blocks.total}
                    </span>
                    <button className={styles.secondaryButton} disabled={offset + blocks.limit >= blocks.total} onClick={() => setOffset(offset + blocks.limit)} type="button">
                      Next
                    </button>
                  </div>
                </section>
                <FraudBlockDetail
                  block={selectedBlock}
                  auditError={auditError}
                  auditEvents={auditEvents}
                  auditState={auditState}
                  canRevoke={canRevoke}
                  createError={createError}
                  createEvidence={createEvidence}
                  createReason={createReason}
                  createScopeType={createScopeType}
                  createState={createState}
                  createTargetId={createTargetId}
                  onCreateEvidenceChange={setCreateEvidence}
                  onCreateReasonChange={setCreateReason}
                  onCreateScopeTypeChange={setCreateScopeType}
                  onCreateTargetIdChange={setCreateTargetId}
                  onRefreshAudit={() => selectedBlock ? void loadAudit(selectedBlock.id) : undefined}
                  onRevoke={submitRevoke}
                  onSubmitCreate={submitCreate}
                />
              </div>
            ) : null}
          </div>
        ) : (
          <GatedPanel
            capability={{
              enabled: false,
              key: "fraud_blocks",
              label: "Fraud blocks",
              permissions: ["VIEW_REWARD_AUDIT", "MANAGE_REWARD_FRAUD_BLOCKS"],
            }}
            icon={<ShieldAlert size={20} aria-hidden />}
            title="Fraud blocks unavailable"
          />
        )
      ) : null}
    </ProductShell>
  );
}

function FraudBlockDetail({
  block,
  auditError,
  auditEvents,
  auditState,
  canRevoke,
  createError,
  createEvidence,
  createReason,
  createScopeType,
  createState,
  createTargetId,
  onCreateEvidenceChange,
  onCreateReasonChange,
  onCreateScopeTypeChange,
  onCreateTargetIdChange,
  onRefreshAudit,
  onRevoke,
  onSubmitCreate,
}: {
  block: FraudBlockItem | null;
  auditError: RouteError | null;
  auditEvents: FraudBlockAuditEvent[];
  auditState: SectionState;
  canRevoke: boolean;
  createError: RouteError | null;
  createEvidence: string;
  createReason: string;
  createScopeType: string;
  createState: "idle" | "submitting" | "success" | "error";
  createTargetId: string;
  onCreateEvidenceChange: (value: string) => void;
  onCreateReasonChange: (value: string) => void;
  onCreateScopeTypeChange: (value: string) => void;
  onCreateTargetIdChange: (value: string) => void;
  onRefreshAudit: () => void;
  onRevoke: (blockId: number) => void;
  onSubmitCreate: (event: FormEvent<HTMLFormElement>) => void;
}) {
  const scopeCreateOptions = [
    { label: "Select scope…", value: "" },
    { label: "Teacher", value: "teacher" },
    { label: "Organization", value: "organization" },
    { label: "Course", value: "course" },
    { label: "Reward policy", value: "reward_policy" },
  ];

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldAlert size={20} aria-hidden />
        <div>
          <h2>{block ? `Block ${block.id}` : "Create block"}</h2>
          <p>{block ? "Inspect context and audit history." : "Add a new fraud block."}</p>
        </div>
        {block ? <StatusPill label={block.revoked_at ? "Revoked" : block.expires_at && new Date(block.expires_at) <= new Date() ? "Expired" : "Active"} tone={block.revoked_at ? "neutral" : "warn"} /> : null}
      </div>

      {!block ? (
        <form className={styles.decisionForm} onSubmit={onSubmitCreate}>
          <div className={styles.subsectionHeader}>
            <h3>New fraud block</h3>
          </div>
          <label>
            <span>Scope type</span>
            <select onChange={(event) => onCreateScopeTypeChange(event.target.value)} value={createScopeType}>
              {scopeCreateOptions.map((option) => (
                <option key={option.label} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>Target ID</span>
            <input
              onChange={(event) => onCreateTargetIdChange(event.target.value)}
              placeholder="Optional numeric ID for the scope target"
              type="text"
              value={createTargetId}
            />
          </label>
          <label>
            <span>Reason</span>
            <textarea
              onChange={(event) => onCreateReasonChange(event.target.value)}
              placeholder="Explain why this fraud block is being created."
              rows={4}
              value={createReason}
            />
          </label>
          <label>
            <span>Evidence reference</span>
            <input
              onChange={(event) => onCreateEvidenceChange(event.target.value)}
              placeholder="Optional case or ticket reference"
              type="text"
              value={createEvidence}
            />
          </label>
          {createError ? (
            <div className={styles.inlineError} role="alert">
              <AlertTriangle size={16} aria-hidden />
              <span>{createError.message}</span>
            </div>
          ) : null}
          <button className={styles.primaryButton} disabled={createState === "submitting" || !createScopeType || !createReason.trim()} type="submit">
            {createState === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
            Create block
          </button>
        </form>
      ) : null}

      {block ? (
        <>
          <div className={styles.detailGrid}>
            <ContextRow label="Scope type" value={formatUnderscoreLabel(block.scope_type)} />
            <ContextRow label="Target" value={targetLabel(block)} />
            <ContextRow label="Reason" value={block.reason} />
            <ContextRow label="Evidence" value={block.evidence_reference || "None"} />
            <ContextRow label="Created" value={formatDate(block.created_at)} />
            <ContextRow label="Updated" value={formatDate(block.updated_at)} />
          </div>
          <div className={styles.textBlock}>
            <h3>Audit history</h3>
            <button className={styles.secondaryButton} disabled={auditState === "loading"} onClick={onRefreshAudit} type="button">
              {auditState === "loading" ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <RefreshCw size={15} aria-hidden />}
              Refresh
            </button>
            {auditState === "loading" || auditState === "idle" ? (
              <p className={styles.muted}>Loading audit events.</p>
            ) : null}
            {auditState === "error" ? (
              <div className={styles.inlineError} role="alert">
                <AlertTriangle size={16} aria-hidden />
                <span>{auditError?.message || "Audit history could not be loaded."}</span>
              </div>
            ) : null}
            {auditState === "success" && auditEvents.length ? (
              <ol className={styles.auditList}>
                {auditEvents.map((event) => (
                  <li key={event.id}>
                    <strong>{formatUnderscoreLabel(event.event_type)}</strong>
                    <span>{formatDate(event.created_at)}</span>
                    {event.reason ? <small>{event.reason}</small> : null}
                  </li>
                ))}
              </ol>
            ) : null}
            {auditState === "success" && !auditEvents.length ? <EmptyState text="No audit events were returned for this block." /> : null}
          </div>
          {!block.revoked_at && canRevoke ? (
            <div className={styles.textBlock}>
              <h3>Revoke block</h3>
              <button className={styles.primaryButton} disabled={false} onClick={() => onRevoke(block.id)} type="button">
                Revoke
              </button>
            </div>
          ) : null}
          {block.revoked_at ? (
            <p className={styles.muted}>This block was revoked and cannot be changed.</p>
          ) : null}
          {!block.revoked_at && !canRevoke ? (
            <p className={styles.muted}>Revoke requires the MANAGE_REWARD_FRAUD_BLOCKS permission.</p>
          ) : null}
        </>
      ) : null}
    </section>
  );
}

function targetLabel(block: FraudBlockItem | PlatformFraudDashboard["active_blocks"][number]) {
  if (block.teacher_user_id) {
    return `Teacher user ${block.teacher_user_id}`;
  }
  if (block.organization_id) {
    return `Organization ${block.organization_id}`;
  }
  if (block.course_id) {
    return `Course ${block.course_id}`;
  }
  if (block.reward_policy_id) {
    return `Reward policy ${block.reward_policy_id}`;
  }
  return "Scope target unavailable";
}

function SummaryPanel({
  canViewSummary,
  error,
  onRetry,
  state,
  summary,
  workspace,
}: {
  canViewSummary: boolean;
  error: RouteError | null;
  onRetry: () => void;
  state: SectionState;
  summary: PlatformReportSummary | null;
  workspace: PlatformAdminWorkspace;
}) {
  if (!canViewSummary) {
    return (
      <GatedPanel
        capability={getCapability(workspace, "summary")}
        icon={<Gauge size={20} aria-hidden />}
        title="Platform summary unavailable"
      />
    );
  }

  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading platform summary" />;
  }

  if (state === "error" || !summary) {
    return <PanelError error={error} onRetry={onRetry} title="Platform summary failed" />;
  }

  return (
    <section className={styles.summaryGrid} aria-label="Platform summary">
      <SummaryCard icon={<Users size={20} aria-hidden />} label="Users" value={summary.total_users} />
      <SummaryCard icon={<Landmark size={20} aria-hidden />} label="Organizations" value={summary.total_organizations} />
      <SummaryCard icon={<FileText size={20} aria-hidden />} label="Courses" value={summary.total_courses} />
      <SummaryCard icon={<WalletCards size={20} aria-hidden />} label="Wallets" value={summary.total_wallets} />
      <SummaryCard icon={<Gauge size={20} aria-hidden />} label="Notifications" value={summary.total_notifications} />
    </section>
  );
}

function ActionPanel({
  canApproveRewardAmount,
  canExportData,
  canManageFraud,
  canViewRewardAudit,
  fraudDashboard,
  rewardDashboard,
  systemStatus,
  workspace,
}: {
  canApproveRewardAmount: boolean;
  canExportData: boolean;
  canManageFraud: boolean;
  canViewRewardAudit: boolean;
  fraudDashboard: PlatformFraudDashboard | null;
  rewardDashboard: PlatformRewardDashboard | null;
  systemStatus: PlatformSystemStatus | null;
  workspace: PlatformAdminWorkspace;
}) {
  const readinessTone = systemStatus?.readiness.status === "ready" ? "good" : systemStatus ? "warn" : "neutral";
  const cards: Array<{
    detail: string;
    key: PlatformCapabilityKey;
    label: string;
    state: string;
    tone: "good" | "neutral" | "warn";
    value: string | number;
  }> = [
    {
      detail: "Submitted teacher applications waiting for platform review.",
      key: "teacher_applications",
      label: "Teacher review",
      state: capabilityLabel(workspace, "teacher_applications"),
      tone: rewardDashboard?.teacher_applications.submitted ? "warn" : "neutral",
      value: rewardDashboard?.teacher_applications.submitted ?? "Gated",
    },
    {
      detail: "Teacher-approved candidates waiting for platform amount decision.",
      key: "reward_amount_review",
      label: "Amount review",
      state: canApproveRewardAmount ? "Approval enabled" : canViewRewardAudit ? "Audit only" : "Missing reward audit",
      tone: rewardDashboard?.pending_amount_approval_count ? "warn" : "neutral",
      value: rewardDashboard?.pending_amount_approval_count ?? "Gated",
    },
    {
      detail: "Active reward fraud blocks across teachers, organizations, courses, and policies.",
      key: "fraud_blocks",
      label: "Fraud controls",
      state: canManageFraud ? "Manage enabled" : canViewRewardAudit ? "Audit only" : "Missing reward audit",
      tone: fraudDashboard?.active_total ? "warn" : "neutral",
      value: fraudDashboard?.active_total ?? "Gated",
    },
    {
      detail: "CSV exports available for reports, reward operations, wallets, and delegations.",
      key: "exports",
      label: "Exports",
      state: canExportData ? "CSV enabled" : "Missing EXPORT_DATA",
      tone: canExportData ? "good" : "neutral",
      value: canExportData ? "Ready" : "Gated",
    },
    {
      detail: "Wallet and transaction audit permissions visible in the current platform scope.",
      key: "wallets",
      label: "Wallet audit",
      state: capabilityLabel(workspace, "wallets"),
      tone: platformCapabilityEnabled(workspace, "wallets") ? "good" : "neutral",
      value: platformCapabilityEnabled(workspace, "wallets") ? "Available" : "Gated",
    },
    {
      detail: "API liveness and dependency readiness from the runtime health endpoints.",
      key: "system",
      label: "System",
      state: systemStatus?.readiness.status ? formatUnderscoreLabel(systemStatus.readiness.status) : "Loading",
      tone: readinessTone,
      value: systemStatus?.liveness.status || "Checking",
    },
  ];

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <div>
          <h2>Review lanes</h2>
          <p>Counts and gated lanes follow resolved platform permissions from the current session.</p>
        </div>
      </div>
      <div className={styles.actionGrid}>
        {cards.map((card) => {
          const Icon = actionIcons[card.key];
          return (
            <article className={styles.actionCard} key={card.key}>
              <div className={styles.actionTop}>
                <span className={styles.smallIcon}>
                  <Icon size={19} aria-hidden />
                </span>
                <StatusPill label={card.state} tone={card.tone} />
              </div>
              <strong>{card.value}</strong>
              <span>{card.label}</span>
              <p>{card.detail}</p>
            </article>
          );
        })}
      </div>
    </section>
  );
}

function RewardPanel({
  canApproveRewardAmount,
  canViewRewardAudit,
  dashboard,
  error,
  onRetry,
  state,
  workspace,
}: {
  canApproveRewardAmount: boolean;
  canViewRewardAudit: boolean;
  dashboard: PlatformRewardDashboard | null;
  error: RouteError | null;
  onRetry: () => void;
  state: SectionState;
  workspace: PlatformAdminWorkspace;
}) {
  if (!canViewRewardAudit) {
    return (
      <GatedPanel
        capability={{
          enabled: false,
          key: "reward_amount_review",
          label: "Reward audit",
          permissions: ["VIEW_REWARD_AUDIT"],
        }}
        icon={<Landmark size={20} aria-hidden />}
        title="Reward operations unavailable"
      />
    );
  }

  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading reward operations" />;
  }

  if (state === "error" || !dashboard) {
    return <PanelError error={error} onRetry={onRetry} title="Reward operations failed" />;
  }

  const rewardStatuses = [
    ["Teacher pending", dashboard.reward_candidates.pending_teacher_approval],
    ["Teacher approved", dashboard.reward_candidates.teacher_approved],
    ["Amount approved", dashboard.reward_candidates.amount_approved],
    ["Token pending", dashboard.reward_candidates.token_pending],
    ["Wallet credited", dashboard.reward_candidates.wallet_credited],
    ["Needs reconciliation", dashboard.reward_candidates.needs_reconciliation],
    ["Failed", dashboard.reward_candidates.failed],
  ];

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Landmark size={20} aria-hidden />
        <div>
          <h2>Reward operations</h2>
          <p>Teacher approval, platform amount review, payout failures, and reconciliation stay separated.</p>
        </div>
        <StatusPill label={canApproveRewardAmount ? "Amount approval enabled" : "Audit only"} tone={canApproveRewardAmount ? "good" : "neutral"} />
      </div>

      <div className={styles.metricGrid}>
        <MetricCard label="Teacher applications submitted" value={dashboard.teacher_applications.submitted} />
        <MetricCard label="Pending amount approval" value={dashboard.pending_amount_approval_count} />
        <MetricCard label="Payout failures" value={dashboard.payout_failure_count} tone={dashboard.payout_failure_count ? "warn" : "neutral"} />
        <MetricCard label="Reconciliation mismatches" value={dashboard.reconciliation_mismatch_count} tone={dashboard.reconciliation_mismatch_count ? "warn" : "neutral"} />
      </div>

      <div className={styles.statusList}>
        {rewardStatuses.map(([label, value]) => (
          <div className={styles.statusRow} key={label}>
            <span>{label}</span>
            <strong>{formatNumber(value as number)}</strong>
          </div>
        ))}
      </div>

      <div className={styles.subsectionHeader}>
        <h3>Amount-ready candidates</h3>
        <StatusPill label={`${dashboard.pending_amount_approvals.length} recent`} />
      </div>
      {dashboard.pending_amount_approvals.length ? (
        <div className={styles.rowList}>
          {dashboard.pending_amount_approvals.slice(0, 6).map((candidate) => (
            <article className={styles.compactRow} key={candidate.reward_candidate_id}>
              <div>
                <strong>Candidate {candidate.reward_candidate_id}</strong>
                <span>
                  Course {candidate.course_id} · Student {candidate.student_user_id} · {formatUnderscoreLabel(candidate.event_type)}
                </span>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label={formatUnderscoreLabel(candidate.status)} tone="warn" />
                <span>{formatDate(candidate.updated_at)}</span>
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No teacher-approved candidates are waiting for platform amount review." />
      )}

      <RiskList dashboard={dashboard} />
      {!platformCapabilityEnabled(workspace, "teacher_applications") ? (
        <p className={styles.muted}>Teacher application review is gated for this session.</p>
      ) : null}
    </section>
  );
}

function FraudPanel({
  canManageFraud,
  canViewRewardAudit,
  dashboard,
  error,
  onRetry,
  state,
  workspace,
}: {
  canManageFraud: boolean;
  canViewRewardAudit: boolean;
  dashboard: PlatformFraudDashboard | null;
  error: RouteError | null;
  onRetry: () => void;
  state: SectionState;
  workspace: PlatformAdminWorkspace;
}) {
  if (!canViewRewardAudit) {
    return (
      <GatedPanel
        capability={{
          enabled: false,
          key: "fraud_blocks",
          label: "Fraud dashboard",
          permissions: ["VIEW_REWARD_AUDIT"],
        }}
        icon={<ShieldAlert size={20} aria-hidden />}
        title="Fraud dashboard unavailable"
      />
    );
  }

  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading fraud controls" />;
  }

  if (state === "error" || !dashboard) {
    return <PanelError error={error} onRetry={onRetry} title="Fraud controls failed" />;
  }

  const scopeRows = [
    ["Teachers", dashboard.active_by_scope.teacher],
    ["Organizations", dashboard.active_by_scope.organization],
    ["Courses", dashboard.active_by_scope.course],
    ["Reward policies", dashboard.active_by_scope.reward_policy],
  ];

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldAlert size={20} aria-hidden />
        <div>
          <h2>Fraud controls</h2>
          <p>Active blocks are separated by target scope and keep reason/evidence context visible.</p>
        </div>
        <StatusPill label={canManageFraud ? "Manage enabled" : "Audit only"} tone={canManageFraud ? "good" : "neutral"} />
      </div>

      <div className={styles.metricGrid}>
        <MetricCard label="Active blocks" value={dashboard.active_total} tone={dashboard.active_total ? "warn" : "neutral"} />
        {scopeRows.map(([label, value]) => (
          <MetricCard key={label} label={label as string} value={value as number} />
        ))}
      </div>

      <div className={styles.subsectionHeader}>
        <h3>Recent active blocks</h3>
        <StatusPill label={`${dashboard.active_blocks.length} active`} tone={dashboard.active_blocks.length ? "warn" : "good"} />
      </div>
      {dashboard.active_blocks.length ? (
        <div className={styles.rowList}>
          {dashboard.active_blocks.slice(0, 7).map((block) => (
            <article className={styles.compactRow} key={block.id}>
              <div>
                <strong>{formatUnderscoreLabel(block.scope_type)} block {block.id}</strong>
                <span>{block.reason}</span>
                <small>{targetLabel(block)}</small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label={block.expires_at ? `Expires ${formatDate(block.expires_at)}` : "No expiry"} tone={block.expires_at ? "neutral" : "warn"} />
                {block.evidence_reference ? <span>Evidence: {block.evidence_reference}</span> : null}
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No active fraud blocks are currently visible." />
      )}
      {!platformCapabilityEnabled(workspace, "fraud_blocks") ? (
        <p className={styles.muted}>Fraud block create and revoke actions are gated for this session.</p>
      ) : null}
    </section>
  );
}

function ExportPanel({
  canExportData,
  csvState,
  onDownload,
  workspace,
}: {
  canExportData: boolean;
  csvState: CsvState;
  onDownload: (report: PlatformCsvReport, label: string) => void;
  workspace: PlatformAdminWorkspace;
}) {
  if (!canExportData) {
    return (
      <GatedPanel
        capability={getCapability(workspace, "exports")}
        icon={<Download size={20} aria-hidden />}
        title="CSV exports unavailable"
      />
    );
  }

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Download size={20} aria-hidden />
        <div>
          <h2>CSV exports</h2>
          <p>Downloads use the platform export endpoints and keep failures visible on the page.</p>
        </div>
      </div>
      <div className={styles.exportList}>
        {exportReports.map((report) => (
          <article className={styles.exportRow} key={report.report}>
            <div>
              <strong>{report.label}</strong>
              <span>{report.description}</span>
            </div>
            <button
              className={styles.secondaryButton}
              disabled={csvState === "downloading"}
              onClick={() => onDownload(report.report, report.label)}
              type="button"
            >
              {csvState === "downloading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Download size={16} aria-hidden />}
              CSV
            </button>
          </article>
        ))}
      </div>
    </section>
  );
}

function SystemPanel({
  error,
  onRetry,
  state,
  status,
}: {
  error: RouteError | null;
  onRetry: () => void;
  state: SectionState;
  status: PlatformSystemStatus | null;
}) {
  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading system status" />;
  }

  if (state === "error" || !status) {
    return <PanelError error={error} onRetry={onRetry} title="System status failed" />;
  }

  const ready = status.readiness.status === "ready";

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Database size={20} aria-hidden />
        <div>
          <h2>System status</h2>
          <p>Readiness checks cover API dependencies instead of only shallow liveness.</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(status.readiness.status)} tone={ready ? "good" : "warn"} />
      </div>
      <div className={styles.metricGrid}>
        <MetricCard label="API liveness" value={formatUnderscoreLabel(status.liveness.status)} tone={status.liveness.status === "ok" ? "good" : "warn"} />
        <MetricCard label="Readiness" value={formatUnderscoreLabel(status.readiness.status)} tone={ready ? "good" : "warn"} />
      </div>
      <div className={styles.rowList}>
        {status.readiness.checks.map((check) => (
          <article className={styles.compactRow} key={check.name}>
            <div>
              <strong>{dependencyLabel(check.name)}</strong>
              <span>{check.message || "Dependency check passed."}</span>
            </div>
            <StatusPill
              icon={check.status === "ok" ? <CheckCircle2 size={15} aria-hidden /> : <XCircle size={15} aria-hidden />}
              label={formatUnderscoreLabel(check.status)}
              tone={check.status === "ok" ? "good" : "warn"}
            />
          </article>
        ))}
      </div>
    </section>
  );
}

function RiskList({ dashboard }: { dashboard: PlatformRewardDashboard }) {
  const risks = [
    ...dashboard.payout_failures.slice(0, 3).map((failure) => ({
      detail: failure.last_error || "No backend error message was recorded.",
      key: `payout-${failure.reward_execution_job_id}`,
      label: `Payout job ${failure.reward_execution_job_id}`,
      meta: `Candidate ${failure.reward_candidate_id} · ${failure.attempts} attempts`,
      tone: "warn" as const,
      updatedAt: failure.updated_at,
    })),
    ...dashboard.reconciliation_mismatches.slice(0, 3).map((mismatch) => ({
      detail: formatUnderscoreLabel(mismatch.mismatch_type),
      key: `reconciliation-${mismatch.reward_candidate_id}`,
      label: `Candidate ${mismatch.reward_candidate_id}`,
      meta: `Course ${mismatch.course_id} · Student ${mismatch.student_user_id}`,
      tone: "warn" as const,
      updatedAt: mismatch.updated_at,
    })),
  ];

  return (
    <>
      <div className={styles.subsectionHeader}>
        <h3>Operational risks</h3>
        <StatusPill label={`${risks.length} visible`} tone={risks.length ? "warn" : "good"} />
      </div>
      {risks.length ? (
        <div className={styles.rowList}>
          {risks.map((risk) => (
            <article className={styles.compactRow} key={risk.key}>
              <div>
                <strong>{risk.label}</strong>
                <span>{risk.detail}</span>
                <small>{risk.meta}</small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label="Needs attention" tone={risk.tone} />
                <span>{formatDate(risk.updatedAt)}</span>
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No payout failures or reconciliation mismatches are visible." />
      )}
    </>
  );
}

function TeacherApplicationSummaryPanel({
  response,
  state,
}: {
  response: PlatformTeacherApplicationsResponse | null;
  state: SectionState;
}) {
  const summary = response?.summary;

  return (
    <section className={styles.summaryGrid} aria-label="Teacher application review summary">
      <SummaryCard icon={<UserCheck size={20} aria-hidden />} label="Submitted" value={summary?.submitted || 0} />
      <SummaryCard icon={<Clock3 size={20} aria-hidden />} label="Needs changes" value={summary?.needs_changes || 0} />
      <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Approved" value={summary?.approved || 0} />
      <SummaryCard icon={<XCircle size={20} aria-hidden />} label="Rejected" value={summary?.rejected || 0} />
      <SummaryCard icon={<FileText size={20} aria-hidden />} label={state === "loading" ? "Loading" : "Total"} value={summary?.total || 0} />
    </section>
  );
}

function TeacherApplicationFilterPanel({
  onApply,
  onRefresh,
  onReset,
  searchInput,
  setSearchInput,
  setStatusFilter,
  state,
  statusFilter,
}: {
  onApply: () => void;
  onRefresh: () => void;
  onReset: () => void;
  searchInput: string;
  setSearchInput: (value: string) => void;
  setStatusFilter: (value: TeacherApplicationStatus | "") => void;
  state: SectionState;
  statusFilter: TeacherApplicationStatus | "";
}) {
  return (
    <section className={styles.filterPanel} aria-label="Teacher application filters">
      <label>
        <span>Search</span>
        <span className={styles.inputWithIcon}>
          <Search size={17} aria-hidden />
          <input
            onChange={(event) => setSearchInput(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.preventDefault();
                onApply();
              }
            }}
            placeholder="Applicant, email, sponsor, course, status"
            type="search"
            value={searchInput}
          />
        </span>
      </label>
      <label>
        <span>Status</span>
        <select
          onChange={(event) => setStatusFilter(event.target.value as TeacherApplicationStatus | "")}
          value={statusFilter}
        >
          {teacherApplicationStatusOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <div className={styles.filterActions}>
        <button className={styles.primaryButton} onClick={onApply} type="button">
          <Search size={16} aria-hidden />
          Apply
        </button>
        <button className={styles.secondaryButton} onClick={onReset} type="button">
          Reset
        </button>
        <button className={styles.secondaryButton} disabled={state === "loading"} onClick={onRefresh} type="button">
          {state === "loading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <RefreshCw size={16} aria-hidden />}
          Refresh
        </button>
      </div>
    </section>
  );
}

function TeacherApplicationQueue({
  applications,
  offset,
  onPage,
  onSelect,
  selectedApplicationId,
}: {
  applications: PlatformTeacherApplicationsResponse;
  offset: number;
  onPage: (offset: number) => void;
  onSelect: (applicationId: number) => void;
  selectedApplicationId: number | null;
}) {
  const hasPrevious = offset > 0;
  const hasNext = offset + applications.limit < applications.total;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <UserCheck size={20} aria-hidden />
        <div>
          <h2>Review queue</h2>
          <p>
            {applications.total} application{applications.total === 1 ? "" : "s"}{" "}
            {applications.total === 1 ? "matches" : "match"} the current filters.
          </p>
        </div>
      </div>

      {applications.applications.length ? (
        <div className={styles.rowList}>
          {applications.applications.map((application) => (
            <article
              className={`${styles.compactRow} ${selectedApplicationId === application.id ? styles.selectedRow : ""}`}
              key={application.id}
            >
              <div>
                <strong>{application.applicant.name}</strong>
                <span>{application.applicant.email}</span>
                <small>
                  {formatUnderscoreLabel(application.requested_scope)} · {scopeTargetLabel(application)}
                </small>
                <small>{application.experience_summary}</small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label={formatUnderscoreLabel(application.status)} tone={statusTone(application.status)} />
                <span>{formatDate(application.updated_at)}</span>
                <button
                  aria-label={`Review ${application.applicant.name}`}
                  className={styles.secondaryButton}
                  onClick={() => onSelect(application.id)}
                  type="button"
                >
                  Review
                </button>
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No teacher applications match these filters." />
      )}

      <div className={styles.paginationRow}>
        <button className={styles.secondaryButton} disabled={!hasPrevious} onClick={() => onPage(Math.max(0, offset - applications.limit))} type="button">
          Previous
        </button>
        <span>
          Showing {applications.applications.length ? offset + 1 : 0}-
          {Math.min(offset + applications.applications.length, applications.total)} of {applications.total}
        </span>
        <button className={styles.secondaryButton} disabled={!hasNext} onClick={() => onPage(offset + applications.limit)} type="button">
          Next
        </button>
      </div>
    </section>
  );
}

function TeacherApplicationDetail({
  application,
  auditError,
  auditEvents,
  auditState,
  canApprove,
  canReject,
  decisionError,
  decisionReason,
  decisionState,
  decisionStatus,
  onDecisionReasonChange,
  onDecisionStatusChange,
  onRefreshAudit,
  onSubmitDecision,
}: {
  application: PlatformTeacherApplicationItem | null;
  auditError: RouteError | null;
  auditEvents: TeacherApplicationAuditEvent[];
  auditState: SectionState;
  canApprove: boolean;
  canReject: boolean;
  decisionError: RouteError | null;
  decisionReason: string;
  decisionState: "idle" | "submitting" | "success" | "error";
  decisionStatus: Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">;
  onDecisionReasonChange: (value: string) => void;
  onDecisionStatusChange: (value: Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">) => void;
  onRefreshAudit: () => void;
  onSubmitDecision: (event: FormEvent<HTMLFormElement>) => void;
}) {
  if (!application) {
    return (
      <section className={styles.panel}>
        <div className={styles.panelHeader}>
          <UserCheck size={20} aria-hidden />
          <div>
            <h2>Application detail</h2>
            <p>Select an application from the queue to inspect context and audit history.</p>
          </div>
        </div>
      </section>
    );
  }

  const isFinal = isFinalApplicationStatus(application.status);
  const canSubmitDecision =
    !isFinal &&
    decisionReason.trim().length > 0 &&
    (decisionStatus === "needs_changes" || (decisionStatus === "approved" && canApprove) || (decisionStatus === "rejected" && canReject)) &&
    decisionState !== "submitting";

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <FileText size={20} aria-hidden />
        <div>
          <h2>{application.applicant.name}</h2>
          <p>{application.applicant.email}</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(application.status)} tone={statusTone(application.status)} />
      </div>

      <div className={styles.detailGrid}>
        <ContextRow label="Requested scope" value={formatUnderscoreLabel(application.requested_scope)} />
        <ContextRow label="Target" value={scopeTargetLabel(application)} />
        <ContextRow label="Sponsor" value={application.sponsor_organization?.name || "No sponsor recorded"} />
        <ContextRow label="Submitted" value={formatDate(application.created_at)} />
        <ContextRow label="Latest update" value={formatDate(application.updated_at)} />
        <ContextRow label="Reviewer" value={application.reviewer?.name || "No reviewer yet"} />
      </div>

      <div className={styles.textBlock}>
        <h3>Experience summary</h3>
        <p>{application.experience_summary}</p>
      </div>

      <div className={styles.textBlock}>
        <h3>Portfolio</h3>
        {application.portfolio_links.length ? (
          <div className={styles.linkList}>
            {application.portfolio_links.map((link) => (
              <a href={link} key={link} rel="noreferrer" target="_blank">
                {link}
              </a>
            ))}
          </div>
        ) : (
          <p className={styles.muted}>No portfolio links were submitted.</p>
        )}
      </div>

      {application.decision_reason ? (
        <div className={styles.inlineNotice}>
          <strong>Latest decision note</strong>
          <span>{application.decision_reason}</span>
        </div>
      ) : null}

      <AuditTimeline
        auditError={auditError}
        auditEvents={auditEvents}
        auditState={auditState}
        onRefreshAudit={onRefreshAudit}
      />

      <form className={styles.decisionForm} onSubmit={onSubmitDecision}>
        <div className={styles.subsectionHeader}>
          <h3>Decision</h3>
          {isFinal ? <StatusPill label="Final state" tone="neutral" /> : null}
        </div>
        <label>
          <span>Decision status</span>
          <select
            disabled={isFinal}
            onChange={(event) =>
              onDecisionStatusChange(event.target.value as Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">)
            }
            value={decisionStatus}
          >
            <option value="needs_changes">Needs changes</option>
            <option disabled={!canApprove} value="approved">
              Approve
            </option>
            <option disabled={!canReject} value="rejected">
              Reject
            </option>
          </select>
        </label>
        {!canApprove || !canReject ? (
          <p className={styles.muted}>
            Approve and reject require the matching platform permissions; requesting changes remains available to reviewers.
          </p>
        ) : null}
        <label>
          <span>Decision reason</span>
          <textarea
            disabled={isFinal}
            onChange={(event) => onDecisionReasonChange(event.target.value)}
            placeholder="Explain what changed, what is missing, or why the application is approved."
            rows={4}
            value={decisionReason}
          />
        </label>
        {!isFinal && !decisionReason.trim() ? (
          <p className={styles.muted}>A decision reason is required before saving.</p>
        ) : null}
        {decisionError ? (
          <div className={styles.inlineError} role="alert">
            <AlertTriangle size={16} aria-hidden />
            <span>{decisionError.message}</span>
          </div>
        ) : null}
        <button className={styles.primaryButton} disabled={!canSubmitDecision} type="submit">
          {decisionState === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
          Save decision
        </button>
        {isFinal ? <p className={styles.muted}>Approved and rejected applications are final in the current backend contract.</p> : null}
      </form>
    </section>
  );
}

function AuditTimeline({
  auditError,
  auditEvents,
  auditState,
  onRefreshAudit,
}: {
  auditError: RouteError | null;
  auditEvents: TeacherApplicationAuditEvent[];
  auditState: SectionState;
  onRefreshAudit: () => void;
}) {
  return (
    <div className={styles.textBlock}>
      <div className={styles.subsectionHeader}>
        <h3>Audit history</h3>
        <button className={styles.secondaryButton} disabled={auditState === "loading"} onClick={onRefreshAudit} type="button">
          {auditState === "loading" ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <RefreshCw size={15} aria-hidden />}
          Refresh
        </button>
      </div>
      {auditState === "loading" || auditState === "idle" ? (
        <p className={styles.muted}>Loading audit events.</p>
      ) : null}
      {auditState === "error" ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{auditError?.message || "Audit history could not be loaded."}</span>
        </div>
      ) : null}
      {auditState === "success" && auditEvents.length ? (
        <ol className={styles.auditList}>
          {auditEvents.map((event) => (
            <li key={event.id}>
              <strong>{formatUnderscoreLabel(event.event_type)}</strong>
              <span>
                {event.from_status ? `${formatUnderscoreLabel(event.from_status)} -> ` : ""}
                {formatUnderscoreLabel(event.to_status)} · {formatDate(event.created_at)}
              </span>
              {event.reason ? <small>{event.reason}</small> : null}
            </li>
          ))}
        </ol>
      ) : null}
      {auditState === "success" && !auditEvents.length ? <EmptyState text="No audit events were returned for this application." /> : null}
    </div>
  );
}

function ContextRow({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.contextRow}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function GatedPanel({
  capability,
  icon,
  title,
}: {
  capability: PlatformCapability;
  icon: ReactNode;
  title: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.gatedPanel}`} role="status">
      <div className={styles.panelHeader}>
        {icon}
        <div>
          <h2>{title}</h2>
          <p>Missing platform permission: {missingPlatformPermissions(capability).join(" or ")}</p>
        </div>
      </div>
    </section>
  );
}

function PanelLoading({ title }: { title: string }) {
  return (
    <section className={styles.panel} role="status">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <div>
          <h2>{title}</h2>
          <p>RustLearn is resolving the latest platform data.</p>
        </div>
      </div>
      <div className={styles.skeletonGrid}>
        <span />
        <span />
        <span />
      </div>
    </section>
  );
}

function PanelError({
  error,
  onRetry,
  title,
}: {
  error: RouteError | null;
  onRetry: () => void;
  title: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.errorBox}`} role="alert">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <div>
          <h2>{title}</h2>
          <p>{error?.message || "The request failed before the API returned details."}</p>
        </div>
      </div>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={16} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function SignedOutState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <div>
          <h2>Sign in required</h2>
          <p>Platform admin data loads only after the current session is resolved.</p>
        </div>
      </div>
      <Link className={styles.secondaryLink} href="/login?redirect=/admin">
        Return to login
      </Link>
    </section>
  );
}

function LoadingState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <div>
          <h2>Resolving platform access</h2>
          <p>RustLearn is loading the current user and platform permissions.</p>
        </div>
      </div>
    </section>
  );
}

function SessionErrorState({ error }: { error: RouteError }) {
  return (
    <section className={`${styles.panel} ${styles.errorBox}`} role="alert">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <div>
          <h2>Admin session failed</h2>
          <p>{error.message}</p>
        </div>
      </div>
      <Link className={styles.secondaryLink} href="/login?redirect=/admin">
        Return to login
      </Link>
    </section>
  );
}

function AdminDeniedState({ workspace }: { workspace: PlatformAdminWorkspace }) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <ShieldAlert size={20} aria-hidden />
        <div>
          <h2>Platform admin access is not available</h2>
          <p>This session has {workspace.effectivePermissionCount} resolved platform permissions, but none unlock the platform admin workspace.</p>
        </div>
      </div>
      <Link className={styles.secondaryLink} href="/session">
        Review current session
      </Link>
    </section>
  );
}

function SummaryCard({ icon, label, value }: { icon: ReactNode; label: string; value: number }) {
  return (
    <article className={styles.summaryCard}>
      <span className={styles.smallIcon}>{icon}</span>
      <strong>{formatNumber(value)}</strong>
      <span>{label}</span>
    </article>
  );
}

function MetricCard({
  label,
  tone = "neutral",
  value,
}: {
  label: string;
  tone?: "good" | "neutral" | "warn";
  value: number | string;
}) {
  return (
    <article className={`${styles.metricCard} ${styles[tone]}`}>
      <strong>{typeof value === "number" ? formatNumber(value) : value}</strong>
      <span>{label}</span>
    </article>
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

function EmptyState({ text }: { text: string }) {
  return (
    <div className={styles.emptyState} role="status">
      <CheckCircle2 size={18} aria-hidden />
      <span>{text}</span>
    </div>
  );
}

function useAdminSession() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [token, setToken] = useState<string | null>(null);

  const loadSession = useCallback(async () => {
    const storedToken = readStoredSessionToken();
    if (!storedToken) {
      setError(null);
      setHasToken(false);
      setLoadState("idle");
      setSession(null);
      setToken(null);
      return;
    }

    setError(null);
    setHasToken(true);
    setLoadState("loading");
    setToken(storedToken);

    try {
      const nextSession = await fetchCurrentSession({ token: storedToken });
      setSession(nextSession);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError, "Platform admin session could not be loaded.");
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
        setToken(null);
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
    setToken(null);
  }

  return {
    error,
    hasToken,
    loadSession,
    loadState,
    session,
    signOut,
    token,
  };
}

function normalizeRouteError(error: unknown, fallbackMessage: string): RouteError {
  if (error instanceof SessionRequestError || error instanceof AdminRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "network_error",
    message: fallbackMessage,
    status: 0,
  };
}

function getCapability(workspace: PlatformAdminWorkspace, key: PlatformCapabilityKey): PlatformCapability {
  const capability = workspace.capabilities.find((item) => item.key === key);
  if (capability) {
    return capability;
  }

  const definition = platformCapabilityDefinitions.find((item) => item.key === key);
  return {
    enabled: false,
    key,
    label: definition?.label || key,
    permissions: definition?.permissions || [],
  };
}

function capabilityLabel(workspace: PlatformAdminWorkspace, key: PlatformCapabilityKey) {
  return platformCapabilityEnabled(workspace, key) ? "Available" : `Missing ${getCapability(workspace, key).permissions[0]}`;
}

function hasPlatformPermission(workspace: PlatformAdminWorkspace, permission: string) {
  return workspace.effectivePermissions.includes(permission);
}

function hasAnyPlatformPermission(workspace: PlatformAdminWorkspace, permissions: string[]) {
  return permissions.some((permission) => hasPlatformPermission(workspace, permission));
}

function startCsvDownload(csv: PlatformCsvDownload) {
  const blob = new Blob([csv.body], { type: "text/csv;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = csv.filename;
  document.body.append(anchor);
  anchor.click();
  anchor.remove();
  URL.revokeObjectURL(url);
}

function formatNumber(value: number) {
  return new Intl.NumberFormat().format(value);
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

function formatUnderscoreLabel(value: string) {
  return value
    .split("_")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

function dependencyLabel(name: string) {
  if (name === "postgres") {
    return "PostgreSQL";
  }
  if (name === "s3") {
    return "S3 storage";
  }
  if (name === "ethereum") {
    return "Ethereum";
  }
  return formatUnderscoreLabel(name);
}

function scopeTargetLabel(application: PlatformTeacherApplicationItem) {
  if (application.requested_course) {
    return application.requested_course.title;
  }
  if (application.requested_organization) {
    return application.requested_organization.name;
  }
  if (application.sponsor_organization) {
    return application.sponsor_organization.name;
  }
  return "Platform scope";
}

function statusTone(status: TeacherApplicationStatus): "good" | "neutral" | "warn" {
  if (status === "approved") {
    return "good";
  }
  if (status === "submitted" || status === "needs_changes") {
    return "warn";
  }
  return "neutral";
}

function isFinalApplicationStatus(status: TeacherApplicationStatus) {
  return status === "approved" || status === "rejected";
}

function candidateStatusTone(status: string): "good" | "neutral" | "warn" {
  if (status === "amount_approved" || status === "wallet_credited" || status === "token_confirmed" || status === "completed") {
    return "good";
  }
  if (status === "teacher_approved" || status === "pending_teacher_approval" || status === "token_pending" || status === "needs_reconciliation") {
    return "warn";
  }
  if (status === "amount_rejected" || status === "teacher_rejected" || status === "failed") {
    return "warn";
  }
  return "neutral";
}

function RewardCandidateDetail({
  candidate,
  auditError,
  auditEvents,
  auditState,
  canApprove,
  decisionAmount,
  decisionError,
  decisionReason,
  decisionState,
  decisionStatus,
  onDecisionAmountChange,
  onDecisionReasonChange,
  onDecisionStatusChange,
  onRefreshAudit,
  onSubmitDecision,
}: {
  candidate: PlatformRewardCandidateItem | null;
  auditError: RouteError | null;
  auditEvents: RewardAuditEvent[];
  auditState: SectionState;
  canApprove: boolean;
  decisionAmount: string;
  decisionError: RouteError | null;
  decisionReason: string;
  decisionState: "idle" | "submitting" | "success" | "error";
  decisionStatus: "approved" | "rejected";
  onDecisionAmountChange: (value: string) => void;
  onDecisionReasonChange: (value: string) => void;
  onDecisionStatusChange: (value: "approved" | "rejected") => void;
  onRefreshAudit: () => void;
  onSubmitDecision: (event: FormEvent<HTMLFormElement>) => void;
}) {
  if (!candidate) {
    return (
      <section className={styles.panel}>
        <div className={styles.panelHeader}>
          <Landmark size={20} aria-hidden />
          <div>
            <h2>Candidate detail</h2>
            <p>Select a candidate from the queue to inspect context and audit history.</p>
          </div>
        </div>
      </section>
    );
  }

  const isFinal = candidate.status !== "teacher_approved" && candidate.status !== "pending_teacher_approval";
  const canSubmitDecision =
    !isFinal &&
    decisionReason.trim().length > 0 &&
    (decisionStatus === "rejected" || (decisionStatus === "approved" && canApprove && decisionAmount.trim().length > 0 && !Number.isNaN(Number(decisionAmount)) && Number(decisionAmount) >= 0)) &&
    decisionState !== "submitting";

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Landmark size={20} aria-hidden />
        <div>
          <h2>Candidate {candidate.id}</h2>
          <p>{candidate.student.name} · {candidate.student.email}</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(candidate.status)} tone={candidateStatusTone(candidate.status)} />
      </div>

      <div className={styles.detailGrid}>
        <ContextRow label="Student" value={candidate.student.name} />
        <ContextRow label="Course" value={candidate.course.title} />
        <ContextRow label="Event type" value={formatUnderscoreLabel(candidate.event_type)} />
        <ContextRow label="Submitter" value={candidate.submitter.name} />
        <ContextRow label="Teacher approver" value={candidate.teacher_approver?.name || "No approver yet"} />
        <ContextRow label="Teacher decision" value={candidate.teacher_decision_reason || "No reason recorded"} />
        <ContextRow label="Approved amount" value={candidate.approved_amount ?? "Not set"} />
        <ContextRow label="Created" value={formatDate(candidate.created_at)} />
        <ContextRow label="Updated" value={formatDate(candidate.updated_at)} />
      </div>

      <div className={styles.textBlock}>
        <h3>Audit history</h3>
        <button className={styles.secondaryButton} disabled={auditState === "loading"} onClick={onRefreshAudit} type="button">
          {auditState === "loading" ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <RefreshCw size={15} aria-hidden />}
          Refresh
        </button>
        {auditState === "loading" || auditState === "idle" ? (
          <p className={styles.muted}>Loading audit events.</p>
        ) : null}
        {auditState === "error" ? (
          <div className={styles.inlineError} role="alert">
            <AlertTriangle size={16} aria-hidden />
            <span>{auditError?.message || "Audit history could not be loaded."}</span>
          </div>
        ) : null}
        {auditState === "success" && auditEvents.length ? (
          <ol className={styles.auditList}>
            {auditEvents.map((event) => (
              <li key={event.id}>
                <strong>{formatUnderscoreLabel(event.event_type)}</strong>
                <span>
                  {event.from_status ? `${formatUnderscoreLabel(event.from_status)} -> ` : ""}
                  {formatUnderscoreLabel(event.to_status)} · {formatDate(event.created_at)}
                </span>
                {event.reason ? <small>{event.reason}</small> : null}
              </li>
            ))}
          </ol>
        ) : null}
        {auditState === "success" && !auditEvents.length ? <EmptyState text="No audit events were returned for this candidate." /> : null}
      </div>

      <form className={styles.decisionForm} onSubmit={onSubmitDecision}>
        <div className={styles.subsectionHeader}>
          <h3>Amount decision</h3>
          {isFinal ? <StatusPill label="Final state" tone="neutral" /> : null}
        </div>
        <label>
          <span>Decision status</span>
          <select
            disabled={isFinal}
            onChange={(event) => onDecisionStatusChange(event.target.value as "approved" | "rejected")}
            value={decisionStatus}
          >
            <option disabled={!canApprove} value="approved">Approve amount</option>
            <option value="rejected">Reject</option>
          </select>
        </label>
        {!canApprove ? (
          <p className={styles.muted}>Approve amount requires the APPROVE_REWARD_AMOUNT platform permission.</p>
        ) : null}
        {decisionStatus === "approved" ? (
          <label>
            <span>Approved amount</span>
            <input
              disabled={isFinal}
              onChange={(event) => onDecisionAmountChange(event.target.value)}
              placeholder="0.00"
              type="text"
              value={decisionAmount}
            />
          </label>
        ) : null}
        <label>
          <span>Decision reason</span>
          <textarea
            disabled={isFinal}
            onChange={(event) => onDecisionReasonChange(event.target.value)}
            placeholder="Explain the amount decision or rejection reason."
            rows={4}
            value={decisionReason}
          />
        </label>
        {!isFinal && !decisionReason.trim() ? (
          <p className={styles.muted}>A decision reason is required before saving.</p>
        ) : null}
        {decisionError ? (
          <div className={styles.inlineError} role="alert">
            <AlertTriangle size={16} aria-hidden />
            <span>{decisionError.message}</span>
          </div>
        ) : null}
        <button className={styles.primaryButton} disabled={!canSubmitDecision} type="submit">
          {decisionState === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
          Save decision
        </button>
        {isFinal ? <p className={styles.muted}>This candidate is in a final state and cannot be changed from this screen.</p> : null}
      </form>
    </section>
  );
}
