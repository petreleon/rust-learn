"use client";

import {
  Ban,
  CheckCircle2,
  ClipboardList,
  Download,
  Eye,
  EyeOff,
  FileCheck,
  GraduationCap,
  History,
  KeyRound,
  RefreshCw,
  Send,
  ShieldAlert,
  WalletCards,
} from "lucide-react";
import { type FormEvent, useEffect, useMemo, useRef, useState } from "react";
import styles from "./page.module.css";

type ApiState = "checking" | "online" | "offline";
type HttpMethod = "GET" | "POST" | "PUT";

type ApiResult = {
  label: string;
  status: string;
  body: string;
  ok: boolean;
};

type PermissionOption = {
  key: string;
  label: string;
  scope: "platform" | "organization" | "course";
};

const HEALTH_CHECK_TIMEOUT_MS = 5000;
const API_REQUEST_TIMEOUT_MS = 10000;
const API_REQUEST_TIMEOUT_SECONDS = API_REQUEST_TIMEOUT_MS / 1000;

const PERMISSIONS: PermissionOption[] = [
  { key: "SUBMIT_TEACHER_APPLICATION", label: "Submit application", scope: "platform" },
  { key: "REVIEW_TEACHER_APPLICATIONS", label: "Review applications", scope: "platform" },
  { key: "APPROVE_TEACHER_APPLICATION", label: "Approve teachers", scope: "platform" },
  { key: "REJECT_TEACHER_APPLICATION", label: "Reject teachers", scope: "platform" },
  { key: "SUBMIT_COURSE_REWARD_EVENT", label: "Submit course reward", scope: "course" },
  { key: "VIEW_COURSE_REWARD_STATUS", label: "View course rewards", scope: "course" },
  {
    key: "APPROVE_STUDENT_REWARD_CANDIDATE",
    label: "Approve student reward",
    scope: "course",
  },
  { key: "APPROVE_REWARD_AMOUNT", label: "Approve amount", scope: "platform" },
  { key: "VIEW_REPORT", label: "View summary reports", scope: "platform" },
  { key: "GENERATE_REPORT", label: "Generate summary CSV", scope: "organization" },
  { key: "VIEW_ORG_REWARD_REPORTS", label: "View org rewards", scope: "organization" },
  { key: "VIEW_REWARD_AUDIT", label: "View reward audit", scope: "platform" },
  { key: "MANAGE_REWARD_FRAUD_BLOCKS", label: "Manage fraud blocks", scope: "platform" },
  { key: "BLOCK_REWARD_TEACHER", label: "Block teacher rewards", scope: "platform" },
  { key: "BLOCK_REWARD_ORGANIZATION", label: "Block org rewards", scope: "platform" },
  { key: "DELEGATE_REWARD_APPROVAL", label: "Delegate reward approval", scope: "platform" },
  { key: "EXPORT_DATA", label: "Export platform data", scope: "platform" },
];

const DEFAULT_PERMISSION_KEYS = [
  "SUBMIT_TEACHER_APPLICATION",
  "REVIEW_TEACHER_APPLICATIONS",
  "APPROVE_TEACHER_APPLICATION",
  "REJECT_TEACHER_APPLICATION",
  "SUBMIT_COURSE_REWARD_EVENT",
  "VIEW_COURSE_REWARD_STATUS",
  "APPROVE_STUDENT_REWARD_CANDIDATE",
  "APPROVE_REWARD_AMOUNT",
  "VIEW_REPORT",
  "GENERATE_REPORT",
  "VIEW_ORG_REWARD_REPORTS",
  "VIEW_REWARD_AUDIT",
  "MANAGE_REWARD_FRAUD_BLOCKS",
  "DELEGATE_REWARD_APPROVAL",
  "EXPORT_DATA",
];

const rewardStatuses = [
  "pending_teacher_approval",
  "teacher_approved",
  "teacher_rejected",
  "amount_approved",
  "amount_rejected",
  "adjusted",
  "token_pending",
  "token_confirmed",
  "wallet_credited",
  "notified",
  "completed",
  "needs_reconciliation",
  "failed",
];

const teacherApplicationStatuses = ["submitted", "needs_changes", "approved", "rejected"];
const DEFAULT_TEACHER_APPLICATION_SCOPE = "platform";
const DEFAULT_TEACHER_DECISION_STATUS = "approved";
const DEFAULT_REWARD_STATUS = "pending_teacher_approval";
const DEFAULT_TEACHER_REWARD_DECISION_STATUS = "approved";
const DEFAULT_AMOUNT_DECISION_STATUS = "approved";
const DEFAULT_AMOUNT_VALUE = "10";
const DEFAULT_FRAUD_BLOCK_SCOPE = "teacher";
const DEFAULT_DELEGATED_PERMISSION = "APPROVE_REWARD_AMOUNT";
const DEFAULT_DELEGATION_SCOPE = "platform";

const delegatedPermissionOptions: Array<{ key: string; scopes: string[] }> = [
  { key: "APPROVE_REWARD_AMOUNT", scopes: ["platform"] },
  { key: "EXECUTE_REWARD_PAYOUT", scopes: ["platform"] },
  { key: "VIEW_REWARD_AUDIT", scopes: ["platform"] },
  { key: "MANAGE_REWARD_FRAUD_BLOCKS", scopes: ["platform"] },
  { key: "BLOCK_REWARD_TEACHER", scopes: ["platform"] },
  { key: "BLOCK_REWARD_ORGANIZATION", scopes: ["platform"] },
  { key: "SUBMIT_ORG_COURSE_REWARD_EVENT", scopes: ["organization"] },
  { key: "VIEW_ORG_REWARD_REPORTS", scopes: ["organization"] },
  { key: "MANAGE_ORG_REWARD_BUDGET", scopes: ["organization"] },
  { key: "SUBMIT_COURSE_REWARD_EVENT", scopes: ["course"] },
  { key: "CREATE_REWARDABLE_COURSE_EVENT", scopes: ["course"] },
  { key: "APPROVE_STUDENT_REWARD_CANDIDATE", scopes: ["course"] },
  { key: "VIEW_COURSE_REWARD_STATUS", scopes: ["course"] },
  { key: "GRADE_REWARDABLE_ASSESSMENT", scopes: ["course"] },
  { key: "MANAGE_COURSE_REWARD_RULES", scopes: ["course"] },
];

const fraudBlockScopes = ["teacher", "organization", "course", "reward_policy"];

const platformExportReports = [
  {
    path: "/reports/platform/summary.csv",
    label: "Platform summary CSV",
    resultLabel: "Platform summary CSV",
  },
  {
    path: "/reports/platform/reward-dashboard.csv",
    label: "Reward dashboard CSV",
    resultLabel: "Platform reward dashboard CSV",
  },
  {
    path: "/reports/platform/fraud-dashboard.csv",
    label: "Fraud dashboard CSV",
    resultLabel: "Platform fraud dashboard CSV",
  },
  {
    path: "/reports/platform/teacher-applications.csv",
    label: "Teacher applications CSV",
    resultLabel: "Platform teacher applications CSV",
  },
  {
    path: "/reports/platform/reward-approvals.csv",
    label: "Reward approvals CSV",
    resultLabel: "Platform reward approvals CSV",
  },
  {
    path: "/reports/platform/token-payouts.csv",
    label: "Token payouts CSV",
    resultLabel: "Platform token payouts CSV",
  },
  {
    path: "/reports/platform/wallet-credits.csv",
    label: "Wallet credits CSV",
    resultLabel: "Platform wallet credits CSV",
  },
  {
    path: "/reports/platform/delegated-permissions.csv",
    label: "Delegations CSV",
    resultLabel: "Platform delegated permissions CSV",
  },
];

const workflowNavItems = [
  { href: "#teacher-workflow", label: "Applications" },
  { href: "#reward-workflow", label: "Rewards" },
  { href: "#history-workflow", label: "History" },
  { href: "#report-workflow", label: "Reports" },
  { href: "#fraud-workflow", label: "Fraud" },
  { href: "#delegation-workflow", label: "Delegation" },
  { href: "#result-panel", label: "Result" },
];

const PROTECTED_ACTIONS = {
  submitTeacherApplication: "Submit teacher application",
  teacherApplicationQueue: "Teacher application queue",
  teacherApplicationDecision: "Teacher application decision",
  submitRewardCandidate: "Submit reward candidate",
  courseRewardCandidates: "Course reward candidates",
  courseRewardDecision: "Course reward decision",
  rewardAmountDecision: "Reward amount decision",
  studentRewardHistory: "Student reward history",
  organizationSummary: "Organization summary",
  organizationSummaryCsv: "Organization summary CSV",
  organizationRewardReport: "Organization reward report",
  organizationRewardCsv: "Organization reward CSV",
  platformSummary: "Platform summary",
  platformRewardDashboard: "Platform reward dashboard",
  platformFraudDashboard: "Platform fraud dashboard",
  createRewardFraudBlock: "Create reward fraud block",
  rewardFraudBlocks: "Reward fraud blocks",
  rewardFraudAudit: "Reward fraud audit",
  revokeRewardFraudBlock: "Revoke reward fraud block",
  grantDelegatedPermission: "Grant delegated permission",
  delegatedPermissions: "Delegated permissions",
  revokeDelegatedPermission: "Revoke delegated permission",
} as const;

const positiveIntegerInputProps = {
  inputMode: "numeric" as const,
  pattern: "[0-9]*",
};

const decimalInputProps = {
  inputMode: "decimal" as const,
};

function normalizeRoot(root: string) {
  const trimmed = root.trim();
  return trimmed.endsWith("/") ? trimmed.slice(0, -1) : trimmed;
}

function optionalNumber(value: string) {
  const trimmed = value.trim();
  if (!trimmed) {
    return undefined;
  }
  const parsed = Number(trimmed);
  return Number.isFinite(parsed) ? parsed : undefined;
}

function hasText(value: string) {
  return value.trim().length > 0;
}

function hasAnyText(values: string[]) {
  return values.some(hasText);
}

function hasPositiveInteger(value: string) {
  return /^[1-9]\d*$/.test(value.trim());
}

function optionalPositiveInteger(value: string) {
  const trimmed = value.trim();
  return hasPositiveInteger(trimmed) ? Number(trimmed) : undefined;
}

function optionalUtcDateTime(value: string) {
  const trimmed = value.trim();
  if (!trimmed) {
    return undefined;
  }

  const parsed = new Date(trimmed);
  return Number.isNaN(parsed.getTime()) ? undefined : parsed.toISOString();
}

function hasNonNegativeNumber(value: string) {
  const trimmed = value.trim();
  if (!trimmed) {
    return false;
  }

  const parsed = Number(trimmed);
  return Number.isFinite(parsed) && parsed >= 0;
}

