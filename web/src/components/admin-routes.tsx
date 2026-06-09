"use client";

import {
  AlertTriangle,
  CheckCircle2,
  Database,
  Download,
  FileSpreadsheet,
  FileText,
  Gauge,
  Landmark,
  Loader2,
  RefreshCw,
  ShieldAlert,
  ShieldCheck,
  UserCheck,
  Users,
  WalletCards,
  XCircle,
  type LucideIcon,
} from "lucide-react";
import Link from "next/link";
import { type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import {
  AdminRequestError,
  buildPlatformAdminWorkspace,
  downloadPlatformCsv,
  fetchPlatformFraudDashboard,
  fetchPlatformRewardDashboard,
  fetchPlatformSummary,
  fetchPlatformSystemStatus,
  missingPlatformPermissions,
  platformCapabilityDefinitions,
  platformCapabilityEnabled,
  type PlatformAdminWorkspace,
  type PlatformCapability,
  type PlatformCapabilityKey,
  type PlatformCsvDownload,
  type PlatformCsvReport,
  type PlatformFraudDashboard,
  type PlatformReportSummary,
  type PlatformRewardDashboard,
  type PlatformSystemStatus,
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

function targetLabel(block: PlatformFraudDashboard["active_blocks"][number]) {
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
