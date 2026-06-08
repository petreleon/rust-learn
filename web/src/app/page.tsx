"use client";

import {
  Ban,
  CheckCircle2,
  ClipboardList,
  Download,
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

function flowStatus(hasAccess: boolean, hasSessionToken: boolean) {
  if (!hasAccess) {
    return "Locked";
  }
  return hasSessionToken ? "Open" : "Needs JWT";
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
    <div className={styles.panelNotice} role="status">
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

  return (
    <div className={styles.requirementNotice} role="status">
      <FileCheck size={16} aria-hidden />
      <div>
        <strong>{action}</strong>
        <span>{formatFieldList(fields)}</span>
      </div>
    </div>
  );
}

export default function Home() {
  const resultPanelRef = useRef<HTMLElement | null>(null);
  const [apiRoot, setApiRoot] = useState(process.env.NEXT_PUBLIC_API_URL || "/api");
  const [token, setToken] = useState("");
  const [credentials, setCredentials] = useState({ email: "", password: "" });
  const [sessionMessage, setSessionMessage] = useState("Not signed in");
  const [healthCheckTick, setHealthCheckTick] = useState(0);
  const [apiState, setApiState] = useState<ApiState>("checking");
  const [apiMessage, setApiMessage] = useState("Checking API");
  const [selectedPermissions, setSelectedPermissions] = useState<Set<string>>(
    () => new Set(DEFAULT_PERMISSION_KEYS)
  );
  const [result, setResult] = useState<ApiResult>({
    label: "Result",
    status: "Idle",
    body: "No request sent.",
    ok: true,
  });

  const [teacherForm, setTeacherForm] = useState({
    requested_scope: "platform",
    requested_organization_id: "",
    requested_course_id: "",
    experience_summary: "",
    organization_sponsor_id: "",
    portfolio_links: "",
  });
  const [teacherStatus, setTeacherStatus] = useState("submitted");
  const [teacherDecision, setTeacherDecision] = useState({
    application_id: "",
    status: "approved",
    decision_reason: "",
  });

  const [rewardCourseId, setRewardCourseId] = useState("");
  const [rewardCandidateId, setRewardCandidateId] = useState("");
  const [rewardStudentId, setRewardStudentId] = useState("");
  const [rewardStatus, setRewardStatus] = useState("pending_teacher_approval");
  const [teacherRewardDecision, setTeacherRewardDecision] = useState({
    status: "approved",
    decision_reason: "",
  });
  const [amountDecision, setAmountDecision] = useState({
    status: "approved",
    approved_amount: "10",
    decision_reason: "",
  });

  const [historyStatus, setHistoryStatus] = useState("");
  const [organizationId, setOrganizationId] = useState("");

  const [fraudBlock, setFraudBlock] = useState({
    scope_type: "teacher",
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
    permission: "APPROVE_REWARD_AMOUNT",
    scope_type: "platform",
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

    fetch(`${healthRoot || ""}/health`, { signal: controller.signal })
      .then(async (response) => {
        const body = await response.text();
        if (!response.ok) {
          throw new Error(`HTTP ${response.status}`);
        }
        setApiState("online");
        setApiMessage(formatHealthMessage(body));
      })
      .catch((error: Error) => {
        if (controller.signal.aborted) {
          return;
        }
        setApiState("offline");
        setApiMessage(error.message || "Connection failed");
      });

    return () => controller.abort();
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
  const canSignIn = hasText(credentials.email) && hasText(credentials.password);
  const actionState = (
    ready = true,
    allowed = true,
    permissionLabel = "Required permission"
  ) => {
    if (!allowed) {
      return { disabled: true, title: permissionLabel };
    }
    if (!hasSessionToken) {
      return { disabled: true, title: "JWT required" };
    }
    if (!ready) {
      return { disabled: true, title: "Complete required fields" };
    }
    return { disabled: false, title: undefined };
  };
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
    if (!canSignIn) {
      setSessionMessage("Email and password required");
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
      const response = await fetch(`${root}/auth/login`, {
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
      setCredentials((current) => ({ ...current, password: "" }));
      setSessionMessage("Signed in");
      setResult({
        label: "Sign in",
        status: `HTTP ${response.status}`,
        body: "JWT loaded into this session.",
        ok: true,
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : "Unknown request failure";
      setSessionMessage(message);
      setResult({
        label: "Sign in",
        status: "Request failed",
        body: message,
        ok: false,
      });
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
    setCredentials((current) => ({ ...current, password: "" }));
    setSessionMessage("Session cleared");
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
      const response = await fetch(`${root}${path}`, {
        method,
        headers,
        body: body === undefined ? undefined : JSON.stringify(body),
      });
      const text = await response.text();
      setResult({
        label,
        status: `HTTP ${response.status}`,
        body: prettyBody(text),
        ok: response.ok,
      });
      revealResultPanel();
    } catch (error) {
      setResult({
        label,
        status: "Request failed",
        body: error instanceof Error ? error.message : "Unknown request failure",
        ok: false,
      });
      revealResultPanel();
    }
  }

  function submitTeacherApplication() {
    const requestedScope = teacherForm.requested_scope;

    void sendApi("Submit teacher application", "/teacher-applications", "POST", {
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
      "Teacher application queue",
      `/teacher-applications${buildQuery({ status: teacherStatus, limit: 25 })}`
    );
  }

  function decideTeacherApplication() {
    void sendApi(
      "Teacher application decision",
      `/teacher-applications/${teacherDecision.application_id}/decision`,
      "PUT",
      {
        status: activeTeacherDecisionStatus,
        decision_reason: teacherDecision.decision_reason || undefined,
      }
    );
  }

  function submitRewardCandidate() {
    void sendApi("Submit reward candidate", `/courses/${rewardCourseId}/reward-candidates`, "POST", {
      student_user_id: optionalPositiveInteger(rewardStudentId),
      event_type: "course_completion",
      evidence: { completion_percentage: 100 },
    });
  }

  function loadRewardCandidates() {
    void sendApi(
      "Course reward candidates",
      `/courses/${rewardCourseId}/reward-candidates${buildQuery({
        status: rewardStatus,
        limit: 25,
      })}`
    );
  }

  function decideStudentReward() {
    void sendApi(
      "Course reward decision",
      `/courses/${rewardCourseId}/reward-candidates/${rewardCandidateId}/teacher-decision`,
      "PUT",
      {
        status: teacherRewardDecision.status,
        decision_reason: teacherRewardDecision.decision_reason || undefined,
      }
    );
  }

  function decideRewardAmount() {
    void sendApi("Reward amount decision", `/reward-candidates/${rewardCandidateId}/amount-decision`, "PUT", {
      status: amountDecision.status,
      approved_amount:
        amountDecision.status === "approved" ? optionalNumber(amountDecision.approved_amount) : undefined,
      decision_reason: amountDecision.decision_reason || undefined,
    });
  }

  function loadStudentHistory() {
    void sendApi(
      "Student reward history",
      `/reward-candidates/me/history${buildQuery({ status: historyStatus, limit: 25 })}`
    );
  }

  function loadOrganizationReport(csv = false) {
    void sendApi(
      csv ? "Organization reward CSV" : "Organization reward report",
      `/reports/organizations/${organizationId}/reward-dashboard${csv ? ".csv" : ""}`
    );
  }

  function loadOrganizationSummary(csv = false) {
    void sendApi(
      csv ? "Organization summary CSV" : "Organization summary",
      `/reports/organizations/${organizationId}/summary${csv ? ".csv" : ""}`
    );
  }

  function loadPlatformExport(path: string, label: string) {
    void sendApi(label, path);
  }

  function createFraudBlock() {
    void sendApi("Create reward fraud block", "/reward-fraud-blocks", "POST", {
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
    void sendApi("Reward fraud blocks", "/reward-fraud-blocks?active=true&limit=25");
  }

  function revokeFraudBlock() {
    void sendApi("Revoke reward fraud block", `/reward-fraud-blocks/${fraudBlockId}/revoke`, "PUT");
  }

  function loadFraudAudit() {
    void sendApi("Reward fraud audit", `/reward-fraud-blocks/${fraudBlockId}/audit`);
  }

  function grantDelegation() {
    void sendApi("Grant delegated permission", "/delegated-permissions", "POST", {
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
    void sendApi("Delegated permissions", "/delegated-permissions?active=true&limit=25");
  }

  function revokeDelegation() {
    void sendApi("Revoke delegated permission", `/delegated-permissions/${delegationId}/revoke`, "PUT", {
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
                title={canSignIn ? undefined : "Email and password required"}
              >
                <KeyRound size={17} aria-hidden />
                <span>Sign in</span>
              </button>
              <button
                type="button"
                className={styles.secondaryButton}
                onClick={clearSession}
                disabled={!hasSessionToken}
                title={hasSessionToken ? undefined : "No active JWT"}
              >
                <Ban size={17} aria-hidden />
                <span>Clear</span>
              </button>
            </div>
          </form>
          <label className={styles.fieldLabel}>
            JWT
            <textarea
              rows={4}
              value={token}
              onChange={(event) => {
                const nextToken = event.target.value;
                setToken(nextToken);
                if (hasText(nextToken)) {
                  setSessionMessage("JWT loaded");
                } else {
                  setSessionMessage("Session cleared");
                  resetSessionResult();
                }
              }}
              spellCheck={false}
            />
          </label>
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
            onClick={() => {
              setApiState("checking");
              setApiMessage("Checking API");
              setHealthCheckTick((current) => current + 1);
            }}
          >
            <RefreshCw size={18} aria-hidden />
            <span>Refresh</span>
          </button>
        </header>

        <section className={styles.metrics} aria-label="Workflow access">
          <div className={styles.metric}>
            <span>Teacher flow</span>
            <strong>{flowStatus(canUseTeacherWorkflow, hasSessionToken)}</strong>
          </div>
          <div className={styles.metric}>
            <span>Reward flow</span>
            <strong>{flowStatus(canUseRewardWorkflow, hasSessionToken)}</strong>
          </div>
          <div className={styles.metric}>
            <span>Audit flow</span>
            <strong>{flowStatus(canUseAuditWorkflow, hasSessionToken)}</strong>
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
          <section className={styles.panel} aria-labelledby="teacher-title">
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
                {hasSessionToken && (
                  <RequirementNotice
                    action="Submit application"
                    fields={teacherApplicationMissingFields}
                  />
                )}
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
                    {...actionState(canSubmitTeacherApplicationForm)}
                  >
                    <Send size={17} aria-hidden />
                    <span>Submit</span>
                  </button>
                </div>
              </>
            )}

            {canReviewTeachers && (
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
                  {...actionState()}
                >
                  <ClipboardList size={17} aria-hidden />
                  <span>Load queue</span>
                </button>
              </div>
            )}

            {canDecideTeachers && (
              <>
                {hasSessionToken && (
                  <RequirementNotice action="Decide" fields={teacherDecisionMissingFields} />
                )}
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
                    {...actionState(canDecideTeacherApplicationForm)}
                  >
                    <CheckCircle2 size={17} aria-hidden />
                    <span>Decide</span>
                  </button>
                </div>
              </>
            )}
          </section>

          <section className={styles.panel} aria-labelledby="reward-title">
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
                {hasSessionToken && (
                  <RequirementNotice
                    action="Submit candidate"
                    fields={submitRewardCandidateMissingFields}
                  />
                )}
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
                    {...actionState(canSubmitRewardCandidateForm)}
                  >
                    <Send size={17} aria-hidden />
                    <span>Submit candidate</span>
                  </button>
                </div>
              </>
            )}

            {canViewCourseRewards && (
              <>
                {hasSessionToken && (
                  <RequirementNotice
                    action="Load candidates"
                    fields={loadRewardCandidatesMissingFields}
                  />
                )}
                <button
                  type="button"
                  className={styles.secondaryButton}
                  onClick={loadRewardCandidates}
                  {...actionState(canLoadRewardCandidatesForm)}
                >
                  <ClipboardList size={17} aria-hidden />
                  <span>Load candidates</span>
                </button>
              </>
            )}

            {canTeacherApproveReward && (
              <>
                {hasSessionToken && (
                  <RequirementNotice
                    action="Teacher decision"
                    fields={teacherRewardDecisionMissingFields}
                  />
                )}
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
                    {...actionState(canDecideStudentRewardForm)}
                  >
                    <CheckCircle2 size={17} aria-hidden />
                    <span>Teacher decision</span>
                  </button>
                </div>
              </>
            )}

            {canApproveAmount && (
              <>
                {hasSessionToken && (
                  <RequirementNotice action="Set amount" fields={amountDecisionMissingFields} />
                )}
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
                    {...actionState(canDecideRewardAmountForm)}
                  >
                    <WalletCards size={17} aria-hidden />
                    <span>Set amount</span>
                  </button>
                </div>
              </>
            )}
          </section>

          <section className={styles.panel} aria-labelledby="history-title">
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
                  {...actionState()}
                >
                  <History size={17} aria-hidden />
                  <span>Load history</span>
                </button>
              </div>
            )}
          </section>

          <section className={styles.panel} aria-labelledby="report-title">
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
            {hasSessionToken && canUseOrganizationReportControls && (
              <RequirementNotice
                action="Load organization reports"
                fields={organizationReportMissingFields}
              />
            )}
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
                    {...actionState(canLoadOrganizationReportForm)}
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
                    {...actionState(canLoadOrganizationReportForm)}
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
                    {...actionState(canLoadOrganizationReportForm)}
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
                    {...actionState(canLoadOrganizationReportForm)}
                  >
                    <Download size={17} aria-hidden />
                    <span>Reward CSV</span>
                  </button>
                )}
              </div>
            )}
            {canExport && (
              <div className={styles.reportLinks}>
                {platformExportReports.map((report) => (
                  <button
                    key={report.path}
                    type="button"
                    onClick={() => loadPlatformExport(report.path, report.resultLabel)}
                    {...actionState()}
                  >
                    <Download size={16} aria-hidden />
                    <span>{report.label}</span>
                  </button>
                ))}
              </div>
            )}
            {canViewSummaryReports && (
              <div className={styles.reportLinks}>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport("/reports/platform/summary", "Platform summary")
                  }
                  {...actionState()}
                >
                  <ClipboardList size={16} aria-hidden />
                  <span>Platform summary</span>
                </button>
              </div>
            )}
            {canViewPlatformDashboards && (
              <div className={styles.reportLinks}>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport("/reports/platform/reward-dashboard", "Platform reward dashboard")
                  }
                  {...actionState()}
                >
                  <ClipboardList size={16} aria-hidden />
                  <span>Reward dashboard</span>
                </button>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport("/reports/platform/fraud-dashboard", "Platform fraud dashboard")
                  }
                  {...actionState()}
                >
                  <ShieldAlert size={16} aria-hidden />
                  <span>Fraud dashboard</span>
                </button>
              </div>
            )}
          </section>

          <section className={styles.panel} aria-labelledby="fraud-title">
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
                {hasSessionToken && (
                  <RequirementNotice
                    action="Create block"
                    fields={fraudBlockCreateMissingFields}
                  />
                )}
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
                      selectedFraudBlockScopePermission.permissionTitle
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
                {hasSessionToken && (
                  <RequirementNotice
                    action={fraudBlockActionLabel}
                    fields={fraudBlockUseMissingFields}
                  />
                )}
                <div className={styles.actionStrip}>
                  {canViewFraud && (
                    <button
                      type="button"
                      className={styles.secondaryButton}
                      onClick={listFraudBlocks}
                      {...actionState()}
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
                      {...actionState(canUseFraudBlockForm)}
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
                      {...actionState(canUseFraudBlockForm)}
                    >
                      <CheckCircle2 size={17} aria-hidden />
                      <span>Revoke</span>
                    </button>
                  )}
                </div>
              </>
            )}
          </section>

          <section className={styles.panel} aria-labelledby="delegation-title">
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
                {hasSessionToken && (
                  <RequirementNotice
                    action="Grant delegation"
                    fields={delegationGrantMissingFields}
                  />
                )}
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
                  <input
                    aria-label="Delegation expiration"
                    type="datetime-local"
                    placeholder="Expires at"
                    value={delegation.expires_at}
                    onChange={(event) =>
                      setDelegation((current) => ({ ...current, expires_at: event.target.value }))
                    }
                  />
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
                    {...actionState(canGrantDelegationForm)}
                  >
                    <KeyRound size={17} aria-hidden />
                    <span>Grant</span>
                  </button>
                  <button
                    type="button"
                    className={styles.secondaryButton}
                    onClick={listDelegations}
                    {...actionState()}
                  >
                    <ClipboardList size={17} aria-hidden />
                    <span>Load</span>
                  </button>
                </fieldset>
                {hasSessionToken && (
                  <RequirementNotice
                    action="Revoke delegation"
                    fields={delegationRevokeMissingFields}
                  />
                )}
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
                    {...actionState(canRevokeDelegationForm)}
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
          ref={resultPanelRef}
          className={styles.resultPanel}
          aria-labelledby="result-title"
          aria-live="polite"
          aria-atomic="false"
        >
          <div className={styles.panelHeader}>
            <div>
              <p className={styles.eyebrow}>{result.status}</p>
              <h2 id="result-title">{result.label}</h2>
            </div>
            <span className={`${styles.statusPill} ${result.ok ? styles.online : styles.offline}`}>
              {result.ok ? "OK" : "Error"}
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