function splitLinks(value: string) {
  const links = value
    .split(/[\n,]/)
    .map((link) => link.trim())
    .filter(Boolean);
  return links.length > 0 ? links : undefined;
}

function buildQuery(params: Record<string, string | boolean | number | undefined>) {
  const query = new URLSearchParams();
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== "") {
      query.set(key, String(value));
    }
  });
  const serialized = query.toString();
  return serialized ? `?${serialized}` : "";
}

function prettyBody(value: string) {
  if (!value) {
    return "";
  }
  try {
    return JSON.stringify(JSON.parse(value), null, 2);
  } catch {
    return value;
  }
}

function formatHealthMessage(value: string) {
  const trimmed = value.trim();
  if (!trimmed) {
    return "API responded";
  }

  try {
    const parsed: unknown = JSON.parse(trimmed);
    if (parsed && typeof parsed === "object" && "status" in parsed) {
      const status = String((parsed as { status?: unknown }).status ?? "").trim();
      if (status.toLowerCase() === "ok") {
        return "API healthy";
      }
      if (status) {
        return `API status: ${status}`;
      }
    }
  } catch {
    return trimmed;
  }

  return trimmed;
}

async function fetchWithTimeout(input: string, init: RequestInit = {}) {
  const controller = new AbortController();
  const timeoutId = window.setTimeout(() => controller.abort(), API_REQUEST_TIMEOUT_MS);

  try {
    return await fetch(input, { ...init, signal: controller.signal });
  } finally {
    window.clearTimeout(timeoutId);
  }
}

function requestFailureMessage(error: unknown) {
  if (error instanceof Error && error.name === "AbortError") {
    return `Request timed out after ${API_REQUEST_TIMEOUT_SECONDS} seconds.`;
  }

  return error instanceof Error ? error.message : "Unknown request failure";
}

function statusLabel(status: ApiState) {
  if (status === "online") {
    return "Online";
  }
  if (status === "offline") {
    return "Offline";
  }
  return "Checking";
}

function optionLabel(value: string) {
  return value
    .split("_")
    .filter(Boolean)
    .map((word) => `${word.charAt(0).toUpperCase()}${word.slice(1).toLowerCase()}`)
    .join(" ");
}

function flowStatus(hasAccess: boolean, hasSessionToken: boolean, hasServerDenial = false) {
  if (!hasAccess) {
    return "Locked";
  }
  if (!hasSessionToken) {
    return "Needs JWT";
  }
  return hasServerDenial ? "Limited" : "Ready";
}

function missingFields(fields: Array<[label: string, complete: boolean]>) {
  return fields.filter(([, complete]) => !complete).map(([label]) => label);
}

function formatFieldList(fields: string[]) {
  if (fields.length === 1) {
    return `${fields[0]} is required.`;
  }

  return `${fields.slice(0, -1).join(", ")} and ${fields[fields.length - 1]} are required.`;
}

function PermissionNotice({ title, detail }: { title: string; detail: string }) {
  return (
    <div className={styles.panelNotice} role="status" aria-label={`${title}: ${detail}`}>
      <ShieldAlert size={16} aria-hidden />
      <div>
        <strong>{title}</strong>
        <span>{detail}</span>
      </div>
    </div>
  );
}

function RequirementNotice({ action, fields }: { action: string; fields: string[] }) {
  if (fields.length === 0) {
    return null;
  }
  const message = formatFieldList(fields);

  return (
    <div className={styles.requirementNotice} role="status" aria-label={`${action}: ${message}`}>
      <FileCheck size={16} aria-hidden />
      <div>
        <strong>{action}</strong>
        <span>{message}</span>
      </div>
    </div>
  );
}

function ServerDeniedNotice({ action }: { action: string }) {
  return (
    <div
      className={`${styles.requirementNotice} ${styles.deniedNotice}`}
      role="status"
      aria-label={`${action}: server denied`}
    >
      <ShieldAlert size={16} aria-hidden />
      <div>
        <strong>Server denied</strong>
        <span>{action} is locked for this JWT.</span>
      </div>
    </div>
  );
}

export default function Home() {
  const resultPanelRef = useRef<HTMLElement | null>(null);
  const [apiRoot, setApiRoot] = useState(process.env.NEXT_PUBLIC_API_URL || "/api");
  const [token, setToken] = useState("");
  const [showToken, setShowToken] = useState(false);
  const [credentials, setCredentials] = useState({ email: "", password: "" });
  const [sessionMessage, setSessionMessage] = useState("Not signed in");
  const [healthCheckTick, setHealthCheckTick] = useState(0);
  const [apiState, setApiState] = useState<ApiState>("checking");
  const [apiMessage, setApiMessage] = useState("Checking API");
  const [selectedPermissions, setSelectedPermissions] = useState<Set<string>>(
    () => new Set(DEFAULT_PERMISSION_KEYS)
  );
  const [serverDeniedActions, setServerDeniedActions] = useState<Set<string>>(() => new Set());
  const pendingActionRef = useRef<string | null>(null);
  const [pendingAction, setPendingAction] = useState<string | null>(null);
  const [result, setResult] = useState<ApiResult>({
    label: "Result",
    status: "Idle",
    body: "No request sent.",
    ok: true,
  });

  const [teacherForm, setTeacherForm] = useState({
    requested_scope: DEFAULT_TEACHER_APPLICATION_SCOPE,
    requested_organization_id: "",
    requested_course_id: "",
    experience_summary: "",
    organization_sponsor_id: "",
    portfolio_links: "",
  });
  const [teacherStatus, setTeacherStatus] = useState("submitted");
  const [teacherDecision, setTeacherDecision] = useState({
    application_id: "",
    status: DEFAULT_TEACHER_DECISION_STATUS,
    decision_reason: "",
  });

  const [rewardCourseId, setRewardCourseId] = useState("");
  const [rewardCandidateId, setRewardCandidateId] = useState("");
  const [rewardStudentId, setRewardStudentId] = useState("");
  const [rewardStatus, setRewardStatus] = useState(DEFAULT_REWARD_STATUS);
  const [teacherRewardDecision, setTeacherRewardDecision] = useState({
    status: DEFAULT_TEACHER_REWARD_DECISION_STATUS,
    decision_reason: "",
  });
  const [amountDecision, setAmountDecision] = useState({
    status: DEFAULT_AMOUNT_DECISION_STATUS,
    approved_amount: DEFAULT_AMOUNT_VALUE,
    decision_reason: "",
  });

  const [historyStatus, setHistoryStatus] = useState("");
  const [organizationId, setOrganizationId] = useState("");

  const [fraudBlock, setFraudBlock] = useState({
    scope_type: DEFAULT_FRAUD_BLOCK_SCOPE,
    teacher_user_id: "",
    organization_id: "",
    course_id: "",
    reward_policy_id: "",
    reason: "",
    evidence_reference: "",
  });
  const [fraudBlockId, setFraudBlockId] = useState("");

  const [delegation, setDelegation] = useState({
    grantee_user_id: "",
    permission: DEFAULT_DELEGATED_PERMISSION,
    scope_type: DEFAULT_DELEGATION_SCOPE,
    organization_id: "",
    course_id: "",
    reason: "",
    expires_at: "",
  });
  const [delegationId, setDelegationId] = useState("");
  const [revokeReason, setRevokeReason] = useState("");

  useEffect(() => {
    const controller = new AbortController();
    const root = normalizeRoot(apiRoot);
    const healthRoot = root.endsWith("/api") ? root.slice(0, -4) : root;
    let timedOut = false;
    const timeoutId = window.setTimeout(() => {
      timedOut = true;
      controller.abort();
    }, HEALTH_CHECK_TIMEOUT_MS);

    fetch(`${healthRoot || ""}/health`, { signal: controller.signal })
      .then(async (response) => {
        const body = await response.text();
        window.clearTimeout(timeoutId);
        if (!response.ok) {
          throw new Error(`HTTP ${response.status}`);
        }
        setApiState("online");
        setApiMessage(formatHealthMessage(body));
      })
      .catch((error: Error) => {
        window.clearTimeout(timeoutId);
        if (controller.signal.aborted && !timedOut) {
          return;
        }
        setApiState("offline");
        setApiMessage(
          timedOut ? "API health check timed out" : error.message || "Connection failed"
        );
      });

    return () => {
      window.clearTimeout(timeoutId);
      controller.abort();
    };
  }, [apiRoot, healthCheckTick]);

  const permissionGroups = useMemo(
    () =>
      PERMISSIONS.reduce<Record<PermissionOption["scope"], PermissionOption[]>>(
        (groups, permission) => {
          groups[permission.scope].push(permission);
          return groups;
        },
        { platform: [], organization: [], course: [] }
      ),
    []
  );

  const hasPermission = (permission: string) => selectedPermissions.has(permission);
  const hasSessionToken = hasText(token);
  const isServerDenied = (action: string) => serverDeniedActions.has(action);
  const hasCompleteCredentials = hasText(credentials.email) && hasText(credentials.password);
  const hasPendingAction = pendingAction !== null;
  const pendingActionTitle = pendingAction ? `${pendingAction} request in progress` : undefined;
  const canSignIn = hasCompleteCredentials && !hasPendingAction;
  const hasSessionDraft =
    hasSessionToken || hasText(credentials.email) || hasText(credentials.password);
  const resetServerDenials = () => setServerDeniedActions(new Set());
  const setServerDeniedAction = (action: string, denied: boolean) => {
    setServerDeniedActions((current) => {
      const isAlreadyDenied = current.has(action);
      if (isAlreadyDenied === denied) {
        return current;
      }

      const next = new Set(current);
      if (denied) {
        next.add(action);
      } else {
        next.delete(action);
      }
      return next;
    });
  };
  const beginPendingAction = (action: string) => {
    if (pendingActionRef.current) {
      return false;
    }

    pendingActionRef.current = action;
    setPendingAction(action);
    return true;
  };
  const finishPendingAction = (action: string) => {
    if (pendingActionRef.current !== action) {
      return;
    }

    pendingActionRef.current = null;
    setPendingAction(null);
  };
  const actionState = (
    ready = true,
    allowed = true,
    permissionLabel = "Required permission",
    serverAction?: string
  ) => {
    if (serverAction && isServerDenied(serverAction)) {
      return { disabled: true, title: "Server denied this JWT" };
    }
    if (!allowed) {
      return { disabled: true, title: permissionLabel };
    }
    if (!hasSessionToken) {
      return { disabled: true, title: "JWT required" };
    }
    if (!ready) {
      return { disabled: true, title: "Complete required fields" };
    }
    if (hasPendingAction) {
      return { disabled: true, title: pendingActionTitle };
    }
    return { disabled: false, title: undefined };
  };
  const serverDeniedNotice = (serverAction: string, action: string, key = serverAction) =>
    hasSessionToken && isServerDenied(serverAction) ? (
      <ServerDeniedNotice key={key} action={action} />
    ) : null;
  const canTeacherApply = hasPermission("SUBMIT_TEACHER_APPLICATION");
  const canReviewTeachers = hasPermission("REVIEW_TEACHER_APPLICATIONS");
  const canApproveTeachers = hasPermission("APPROVE_TEACHER_APPLICATION");
  const canRejectTeachers = hasPermission("REJECT_TEACHER_APPLICATION");
  const teacherDecisionStatuses = [
    ...(canApproveTeachers ? ["approved"] : []),
    ...(canReviewTeachers ? ["needs_changes"] : []),
    ...(canRejectTeachers ? ["rejected"] : []),
  ];
  const activeTeacherDecisionStatus = teacherDecisionStatuses.includes(teacherDecision.status)
    ? teacherDecision.status
    : teacherDecisionStatuses[0] || teacherDecision.status;
  const canDecideTeachers = teacherDecisionStatuses.length > 0;
  const canSubmitReward = hasPermission("SUBMIT_COURSE_REWARD_EVENT");
  const canViewCourseRewards = hasPermission("VIEW_COURSE_REWARD_STATUS");
  const canTeacherApproveReward = hasPermission("APPROVE_STUDENT_REWARD_CANDIDATE");
  const canApproveAmount = hasPermission("APPROVE_REWARD_AMOUNT");
  const canViewOrgReports = hasPermission("VIEW_ORG_REWARD_REPORTS");
  const canViewSummaryReports = hasPermission("VIEW_REPORT");
  const canGenerateReports = hasPermission("GENERATE_REPORT");
  const canManageFraudBlocks = hasPermission("MANAGE_REWARD_FRAUD_BLOCKS");
  const canBlockTeacherRewards = hasPermission("BLOCK_REWARD_TEACHER") || canManageFraudBlocks;
  const canBlockOrganizationRewards =
    hasPermission("BLOCK_REWARD_ORGANIZATION") || canManageFraudBlocks;
  const canViewFraud = hasPermission("VIEW_REWARD_AUDIT") || canManageFraudBlocks;
  const canViewPlatformDashboards = hasPermission("VIEW_REWARD_AUDIT");
  const canManageFraud =
    canManageFraudBlocks || canBlockTeacherRewards || canBlockOrganizationRewards;
  const canDelegate = hasPermission("DELEGATE_REWARD_APPROVAL");
  const canExport = hasPermission("EXPORT_DATA");
  const canUseTeacherWorkflow = canTeacherApply || canReviewTeachers || canDecideTeachers;
  const canUseRewardWorkflow =
    canSubmitReward || canViewCourseRewards || canTeacherApproveReward || canApproveAmount;
  const showRewardCourseId = canSubmitReward || canViewCourseRewards || canTeacherApproveReward;
  const showRewardCandidateId = canTeacherApproveReward || canApproveAmount;
  const showRewardStatusFilter = canViewCourseRewards;
  const showRewardSharedFields =
    showRewardCourseId || showRewardCandidateId || showRewardStatusFilter;
  const canUseOrganizationReportControls =
    canViewSummaryReports || canGenerateReports || canViewOrgReports;
  const canUseReportWorkflow =
    canUseOrganizationReportControls || canViewPlatformDashboards || canExport;
  const canUseFraudWorkflow = canViewFraud || canManageFraud;
  const canUseAuditWorkflow = canUseReportWorkflow || canUseFraudWorkflow || canDelegate;
  const visibleTeacherWorkflowActions = [
    ...(canTeacherApply ? [PROTECTED_ACTIONS.submitTeacherApplication] : []),
    ...(canReviewTeachers ? [PROTECTED_ACTIONS.teacherApplicationQueue] : []),
    ...(canDecideTeachers ? [PROTECTED_ACTIONS.teacherApplicationDecision] : []),
  ];
  const visibleRewardWorkflowActions = [
    ...(canSubmitReward ? [PROTECTED_ACTIONS.submitRewardCandidate] : []),
    ...(canViewCourseRewards
      ? [PROTECTED_ACTIONS.courseRewardCandidates, PROTECTED_ACTIONS.studentRewardHistory]
      : []),
    ...(canTeacherApproveReward ? [PROTECTED_ACTIONS.courseRewardDecision] : []),
    ...(canApproveAmount ? [PROTECTED_ACTIONS.rewardAmountDecision] : []),
  ];
  const visibleAuditWorkflowActions = [
    ...(canViewSummaryReports
      ? [PROTECTED_ACTIONS.organizationSummary, PROTECTED_ACTIONS.platformSummary]
      : []),
    ...(canGenerateReports ? [PROTECTED_ACTIONS.organizationSummaryCsv] : []),
    ...(canViewOrgReports
      ? [PROTECTED_ACTIONS.organizationRewardReport, PROTECTED_ACTIONS.organizationRewardCsv]
      : []),
    ...(canExport ? platformExportReports.map((report) => report.resultLabel) : []),
    ...(canViewPlatformDashboards
      ? [PROTECTED_ACTIONS.platformRewardDashboard, PROTECTED_ACTIONS.platformFraudDashboard]
      : []),
    ...(canManageFraud
      ? [PROTECTED_ACTIONS.createRewardFraudBlock, PROTECTED_ACTIONS.revokeRewardFraudBlock]
      : []),
    ...(canViewFraud
      ? [PROTECTED_ACTIONS.rewardFraudBlocks, PROTECTED_ACTIONS.rewardFraudAudit]
      : []),
    ...(canDelegate
      ? [
          PROTECTED_ACTIONS.grantDelegatedPermission,
          PROTECTED_ACTIONS.delegatedPermissions,
          PROTECTED_ACTIONS.revokeDelegatedPermission,
        ]
      : []),
  ];
  const teacherWorkflowServerDenied = visibleTeacherWorkflowActions.some(isServerDenied);
  const rewardWorkflowServerDenied = visibleRewardWorkflowActions.some(isServerDenied);
  const auditWorkflowServerDenied = visibleAuditWorkflowActions.some(isServerDenied);
  const canSubmitTeacherApplicationForm =
    hasText(teacherForm.experience_summary) &&
    (teacherForm.requested_scope === "platform" ||
      (teacherForm.requested_scope === "organization" &&
        (hasPositiveInteger(teacherForm.requested_organization_id) ||
          hasPositiveInteger(teacherForm.organization_sponsor_id))) ||
      (teacherForm.requested_scope === "course" &&
        hasPositiveInteger(teacherForm.requested_course_id)));
  const canDecideTeacherApplicationForm =
    canDecideTeachers && hasPositiveInteger(teacherDecision.application_id);
  const canSubmitRewardCandidateForm =
    hasPositiveInteger(rewardCourseId) && hasPositiveInteger(rewardStudentId);
  const canLoadRewardCandidatesForm = hasPositiveInteger(rewardCourseId);
  const canDecideStudentRewardForm =
    hasPositiveInteger(rewardCourseId) && hasPositiveInteger(rewardCandidateId);
  const canDecideRewardAmountForm =
    hasPositiveInteger(rewardCandidateId) &&
    (amountDecision.status !== "approved" || hasNonNegativeNumber(amountDecision.approved_amount));
  const canLoadOrganizationReportForm = hasPositiveInteger(organizationId);
  const fraudBlockScopePermissions: Record<
    string,
    { allowed: boolean; detail: string; permissionTitle: string }
  > = {
    teacher: {
      allowed: canBlockTeacherRewards,
      detail: "Teacher blocks require block teacher rewards or manage fraud blocks permission.",
      permissionTitle: "Block teacher rewards or manage fraud blocks permission required",
    },
    organization: {
      allowed: canBlockOrganizationRewards,
      detail: "Organization blocks require block org rewards or manage fraud blocks permission.",
      permissionTitle: "Block org rewards or manage fraud blocks permission required",
    },
    course: {
      allowed: canManageFraudBlocks,
      detail: "Course blocks require manage fraud blocks permission.",
      permissionTitle: "Manage fraud blocks permission required",
    },
    reward_policy: {
      allowed: canManageFraudBlocks,
      detail: "Reward policy blocks require manage fraud blocks permission.",
      permissionTitle: "Manage fraud blocks permission required",
    },
  };
  const selectedFraudBlockScopePermission = fraudBlockScopePermissions[fraudBlock.scope_type] ?? {
    allowed: false,
    detail: "Selected fraud block scope is not supported.",
    permissionTitle: "Supported fraud block scope required",
  };
  const resultTone = result.status === "Pending" ? "pending" : result.ok ? "success" : "error";
  const resultToneClass =
    resultTone === "pending"
      ? styles.checking
      : resultTone === "success"
        ? styles.online
        : styles.offline;
  const resultToneLabel =
    resultTone === "pending" ? "Pending" : resultTone === "success" ? "OK" : "Error";
  const resultOutcome =
    resultTone === "pending"
      ? "Request pending"
      : resultTone === "success"
        ? "Request succeeded"
        : "Request failed";
  const resultAnnouncement = [
    result.label,
    result.status,
    result.status === resultOutcome ? undefined : resultOutcome,
  ]
    .filter(Boolean)
    .join(". ")
    .concat(".");
  const firstAllowedFraudBlockScope = fraudBlockScopes.find(
    (scope) => fraudBlockScopePermissions[scope]?.allowed
  );
  const activeFraudBlockScope =
    selectedFraudBlockScopePermission.allowed || !firstAllowedFraudBlockScope
      ? fraudBlock.scope_type
      : firstAllowedFraudBlockScope;
  const activeFraudBlockScopePermission = fraudBlockScopePermissions[activeFraudBlockScope] ?? {
    allowed: false,
    detail: "Selected fraud block scope is not supported.",
    permissionTitle: "Supported fraud block scope required",
  };
  const canCreateFraudBlockTarget =
    (activeFraudBlockScope === "teacher" && hasPositiveInteger(fraudBlock.teacher_user_id)) ||
    (activeFraudBlockScope === "organization" && hasPositiveInteger(fraudBlock.organization_id)) ||
    (activeFraudBlockScope === "course" && hasPositiveInteger(fraudBlock.course_id)) ||
    (activeFraudBlockScope === "reward_policy" && hasPositiveInteger(fraudBlock.reward_policy_id));
  const canCreateFraudBlockFields = canCreateFraudBlockTarget && hasText(fraudBlock.reason);
  const canCreateSelectedFraudScope = activeFraudBlockScopePermission.allowed;
  const canUseFraudBlockForm = hasPositiveInteger(fraudBlockId);
  const canUseFraudBlockActions = canViewFraud || canManageFraud;
  const fraudBlockActionLabel =
    canViewFraud && canManageFraud
      ? "Audit or revoke block"
      : canViewFraud
        ? "Audit block"
        : "Revoke block";
  const scopedDelegatedPermissions = delegatedPermissionOptions
    .filter((permission) => permission.scopes.includes(delegation.scope_type))
    .map((permission) => permission.key);
  const activeDelegatedPermission = scopedDelegatedPermissions.includes(delegation.permission)
    ? delegation.permission
    : scopedDelegatedPermissions[0] || delegation.permission;
  const canGrantDelegationForm =
    hasPositiveInteger(delegation.grantee_user_id) &&
    scopedDelegatedPermissions.length > 0 &&
    (delegation.scope_type === "platform" ||
      (delegation.scope_type === "organization" &&
        hasPositiveInteger(delegation.organization_id)) ||
      (delegation.scope_type === "course" && hasPositiveInteger(delegation.course_id)));
  const canRevokeDelegationForm = hasPositiveInteger(delegationId);
  const hasTeacherApplicationDraft =
    teacherForm.requested_scope !== DEFAULT_TEACHER_APPLICATION_SCOPE ||
    hasAnyText([
      teacherForm.requested_organization_id,
      teacherForm.requested_course_id,
      teacherForm.experience_summary,
      teacherForm.organization_sponsor_id,
      teacherForm.portfolio_links,
    ]);
  const hasTeacherDecisionDraft =
    hasAnyText([teacherDecision.application_id, teacherDecision.decision_reason]) ||
    teacherDecision.status !== DEFAULT_TEACHER_DECISION_STATUS;
  const hasSubmitRewardCandidateDraft = hasAnyText([rewardCourseId, rewardStudentId]);
  const hasLoadRewardCandidatesDraft =
    hasText(rewardCourseId) || rewardStatus !== DEFAULT_REWARD_STATUS;
  const hasTeacherRewardDecisionDraft =
    hasAnyText([rewardCourseId, rewardCandidateId, teacherRewardDecision.decision_reason]) ||
    teacherRewardDecision.status !== DEFAULT_TEACHER_REWARD_DECISION_STATUS;
  const hasAmountDecisionDraft =
    hasText(rewardCandidateId) ||
    amountDecision.approved_amount !== DEFAULT_AMOUNT_VALUE ||
    hasText(amountDecision.decision_reason) ||
    amountDecision.status !== DEFAULT_AMOUNT_DECISION_STATUS;
  const hasOrganizationReportDraft = hasText(organizationId);
  const hasFraudBlockCreateDraft =
    fraudBlock.scope_type !== DEFAULT_FRAUD_BLOCK_SCOPE ||
    hasAnyText([
      fraudBlock.teacher_user_id,
      fraudBlock.organization_id,
      fraudBlock.course_id,
      fraudBlock.reward_policy_id,
      fraudBlock.reason,
      fraudBlock.evidence_reference,
    ]);
  const hasFraudBlockUseDraft = hasText(fraudBlockId);
  const hasDelegationGrantDraft =
    delegation.scope_type !== DEFAULT_DELEGATION_SCOPE ||
    activeDelegatedPermission !== DEFAULT_DELEGATED_PERMISSION ||
    hasAnyText([
      delegation.grantee_user_id,
      delegation.organization_id,
      delegation.course_id,
      delegation.reason,
      delegation.expires_at,
    ]);
  const hasDelegationRevokeDraft = hasAnyText([delegationId, revokeReason]);
  const teacherApplicationMissingFields = missingFields([
    ["Experience summary", hasText(teacherForm.experience_summary)],
    [
      "Organization id or sponsor org id",
      teacherForm.requested_scope !== "organization" ||
        hasPositiveInteger(teacherForm.requested_organization_id) ||
        hasPositiveInteger(teacherForm.organization_sponsor_id),
    ],
    [
      "Course id",
      teacherForm.requested_scope !== "course" ||
        hasPositiveInteger(teacherForm.requested_course_id),
    ],
  ]);
  const teacherDecisionMissingFields = missingFields([
    ["Application id", hasPositiveInteger(teacherDecision.application_id)],
  ]);
  const submitRewardCandidateMissingFields = missingFields([
    ["Course id", hasPositiveInteger(rewardCourseId)],
    ["Student user id", hasPositiveInteger(rewardStudentId)],
  ]);
  const loadRewardCandidatesMissingFields = missingFields([
    ["Course id", hasPositiveInteger(rewardCourseId)],
  ]);
  const teacherRewardDecisionMissingFields = missingFields([
    ["Course id", hasPositiveInteger(rewardCourseId)],
    ["Candidate id", hasPositiveInteger(rewardCandidateId)],
  ]);
  const amountDecisionMissingFields = missingFields([
    ["Candidate id", hasPositiveInteger(rewardCandidateId)],
    [
      "Approved amount",
      amountDecision.status !== "approved" || hasNonNegativeNumber(amountDecision.approved_amount),
    ],
  ]);
  const organizationReportMissingFields = missingFields([
    ["Organization id", hasPositiveInteger(organizationId)],
  ]);
  const fraudBlockCreateMissingFields = missingFields([
    [
      "Teacher user id",
      activeFraudBlockScope !== "teacher" || hasPositiveInteger(fraudBlock.teacher_user_id),
    ],
    [
      "Organization id",
      activeFraudBlockScope !== "organization" ||
        hasPositiveInteger(fraudBlock.organization_id),
    ],
    ["Course id", activeFraudBlockScope !== "course" || hasPositiveInteger(fraudBlock.course_id)],
    [
      "Policy id",
      activeFraudBlockScope !== "reward_policy" ||
        hasPositiveInteger(fraudBlock.reward_policy_id),
    ],
    ["Reason", hasText(fraudBlock.reason)],
  ]);
  const fraudBlockUseMissingFields = missingFields([
    ["Block id", hasPositiveInteger(fraudBlockId)],
  ]);
  const delegationGrantMissingFields = missingFields([
    ["Grantee user id", hasPositiveInteger(delegation.grantee_user_id)],
    [
      "Organization id",
      delegation.scope_type !== "organization" || hasPositiveInteger(delegation.organization_id),
    ],
    ["Course id", delegation.scope_type !== "course" || hasPositiveInteger(delegation.course_id)],
  ]);
  const delegationRevokeMissingFields = missingFields([
    ["Delegation id", hasPositiveInteger(delegationId)],
  ]);

  function togglePermission(permission: string) {
    setSelectedPermissions((current) => {
      const next = new Set(current);
      if (next.has(permission)) {
        next.delete(permission);
      } else {
        next.add(permission);
      }
      return next;
    });
  }

  async function signIn(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!hasCompleteCredentials) {
      setSessionMessage("Email and password required");
      return;
    }
    if (!beginPendingAction("Sign in")) {
      setSessionMessage(`${pendingActionRef.current} request in progress`);
      return;
    }

    const root = normalizeRoot(apiRoot);
    setSessionMessage("Signing in");
    setResult({
      label: "Sign in",
      status: "Pending",
      body: "Waiting for API response.",
      ok: true,
    });

    try {
      const response = await fetchWithTimeout(`${root}/auth/login`, {
        method: "POST",
        headers: {
          Accept: "application/json, text/plain",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          email: credentials.email.trim(),
          password: credentials.password,
        }),
      });
      const text = await response.text();

      if (!response.ok) {
        const body = prettyBody(text);
        setSessionMessage(body || `Sign in failed with HTTP ${response.status}`);
        setResult({
          label: "Sign in",
          status: `HTTP ${response.status}`,
          body,
          ok: false,
        });
        return;
      }

      let nextToken = text.trim();
      try {
        const parsed: unknown = JSON.parse(text);
        if (typeof parsed === "string") {
          nextToken = parsed;
        } else if (
          parsed &&
          typeof parsed === "object" &&
          "token" in parsed &&
          typeof parsed.token === "string"
        ) {
          nextToken = parsed.token;
        }
      } catch {
        // The API currently returns a JSON string, but keep plain text tolerant.
      }

      if (!nextToken) {
        setSessionMessage("Sign in response did not include a JWT");
        setResult({
          label: "Sign in",
          status: `HTTP ${response.status}`,
          body: "Sign in response did not include a JWT.",
          ok: false,
        });
        return;
      }

      setToken(nextToken);
      setShowToken(false);
      resetServerDenials();
      setCredentials((current) => ({ ...current, password: "" }));
      setSessionMessage("Signed in");
      setResult({
        label: "Sign in",
        status: `HTTP ${response.status}`,
        body: "JWT loaded into this session.",
        ok: true,
      });
    } catch (error) {
      const message = requestFailureMessage(error);
      setSessionMessage(message);
      setResult({
        label: "Sign in",
        status: "Request failed",
        body: message,
        ok: false,
      });
    } finally {
      finishPendingAction("Sign in");
    }
  }

  function resetSessionResult() {
    setResult({
      label: "Result",
      status: "Idle",
      body: "Session cleared. Previous API response hidden.",
      ok: true,
    });
  }

  function clearSession() {
    setToken("");
    setShowToken(false);
    setCredentials({ email: "", password: "" });
    resetServerDenials();
    setSessionMessage("Session fields cleared");
    resetSessionResult();
  }

  async function sendApi(label: string, path: string, method: HttpMethod = "GET", body?: unknown) {
    const revealResultPanel = () => {
      requestAnimationFrame(() => {
        resultPanelRef.current?.scrollIntoView({ block: "start", inline: "nearest" });
      });
    };

    if (!hasSessionToken) {
      setResult({
        label,
        status: "Session required",
        body: "Add a JWT before sending protected API requests.",
        ok: false,
      });
      revealResultPanel();
      return;
    }
    if (!beginPendingAction(label)) {
      setResult({
        label,
        status: "Request pending",
        body: `${pendingActionRef.current} is already waiting for an API response.`,
        ok: false,
      });
      revealResultPanel();
      return;
    }

    const root = normalizeRoot(apiRoot);
    const headers = new Headers();
    headers.set("Accept", "application/json, text/csv, text/plain");
    if (body !== undefined) {
      headers.set("Content-Type", "application/json");
    }
    if (token.trim()) {
      headers.set("Authorization", `Bearer ${token.trim()}`);
    }

    setResult({ label, status: "Pending", body: "Waiting for API response.", ok: true });
    revealResultPanel();

    try {
      const response = await fetchWithTimeout(`${root}${path}`, {
        method,
        headers,
        body: body === undefined ? undefined : JSON.stringify(body),
      });
      const text = await response.text();
      const responseBody = prettyBody(text);
      setServerDeniedAction(label, response.status === 403);
      setResult({
        label,
        status: `HTTP ${response.status}`,
        body:
          response.status === 403
            ? [responseBody, "This JWT does not grant server access for this action."]
                .filter(Boolean)
                .join("\n\n")
            : responseBody,
        ok: response.ok,
      });
      revealResultPanel();
    } catch (error) {
      setResult({
        label,
        status: "Request failed",
        body: requestFailureMessage(error),
        ok: false,
      });
      revealResultPanel();
    } finally {
      finishPendingAction(label);
    }
  }

  function submitTeacherApplication() {
    const requestedScope = teacherForm.requested_scope;

    void sendApi(PROTECTED_ACTIONS.submitTeacherApplication, "/teacher-applications", "POST", {
      requested_scope: requestedScope,
      requested_organization_id:
        requestedScope === "organization"
          ? optionalPositiveInteger(teacherForm.requested_organization_id)
          : undefined,
      requested_course_id:
        requestedScope === "course" ? optionalPositiveInteger(teacherForm.requested_course_id) : undefined,
      experience_summary: teacherForm.experience_summary,
      organization_sponsor_id: optionalPositiveInteger(teacherForm.organization_sponsor_id),
      portfolio_links: splitLinks(teacherForm.portfolio_links),
    });
  }

  function loadTeacherApplications() {
    void sendApi(
      PROTECTED_ACTIONS.teacherApplicationQueue,
      `/teacher-applications${buildQuery({ status: teacherStatus, limit: 25 })}`
    );
  }

  function decideTeacherApplication() {
    void sendApi(
      PROTECTED_ACTIONS.teacherApplicationDecision,
      `/teacher-applications/${teacherDecision.application_id}/decision`,
      "PUT",
      {
        status: activeTeacherDecisionStatus,
        decision_reason: teacherDecision.decision_reason || undefined,
      }
    );
  }

  function submitRewardCandidate() {
    void sendApi(PROTECTED_ACTIONS.submitRewardCandidate, `/courses/${rewardCourseId}/reward-candidates`, "POST", {
      student_user_id: optionalPositiveInteger(rewardStudentId),
      event_type: "course_completion",
      evidence: { completion_percentage: 100 },
    });
  }

  function loadRewardCandidates() {
    void sendApi(
      PROTECTED_ACTIONS.courseRewardCandidates,
      `/courses/${rewardCourseId}/reward-candidates${buildQuery({
        status: rewardStatus,
        limit: 25,
      })}`
    );
  }

  function decideStudentReward() {
    void sendApi(
      PROTECTED_ACTIONS.courseRewardDecision,
      `/courses/${rewardCourseId}/reward-candidates/${rewardCandidateId}/teacher-decision`,
      "PUT",
      {
        status: teacherRewardDecision.status,
        decision_reason: teacherRewardDecision.decision_reason || undefined,
      }
    );
  }

  function decideRewardAmount() {
    void sendApi(PROTECTED_ACTIONS.rewardAmountDecision, `/reward-candidates/${rewardCandidateId}/amount-decision`, "PUT", {
      status: amountDecision.status,
      approved_amount:
        amountDecision.status === "approved" ? optionalNumber(amountDecision.approved_amount) : undefined,
      decision_reason: amountDecision.decision_reason || undefined,
    });
  }

  function loadStudentHistory() {
    void sendApi(
      PROTECTED_ACTIONS.studentRewardHistory,
      `/reward-candidates/me/history${buildQuery({ status: historyStatus, limit: 25 })}`
    );
  }

  function loadOrganizationReport(csv = false) {
    void sendApi(
      csv ? PROTECTED_ACTIONS.organizationRewardCsv : PROTECTED_ACTIONS.organizationRewardReport,
      `/reports/organizations/${organizationId}/reward-dashboard${csv ? ".csv" : ""}`
    );
  }

  function loadOrganizationSummary(csv = false) {
    void sendApi(
      csv ? PROTECTED_ACTIONS.organizationSummaryCsv : PROTECTED_ACTIONS.organizationSummary,
      `/reports/organizations/${organizationId}/summary${csv ? ".csv" : ""}`
    );
  }

  function loadPlatformExport(path: string, label: string) {
    void sendApi(label, path);
  }

  function createFraudBlock() {
    void sendApi(PROTECTED_ACTIONS.createRewardFraudBlock, "/reward-fraud-blocks", "POST", {
      scope_type: activeFraudBlockScope,
      teacher_user_id:
        activeFraudBlockScope === "teacher"
          ? optionalPositiveInteger(fraudBlock.teacher_user_id)
          : undefined,
      organization_id:
        activeFraudBlockScope === "organization"
          ? optionalPositiveInteger(fraudBlock.organization_id)
          : undefined,
      course_id:
        activeFraudBlockScope === "course" ? optionalPositiveInteger(fraudBlock.course_id) : undefined,
      reward_policy_id:
        activeFraudBlockScope === "reward_policy"
          ? optionalPositiveInteger(fraudBlock.reward_policy_id)
          : undefined,
      reason: fraudBlock.reason,
      evidence_reference: fraudBlock.evidence_reference || undefined,
    });
  }

  function listFraudBlocks() {
    void sendApi(PROTECTED_ACTIONS.rewardFraudBlocks, "/reward-fraud-blocks?active=true&limit=25");
  }

  function revokeFraudBlock() {
    void sendApi(PROTECTED_ACTIONS.revokeRewardFraudBlock, `/reward-fraud-blocks/${fraudBlockId}/revoke`, "PUT");
  }

  function loadFraudAudit() {
    void sendApi(PROTECTED_ACTIONS.rewardFraudAudit, `/reward-fraud-blocks/${fraudBlockId}/audit`);
  }

  function grantDelegation() {
    void sendApi(PROTECTED_ACTIONS.grantDelegatedPermission, "/delegated-permissions", "POST", {
      grantee_user_id: optionalPositiveInteger(delegation.grantee_user_id),
      permission: activeDelegatedPermission,
      scope_type: delegation.scope_type,
      organization_id:
        delegation.scope_type === "organization"
          ? optionalPositiveInteger(delegation.organization_id)
          : undefined,
      course_id:
        delegation.scope_type === "course" ? optionalPositiveInteger(delegation.course_id) : undefined,
      reason: delegation.reason || undefined,
      expires_at: optionalUtcDateTime(delegation.expires_at),
    });
  }

  function listDelegations() {
    void sendApi(PROTECTED_ACTIONS.delegatedPermissions, "/delegated-permissions?active=true&limit=25");
  }

  function revokeDelegation() {
    void sendApi(PROTECTED_ACTIONS.revokeDelegatedPermission, `/delegated-permissions/${delegationId}/revoke`, "PUT", {
      revoke_reason: revokeReason || undefined,
    });
  }

  return (
    <main className={styles.shell}>
      <aside className={styles.sidebar} aria-label="Workspace controls">
        <div className={styles.brand}>
          <span className={styles.brandMark}>RL</span>
          <div>
            <p className={styles.brandName}>RustLearn</p>
            <p className={styles.brandMeta}>Reward operations</p>
          </div>
        </div>

        <section
          className={`${styles.sidebarSection} ${styles.sessionPanel}`}
          aria-labelledby="api-session-title"
        >
          <div className={styles.sectionHeaderCompact}>
            <h2 id="api-session-title">Session</h2>
            <span className={`${styles.statusPill} ${styles[apiState]}`}>{statusLabel(apiState)}</span>
          </div>
          <label className={styles.fieldLabel}>
            API root
            <input
              value={apiRoot}
              onChange={(event) => {
                setApiState("checking");
                setApiMessage("Checking API");
                resetServerDenials();
                setApiRoot(event.target.value);
              }}
            />
          </label>
          <form className={styles.sessionForm} onSubmit={signIn}>
            <label className={styles.fieldLabel}>
              Email
              <input
                type="email"
                autoComplete="email"
                value={credentials.email}
                onChange={(event) =>
                  setCredentials((current) => ({ ...current, email: event.target.value }))
                }
              />
            </label>
            <label className={styles.fieldLabel}>
              Password
              <input
                type="password"
                autoComplete="current-password"
                value={credentials.password}
                onChange={(event) =>
                  setCredentials((current) => ({ ...current, password: event.target.value }))
                }
              />
            </label>
            <div className={styles.sessionActions}>
              <button
                type="submit"
                className={styles.primaryButton}
                disabled={!canSignIn}
                title={
                  pendingActionTitle ??
                  (hasCompleteCredentials ? undefined : "Email and password required")
                }
              >
                <KeyRound size={17} aria-hidden />
                <span>{pendingAction === "Sign in" ? "Signing in" : "Sign in"}</span>
              </button>
              <button
                type="button"
                className={styles.secondaryButton}
                onClick={clearSession}
                disabled={!hasSessionDraft || hasPendingAction}
                title={
                  pendingActionTitle ??
                  (hasSessionDraft ? "Clear local session fields" : "No session fields to clear")
                }
                aria-label="Clear session fields"
              >
                <Ban size={17} aria-hidden />
                <span>Clear</span>
              </button>
            </div>
          </form>
          <div className={styles.fieldLabel}>
            <span>JWT</span>
            <span className={styles.secretField}>
              <input
                aria-label="JWT"
                type={showToken ? "text" : "password"}
                autoComplete="off"
                value={token}
                onChange={(event) => {
                  const nextToken = event.target.value;
                  resetServerDenials();
                  setToken(nextToken);
                  if (hasText(nextToken)) {
                    setSessionMessage("JWT loaded");
                  } else {
                    setShowToken(false);
                    setSessionMessage("Session cleared");
                    resetSessionResult();
                  }
                }}
                spellCheck={false}
              />
              <button
                type="button"
                className={styles.tokenVisibilityButton}
                onClick={() => setShowToken((current) => !current)}
                disabled={!hasSessionToken}
                aria-label={showToken ? "Hide JWT" : "Show JWT"}
                aria-pressed={showToken}
                title={showToken ? "Hide JWT" : "Show JWT"}
              >
                {showToken ? <EyeOff size={17} aria-hidden /> : <Eye size={17} aria-hidden />}
              </button>
            </span>
          </div>
          <p className={styles.statusMessage} aria-live="polite">
            {sessionMessage}
          </p>
          <p className={styles.statusMessage} aria-live="polite">
            {apiMessage}
          </p>
        </section>

        <section
          className={`${styles.sidebarSection} ${styles.permissionsPanel}`}
          aria-labelledby="permissions-title"
        >
          <div className={styles.sectionHeaderCompact}>
            <h2 id="permissions-title">Permissions</h2>
            <span className={styles.countPill}>{selectedPermissions.size}</span>
          </div>
          {Object.entries(permissionGroups).map(([scope, permissions]) => (
            <div key={scope} className={styles.permissionGroup}>
              <p>{scope}</p>
              {permissions.map((permission) => (
                <label key={permission.key} className={styles.checkboxRow}>
                  <input
                    type="checkbox"
                    checked={selectedPermissions.has(permission.key)}
                    onChange={() => togglePermission(permission.key)}
                  />
                  <span>{permission.label}</span>
                </label>
              ))}
            </div>
          ))}
        </section>
      </aside>

      <section className={styles.workspace}>
        <header className={styles.topbar}>
          <div>
            <p className={styles.eyebrow}>Business console</p>
            <h1>Reward and teaching workflows</h1>
          </div>
          <button
            type="button"
            className={styles.iconButton}
            disabled={apiState === "checking"}
            aria-label="Check API health"
            title="Check API health"
            onClick={() => {
              setApiState("checking");
              setApiMessage("Checking API");
              setHealthCheckTick((current) => current + 1);
            }}
          >
            <RefreshCw size={18} aria-hidden />
            <span>{apiState === "checking" ? "Checking" : "Check API"}</span>
          </button>
        </header>

        <nav className={styles.workflowNav} aria-label="Workflow sections">
          {workflowNavItems.map((item) => (
            <a key={item.href} href={item.href}>
              {item.label}
            </a>
          ))}
        </nav>

        <section className={styles.metrics} aria-label="Workflow access">
          <div className={styles.metric}>
            <span>Teacher flow</span>
            <strong>
              {flowStatus(canUseTeacherWorkflow, hasSessionToken, teacherWorkflowServerDenied)}
            </strong>
          </div>
          <div className={styles.metric}>
            <span>Reward flow</span>
            <strong>
              {flowStatus(canUseRewardWorkflow, hasSessionToken, rewardWorkflowServerDenied)}
            </strong>
          </div>
          <div className={styles.metric}>
            <span>Audit flow</span>
            <strong>
              {flowStatus(canUseAuditWorkflow, hasSessionToken, auditWorkflowServerDenied)}
            </strong>
          </div>
        </section>

        {!hasSessionToken && (
          <section className={styles.workflowNotice} aria-label="Session requirement">
            <KeyRound size={18} aria-hidden />
            <div>
              <strong>JWT required</strong>
              <span>Protected workflow actions are locked until a session token is loaded.</span>
            </div>
          </section>
        )}

        <div className={styles.grid}>
          <section
            id="teacher-workflow"
            className={styles.panel}
            aria-labelledby="teacher-title"
          >
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Teacher applications</p>
                <h2 id="teacher-title">Application and review</h2>
              </div>
              <GraduationCap size={22} aria-hidden />
            </div>

            {!canUseTeacherWorkflow && (
              <PermissionNotice
                title="Teacher permissions disabled"
                detail="Enable application or review permissions to show teacher workflow actions."
              />
            )}

            {canTeacherApply && (
              <>
                {hasSessionToken && hasTeacherApplicationDraft && (
                  <RequirementNotice
                    action="Submit application"
                    fields={teacherApplicationMissingFields}
                  />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.submitTeacherApplication, "Submit application")}
                <div className={styles.formGrid}>
                  <label className={styles.fieldLabel}>
                    Scope
                    <select
                      value={teacherForm.requested_scope}
                      onChange={(event) =>
                        setTeacherForm((current) => ({
                          ...current,
                          requested_scope: event.target.value,
                        }))
                      }
                    >
                      <option value="platform">{optionLabel("platform")}</option>
                      <option value="organization">{optionLabel("organization")}</option>
                      <option value="course">{optionLabel("course")}</option>
                    </select>
                  </label>
                  {teacherForm.requested_scope === "organization" && (
                    <label className={styles.fieldLabel}>
                      Organization id
                      <input
                        {...positiveIntegerInputProps}
                        value={teacherForm.requested_organization_id}
                        onChange={(event) =>
                          setTeacherForm((current) => ({
                            ...current,
                            requested_organization_id: event.target.value,
                          }))
                        }
                      />
                    </label>
                  )}
                  {teacherForm.requested_scope === "course" && (
                    <label className={styles.fieldLabel}>
                      Course id
                      <input
                        {...positiveIntegerInputProps}
                        value={teacherForm.requested_course_id}
                        onChange={(event) =>
                          setTeacherForm((current) => ({
                            ...current,
                            requested_course_id: event.target.value,
                          }))
                        }
                      />
                    </label>
                  )}
                  <label className={styles.fieldLabel}>
                    Sponsor org id
                    <input
                      {...positiveIntegerInputProps}
                      value={teacherForm.organization_sponsor_id}
                      onChange={(event) =>
                        setTeacherForm((current) => ({
                          ...current,
                          organization_sponsor_id: event.target.value,
                        }))
                      }
                    />
                  </label>
                  <label className={`${styles.fieldLabel} ${styles.fullWidth}`}>
                    Experience summary
                    <textarea
                      rows={3}
                      value={teacherForm.experience_summary}
                      onChange={(event) =>
                        setTeacherForm((current) => ({
                          ...current,
                          experience_summary: event.target.value,
                        }))
                      }
                    />
                  </label>
                  <label className={`${styles.fieldLabel} ${styles.fullWidth}`}>
                    Portfolio links
                    <textarea
                      rows={2}
                      value={teacherForm.portfolio_links}
                      onChange={(event) =>
                        setTeacherForm((current) => ({
                          ...current,
                          portfolio_links: event.target.value,
                        }))
                      }
                    />
                  </label>
                  <button
                    type="button"
                    className={styles.primaryButton}
                    onClick={submitTeacherApplication}
                    {...actionState(
                      canSubmitTeacherApplicationForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.submitTeacherApplication
                    )}
                  >
                    <Send size={17} aria-hidden />
                    <span>Submit</span>
                  </button>
                </div>
              </>
            )}

            {canReviewTeachers && (
              <>
                {serverDeniedNotice(PROTECTED_ACTIONS.teacherApplicationQueue, "Load queue")}
                <div className={styles.actionStrip}>
                  <select
                    aria-label="Teacher application status filter"
                    value={teacherStatus}
                    onChange={(event) => setTeacherStatus(event.target.value)}
                  >
                    {teacherApplicationStatuses.map((status) => (
                      <option key={status} value={status}>
                        {optionLabel(status)}
                      </option>
                    ))}
                  </select>
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={loadTeacherApplications}
                    {...actionState(
                      true,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.teacherApplicationQueue
                    )}
                  >
                    <ClipboardList size={17} aria-hidden />
                    <span>Load queue</span>
                  </button>
                </div>
              </>
            )}

            {canDecideTeachers && (
              <>
                {hasSessionToken && hasTeacherDecisionDraft && (
                  <RequirementNotice action="Decide" fields={teacherDecisionMissingFields} />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.teacherApplicationDecision, "Decide")}
                <div className={styles.actionStrip}>
                  <input
                    aria-label="Teacher application id"
                    {...positiveIntegerInputProps}
                    placeholder="Application id"
                    value={teacherDecision.application_id}
                    onChange={(event) =>
                      setTeacherDecision((current) => ({
                        ...current,
                        application_id: event.target.value,
                      }))
                    }
                  />
                  <select
                    aria-label="Teacher decision status"
                    value={activeTeacherDecisionStatus}
                    onChange={(event) =>
                      setTeacherDecision((current) => ({ ...current, status: event.target.value }))
                    }
                  >
                    {teacherDecisionStatuses.map((status) => (
                      <option key={status} value={status}>
                        {optionLabel(status)}
                      </option>
                    ))}
                  </select>
                  <input
                    aria-label="Teacher decision reason"
                    placeholder="Reason"
                    value={teacherDecision.decision_reason}
                    onChange={(event) =>
                      setTeacherDecision((current) => ({
                        ...current,
                        decision_reason: event.target.value,
                      }))
                    }
                  />
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={decideTeacherApplication}
                    {...actionState(
                      canDecideTeacherApplicationForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.teacherApplicationDecision
                    )}
                  >
                    <CheckCircle2 size={17} aria-hidden />
                    <span>Decide</span>
                  </button>
                </div>
              </>
            )}
          </section>

          <section id="reward-workflow" className={styles.panel} aria-labelledby="reward-title">
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Course rewards</p>
                <h2 id="reward-title">Candidate approval</h2>
              </div>
              <FileCheck size={22} aria-hidden />
            </div>

            {showRewardSharedFields && (
              <div className={styles.actionStrip}>
                {showRewardCourseId && (
                  <input
                    aria-label="Reward course id"
                    {...positiveIntegerInputProps}
                    placeholder="Course id"
                    value={rewardCourseId}
                    onChange={(event) => setRewardCourseId(event.target.value)}
                  />
                )}
                {showRewardCandidateId && (
                  <input
                    aria-label="Reward candidate id"
                    {...positiveIntegerInputProps}
                    placeholder="Candidate id"
                    value={rewardCandidateId}
                    onChange={(event) => setRewardCandidateId(event.target.value)}
                  />
                )}
                {showRewardStatusFilter && (
                  <select
                    aria-label="Reward candidate status filter"
                    value={rewardStatus}
                    onChange={(event) => setRewardStatus(event.target.value)}
                  >
                    {rewardStatuses.map((status) => (
                      <option key={status} value={status}>
                        {optionLabel(status)}
                      </option>
                    ))}
                  </select>
                )}
              </div>
            )}

            {!canUseRewardWorkflow && (
              <PermissionNotice
                title="Reward permissions disabled"
                detail="Enable course reward permissions to submit, load, or approve candidates."
              />
            )}

            {canSubmitReward && (
              <>
                {hasSessionToken && hasSubmitRewardCandidateDraft && (
                  <RequirementNotice
                    action="Submit candidate"
                    fields={submitRewardCandidateMissingFields}
                  />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.submitRewardCandidate, "Submit candidate")}
                <div className={styles.actionStrip}>
                  <input
                    aria-label="Reward student user id"
                    {...positiveIntegerInputProps}
                    placeholder="Student user id"
                    value={rewardStudentId}
                    onChange={(event) => setRewardStudentId(event.target.value)}
                  />
                  <button
                    type="button"
                    className={styles.primaryButton}
                    onClick={submitRewardCandidate}
                    {...actionState(
                      canSubmitRewardCandidateForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.submitRewardCandidate
                    )}
                  >
                    <Send size={17} aria-hidden />
                    <span>Submit candidate</span>
                  </button>
                </div>
              </>
            )}

            {canViewCourseRewards && (
              <>
                {hasSessionToken && hasLoadRewardCandidatesDraft && (
                  <RequirementNotice
                    action="Load candidates"
                    fields={loadRewardCandidatesMissingFields}
                  />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.courseRewardCandidates, "Load candidates")}
                <button
                  type="button"
                  className={styles.secondaryButton}
                  onClick={loadRewardCandidates}
                  {...actionState(
                    canLoadRewardCandidatesForm,
                    true,
                    "Required permission",
                    PROTECTED_ACTIONS.courseRewardCandidates
                  )}
                >
                  <ClipboardList size={17} aria-hidden />
                  <span>Load candidates</span>
                </button>
              </>
            )}

            {canTeacherApproveReward && (
              <>
                {hasSessionToken && hasTeacherRewardDecisionDraft && (
                  <RequirementNotice
                    action="Teacher decision"
                    fields={teacherRewardDecisionMissingFields}
                  />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.courseRewardDecision, "Teacher decision")}
                <div className={styles.actionStrip}>
                  <select
                    aria-label="Teacher reward decision status"
                    value={teacherRewardDecision.status}
                    onChange={(event) =>
                      setTeacherRewardDecision((current) => ({
                        ...current,
                        status: event.target.value,
                      }))
                    }
                  >
                    <option value="approved">{optionLabel("approved")}</option>
                    <option value="rejected">{optionLabel("rejected")}</option>
                  </select>
                  <input
                    aria-label="Teacher reward decision reason"
                    placeholder="Teacher reason"
                    value={teacherRewardDecision.decision_reason}
                    onChange={(event) =>
                      setTeacherRewardDecision((current) => ({
                        ...current,
                        decision_reason: event.target.value,
                      }))
                    }
                  />
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={decideStudentReward}
                    {...actionState(
                      canDecideStudentRewardForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.courseRewardDecision
                    )}
                  >
                    <CheckCircle2 size={17} aria-hidden />
                    <span>Teacher decision</span>
                  </button>
                </div>
              </>
            )}

            {canApproveAmount && (
              <>
                {hasSessionToken && hasAmountDecisionDraft && (
                  <RequirementNotice action="Set amount" fields={amountDecisionMissingFields} />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.rewardAmountDecision, "Set amount")}
                <div className={styles.actionStrip}>
                  <select
                    aria-label="Reward amount decision status"
                    value={amountDecision.status}
                    onChange={(event) =>
                      setAmountDecision((current) => ({ ...current, status: event.target.value }))
                    }
                  >
                    <option value="approved">{optionLabel("approved")}</option>
                    <option value="rejected">{optionLabel("rejected")}</option>
                  </select>
                  {amountDecision.status === "approved" && (
                    <input
                      aria-label="Approved reward amount"
                      {...decimalInputProps}
                      placeholder="Amount"
                      value={amountDecision.approved_amount}
                      onChange={(event) =>
                        setAmountDecision((current) => ({
                          ...current,
                          approved_amount: event.target.value,
                        }))
                      }
                    />
                  )}
                  <input
                    aria-label="Reward amount decision reason"
                    placeholder="Amount reason"
                    value={amountDecision.decision_reason}
                    onChange={(event) =>
                      setAmountDecision((current) => ({
                        ...current,
                        decision_reason: event.target.value,
                      }))
                    }
                  />
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={decideRewardAmount}
                    {...actionState(
                      canDecideRewardAmountForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.rewardAmountDecision
                    )}
                  >
                    <WalletCards size={17} aria-hidden />
                    <span>Set amount</span>
                  </button>
                </div>
              </>
            )}
          </section>

          <section id="history-workflow" className={styles.panel} aria-labelledby="history-title">
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Student</p>
                <h2 id="history-title">Reward history</h2>
              </div>
              <History size={22} aria-hidden />
            </div>
            {!canViewCourseRewards && (
              <PermissionNotice
                title="Reward history permission disabled"
                detail="Enable course reward status permission to load student history."
              />
            )}
            {canViewCourseRewards && (
              <>
                {serverDeniedNotice(PROTECTED_ACTIONS.studentRewardHistory, "Load history")}
                <div className={styles.actionStrip}>
                  <select
                    aria-label="Reward history status filter"
                    value={historyStatus}
                    onChange={(event) => setHistoryStatus(event.target.value)}
                  >
                    <option value="">All statuses</option>
                    {rewardStatuses.map((status) => (
                      <option key={status} value={status}>
                        {optionLabel(status)}
                      </option>
                    ))}
                  </select>
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={loadStudentHistory}
                    {...actionState(
                      true,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.studentRewardHistory
                    )}
                  >
                    <History size={17} aria-hidden />
                    <span>Load history</span>
                  </button>
                </div>
              </>
            )}
          </section>

          <section id="report-workflow" className={styles.panel} aria-labelledby="report-title">
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Organization</p>
                <h2 id="report-title">Reward report</h2>
              </div>
              <Download size={22} aria-hidden />
            </div>
            {!canUseReportWorkflow && (
              <PermissionNotice
                title="Reporting permissions disabled"
                detail="Enable summary, organization reward, reward audit, generate report, or export permissions to use reporting actions."
              />
            )}
            {hasSessionToken && canUseOrganizationReportControls && hasOrganizationReportDraft && (
              <RequirementNotice
                action="Load organization reports"
                fields={organizationReportMissingFields}
              />
            )}
            {serverDeniedNotice(PROTECTED_ACTIONS.organizationSummary, "Summary")}
            {serverDeniedNotice(PROTECTED_ACTIONS.organizationSummaryCsv, "Summary CSV")}
            {serverDeniedNotice(PROTECTED_ACTIONS.organizationRewardReport, "Reward report")}
            {serverDeniedNotice(PROTECTED_ACTIONS.organizationRewardCsv, "Reward CSV")}
            {canUseOrganizationReportControls && (
              <div className={styles.actionStrip}>
                <input
                  aria-label="Report organization id"
                  {...positiveIntegerInputProps}
                  placeholder="Organization id"
                  value={organizationId}
                  onChange={(event) => setOrganizationId(event.target.value)}
                />
                {canViewSummaryReports && (
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={() => loadOrganizationSummary(false)}
                    {...actionState(
                      canLoadOrganizationReportForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.organizationSummary
                    )}
                  >
                    <ClipboardList size={17} aria-hidden />
                    <span>Summary</span>
                  </button>
                )}
                {canGenerateReports && (
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={() => loadOrganizationSummary(true)}
                    {...actionState(
                      canLoadOrganizationReportForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.organizationSummaryCsv
                    )}
                  >
                    <Download size={17} aria-hidden />
                    <span>Summary CSV</span>
                  </button>
                )}
                {canViewOrgReports && (
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={() => loadOrganizationReport(false)}
                    {...actionState(
                      canLoadOrganizationReportForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.organizationRewardReport
                    )}
                  >
                    <ClipboardList size={17} aria-hidden />
                    <span>Reward report</span>
                  </button>
                )}
                {canViewOrgReports && (
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={() => loadOrganizationReport(true)}
                    {...actionState(
                      canLoadOrganizationReportForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.organizationRewardCsv
                    )}
                  >
                    <Download size={17} aria-hidden />
                    <span>Reward CSV</span>
                  </button>
                )}
              </div>
            )}
            {canExport &&
              platformExportReports.map((report) =>
                serverDeniedNotice(report.resultLabel, report.label, report.path)
              )}
            {canExport && (
              <div className={styles.reportLinks}>
                {platformExportReports.map((report) => (
                  <button
                    key={report.path}
                    type="button"
                    onClick={() => loadPlatformExport(report.path, report.resultLabel)}
                    {...actionState(true, true, "Required permission", report.resultLabel)}
                  >
                    <Download size={16} aria-hidden />
                    <span>{report.label}</span>
                  </button>
                ))}
              </div>
            )}
            {canViewSummaryReports &&
              serverDeniedNotice(PROTECTED_ACTIONS.platformSummary, "Platform summary")}
            {canViewSummaryReports && (
              <div className={styles.reportLinks}>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport(
                      "/reports/platform/summary",
                      PROTECTED_ACTIONS.platformSummary
                    )
                  }
                  {...actionState(
                    true,
                    true,
                    "Required permission",
                    PROTECTED_ACTIONS.platformSummary
                  )}
                >
                  <ClipboardList size={16} aria-hidden />
                  <span>Platform summary</span>
                </button>
              </div>
            )}
            {canViewPlatformDashboards &&
              serverDeniedNotice(PROTECTED_ACTIONS.platformRewardDashboard, "Reward dashboard")}
            {canViewPlatformDashboards &&
              serverDeniedNotice(PROTECTED_ACTIONS.platformFraudDashboard, "Fraud dashboard")}
            {canViewPlatformDashboards && (
              <div className={styles.reportLinks}>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport(
                      "/reports/platform/reward-dashboard",
                      PROTECTED_ACTIONS.platformRewardDashboard
                    )
                  }
                  {...actionState(
                    true,
                    true,
                    "Required permission",
                    PROTECTED_ACTIONS.platformRewardDashboard
                  )}
                >
                  <ClipboardList size={16} aria-hidden />
                  <span>Reward dashboard</span>
                </button>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport(
                      "/reports/platform/fraud-dashboard",
                      PROTECTED_ACTIONS.platformFraudDashboard
                    )
                  }
                  {...actionState(
                    true,
                    true,
                    "Required permission",
                    PROTECTED_ACTIONS.platformFraudDashboard
                  )}
                >
                  <ShieldAlert size={16} aria-hidden />
                  <span>Fraud dashboard</span>
                </button>
              </div>
            )}
          </section>

          <section id="fraud-workflow" className={styles.panel} aria-labelledby="fraud-title">
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Fraud controls</p>
                <h2 id="fraud-title">Reward blocks</h2>
              </div>
              <ShieldAlert size={22} aria-hidden />
            </div>
            {!canUseFraudWorkflow && (
              <PermissionNotice
                title="Fraud permissions disabled"
                detail="Enable fraud audit or management permissions to inspect reward blocks."
              />
            )}
            {canManageFraud && (
              <>
                {hasSessionToken && hasFraudBlockCreateDraft && (
                  <RequirementNotice
                    action="Create block"
                    fields={fraudBlockCreateMissingFields}
                  />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.createRewardFraudBlock, "Create block")}
                {hasSessionToken && !canCreateSelectedFraudScope && (
                  <PermissionNotice
                    title="Selected block scope permission disabled"
                    detail={selectedFraudBlockScopePermission.detail}
                  />
                )}
                <div className={styles.formGrid}>
                  <label className={styles.fieldLabel}>
                    Scope
                    <select
                      aria-label="Fraud block scope type"
                      value={activeFraudBlockScope}
                      onChange={(event) =>
                        setFraudBlock((current) => ({ ...current, scope_type: event.target.value }))
                      }
                    >
                      {fraudBlockScopes.map((scope) => (
                        <option
                          key={scope}
                          value={scope}
                          disabled={!fraudBlockScopePermissions[scope]?.allowed}
                        >
                          {optionLabel(scope)}
                        </option>
                      ))}
                    </select>
                  </label>
                  {activeFraudBlockScope === "teacher" && (
                    <input
                      aria-label="Fraud block teacher user id"
                      {...positiveIntegerInputProps}
                      placeholder="Teacher user id"
                      value={fraudBlock.teacher_user_id}
                      onChange={(event) =>
                        setFraudBlock((current) => ({
                          ...current,
                          teacher_user_id: event.target.value,
                        }))
                      }
                    />
                  )}
                  {activeFraudBlockScope === "organization" && (
                    <input
                      aria-label="Fraud block organization id"
                      {...positiveIntegerInputProps}
                      placeholder="Organization id"
                      value={fraudBlock.organization_id}
                      onChange={(event) =>
                        setFraudBlock((current) => ({
                          ...current,
                          organization_id: event.target.value,
                        }))
                      }
                    />
                  )}
                  {activeFraudBlockScope === "course" && (
                    <input
                      aria-label="Fraud block course id"
                      {...positiveIntegerInputProps}
                      placeholder="Course id"
                      value={fraudBlock.course_id}
                      onChange={(event) =>
                        setFraudBlock((current) => ({ ...current, course_id: event.target.value }))
                      }
                    />
                  )}
                  {activeFraudBlockScope === "reward_policy" && (
                    <input
                      aria-label="Fraud block policy id"
                      {...positiveIntegerInputProps}
                      placeholder="Policy id"
                      value={fraudBlock.reward_policy_id}
                      onChange={(event) =>
                        setFraudBlock((current) => ({
                          ...current,
                          reward_policy_id: event.target.value,
                        }))
                      }
                    />
                  )}
                  <input
                    aria-label="Fraud block evidence reference"
                    placeholder="Evidence reference"
                    value={fraudBlock.evidence_reference}
                    onChange={(event) =>
                      setFraudBlock((current) => ({
                        ...current,
                        evidence_reference: event.target.value,
                      }))
                    }
                  />
                  <label className={`${styles.fieldLabel} ${styles.fullWidth}`}>
                    Reason
                    <textarea
                      rows={2}
                      value={fraudBlock.reason}
                      onChange={(event) =>
                        setFraudBlock((current) => ({ ...current, reason: event.target.value }))
                      }
                    />
                  </label>
                  <button
                    type="button"
                    className={styles.primaryButton}
                    onClick={createFraudBlock}
                    {...actionState(
                      canCreateFraudBlockFields,
                      canCreateSelectedFraudScope,
                      selectedFraudBlockScopePermission.permissionTitle,
                      PROTECTED_ACTIONS.createRewardFraudBlock
                    )}
                  >
                    <Ban size={17} aria-hidden />
                    <span>Create block</span>
                  </button>
                </div>
              </>
            )}
            {canUseFraudBlockActions && (
              <>
                {hasSessionToken && hasFraudBlockUseDraft && (
                  <RequirementNotice
                    action={fraudBlockActionLabel}
                    fields={fraudBlockUseMissingFields}
                  />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.rewardFraudBlocks, "Load active")}
                {serverDeniedNotice(PROTECTED_ACTIONS.rewardFraudAudit, "Audit")}
                {serverDeniedNotice(PROTECTED_ACTIONS.revokeRewardFraudBlock, "Revoke")}
                <div className={styles.actionStrip}>
                  {canViewFraud && (
                    <button
                      type="button"
                      className={styles.secondaryButton}
                      onClick={listFraudBlocks}
                      {...actionState(
                        true,
                        true,
                        "Required permission",
                        PROTECTED_ACTIONS.rewardFraudBlocks
                      )}
                    >
                      <ClipboardList size={17} aria-hidden />
                      <span>Load active</span>
                    </button>
                  )}
                  <input
                    aria-label="Fraud block id"
                    {...positiveIntegerInputProps}
                    placeholder="Block id"
                    value={fraudBlockId}
                    onChange={(event) => setFraudBlockId(event.target.value)}
                  />
                  {canViewFraud && (
                    <button
                      type="button"
                      className={styles.secondaryButton}
                      onClick={loadFraudAudit}
                      {...actionState(
                        canUseFraudBlockForm,
                        true,
                        "Required permission",
                        PROTECTED_ACTIONS.rewardFraudAudit
                      )}
                    >
                      <History size={17} aria-hidden />
                      <span>Audit</span>
                    </button>
                  )}
                  {canManageFraud && (
                    <button
                      type="button"
                      className={styles.secondaryButton}
                      onClick={revokeFraudBlock}
                      {...actionState(
                        canUseFraudBlockForm,
                        true,
                        "Required permission",
                        PROTECTED_ACTIONS.revokeRewardFraudBlock
                      )}
                    >
                      <CheckCircle2 size={17} aria-hidden />
                      <span>Revoke</span>
                    </button>
                  )}
                </div>
              </>
            )}
          </section>

          <section
            id="delegation-workflow"
            className={styles.panel}
            aria-labelledby="delegation-title"
          >
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Delegation</p>
                <h2 id="delegation-title">Reward permissions</h2>
              </div>
              <KeyRound size={22} aria-hidden />
            </div>
            {!canDelegate && (
              <PermissionNotice
                title="Delegation permission disabled"
                detail="Enable delegated reward approval permission to grant or revoke delegations."
              />
            )}
            {canDelegate && (
              <>
                {hasSessionToken && hasDelegationGrantDraft && (
                  <RequirementNotice
                    action="Grant delegation"
                    fields={delegationGrantMissingFields}
                  />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.grantDelegatedPermission, "Grant")}
                {serverDeniedNotice(PROTECTED_ACTIONS.delegatedPermissions, "Load")}
                <fieldset className={styles.formGrid}>
                  <input
                    aria-label="Delegation grantee user id"
                    {...positiveIntegerInputProps}
                    placeholder="Grantee user id"
                    value={delegation.grantee_user_id}
                    onChange={(event) =>
                      setDelegation((current) => ({
                        ...current,
                        grantee_user_id: event.target.value,
                      }))
                    }
                  />
                  <select
                    aria-label="Delegated permission"
                    value={activeDelegatedPermission}
                    onChange={(event) =>
                      setDelegation((current) => ({ ...current, permission: event.target.value }))
                    }
                  >
                    {scopedDelegatedPermissions.map((permission) => (
                      <option key={permission} value={permission}>
                        {optionLabel(permission)}
                      </option>
                    ))}
                  </select>
                  <select
                    aria-label="Delegation scope type"
                    value={delegation.scope_type}
                    onChange={(event) =>
                      setDelegation((current) => ({ ...current, scope_type: event.target.value }))
                    }
                  >
                    <option value="platform">{optionLabel("platform")}</option>
                    <option value="organization">{optionLabel("organization")}</option>
                    <option value="course">{optionLabel("course")}</option>
                  </select>
                  {delegation.scope_type === "organization" && (
                    <input
                      aria-label="Delegation organization id"
                      {...positiveIntegerInputProps}
                      placeholder="Organization id"
                      value={delegation.organization_id}
                      onChange={(event) =>
                        setDelegation((current) => ({
                          ...current,
                          organization_id: event.target.value,
                        }))
                      }
                    />
                  )}
                  {delegation.scope_type === "course" && (
                    <input
                      aria-label="Delegation course id"
                      {...positiveIntegerInputProps}
                      placeholder="Course id"
                      value={delegation.course_id}
                      onChange={(event) =>
                        setDelegation((current) => ({ ...current, course_id: event.target.value }))
                      }
                    />
                  )}
                  <label className={styles.fieldLabel}>
                    Expires at
                    <input
                      aria-label="Delegation expiration"
                      type="datetime-local"
                      value={delegation.expires_at}
                      onChange={(event) =>
                        setDelegation((current) => ({
                          ...current,
                          expires_at: event.target.value,
                        }))
                      }
                    />
                  </label>
                  <input
                    aria-label="Delegation reason"
                    className={styles.fullWidth}
                    placeholder="Reason"
                    value={delegation.reason}
                    onChange={(event) =>
                      setDelegation((current) => ({ ...current, reason: event.target.value }))
                    }
                  />
                  <button
                    type="button"
                    className={styles.primaryButton}
                    onClick={grantDelegation}
                    {...actionState(
                      canGrantDelegationForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.grantDelegatedPermission
                    )}
                  >
                    <KeyRound size={17} aria-hidden />
                    <span>Grant</span>
                  </button>
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={listDelegations}
                    {...actionState(
                      true,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.delegatedPermissions
                    )}
                  >
                    <ClipboardList size={17} aria-hidden />
                    <span>Load</span>
                  </button>
                </fieldset>
                {hasSessionToken && hasDelegationRevokeDraft && (
                  <RequirementNotice
                    action="Revoke delegation"
                    fields={delegationRevokeMissingFields}
                  />
                )}
                {serverDeniedNotice(PROTECTED_ACTIONS.revokeDelegatedPermission, "Revoke")}
                <div className={styles.actionStrip}>
                  <input
                    aria-label="Delegation id"
                    {...positiveIntegerInputProps}
                    placeholder="Delegation id"
                    value={delegationId}
                    onChange={(event) => setDelegationId(event.target.value)}
                  />
                  <input
                    aria-label="Delegation revoke reason"
                    placeholder="Revoke reason"
                    value={revokeReason}
                    onChange={(event) => setRevokeReason(event.target.value)}
                  />
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={revokeDelegation}
                    {...actionState(
                      canRevokeDelegationForm,
                      true,
                      "Required permission",
                      PROTECTED_ACTIONS.revokeDelegatedPermission
                    )}
                  >
                    <Ban size={17} aria-hidden />
                    <span>Revoke</span>
                  </button>
                </div>
              </>
            )}
          </section>
        </div>

        <section
          id="result-panel"
          ref={resultPanelRef}
          className={styles.resultPanel}
          aria-labelledby="result-title"
        >
          <p className={styles.visuallyHidden} aria-live="polite" aria-atomic="true">
            {resultAnnouncement}
          </p>
          <div className={styles.panelHeader}>
            <div>
              <p className={styles.eyebrow}>{result.status}</p>
              <h2 id="result-title">{result.label}</h2>
            </div>
            <span className={`${styles.statusPill} ${resultToneClass}`}>
              {resultToneLabel}
            </span>
          </div>
          <pre aria-label="API response body" tabIndex={0}>
            {result.body}
          </pre>
        </section>
      </section>
    </main>
  );
}
