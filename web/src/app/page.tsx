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
import { useEffect, useMemo, useRef, useState } from "react";
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
  "SUBMIT_COURSE_REWARD_EVENT",
  "VIEW_COURSE_REWARD_STATUS",
  "APPROVE_STUDENT_REWARD_CANDIDATE",
  "APPROVE_REWARD_AMOUNT",
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
  "token_pending",
  "token_confirmed",
  "wallet_credited",
  "completed",
  "needs_reconciliation",
  "failed",
];

const teacherApplicationStatuses = ["submitted", "needs_changes", "approved", "rejected"];
const delegatedPermissions = [
  "APPROVE_REWARD_AMOUNT",
  "VIEW_REWARD_AUDIT",
  "MANAGE_REWARD_FRAUD_BLOCKS",
  "SUBMIT_COURSE_REWARD_EVENT",
  "APPROVE_STUDENT_REWARD_CANDIDATE",
  "VIEW_ORG_REWARD_REPORTS",
];

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

export default function Home() {
  const resultPanelRef = useRef<HTMLElement | null>(null);
  const [apiRoot, setApiRoot] = useState(process.env.NEXT_PUBLIC_API_URL || "/api");
  const [token, setToken] = useState("");
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
  const canDecideTeachers =
    hasPermission("APPROVE_TEACHER_APPLICATION") || hasPermission("REJECT_TEACHER_APPLICATION");
  const canSubmitReward = hasPermission("SUBMIT_COURSE_REWARD_EVENT");
  const canViewCourseRewards = hasPermission("VIEW_COURSE_REWARD_STATUS");
  const canTeacherApproveReward = hasPermission("APPROVE_STUDENT_REWARD_CANDIDATE");
  const canApproveAmount = hasPermission("APPROVE_REWARD_AMOUNT");
  const canViewOrgReports = hasPermission("VIEW_ORG_REWARD_REPORTS");
  const canViewFraud = hasPermission("VIEW_REWARD_AUDIT") || hasPermission("MANAGE_REWARD_FRAUD_BLOCKS");
  const canManageFraud =
    hasPermission("MANAGE_REWARD_FRAUD_BLOCKS") ||
    hasPermission("BLOCK_REWARD_TEACHER") ||
    hasPermission("BLOCK_REWARD_ORGANIZATION");
  const canDelegate = hasPermission("DELEGATE_REWARD_APPROVAL");
  const canExport = hasPermission("EXPORT_DATA");
  const canSubmitTeacherApplicationForm =
    hasText(teacherForm.experience_summary) &&
    (teacherForm.requested_scope === "platform" ||
      (teacherForm.requested_scope === "organization" &&
        (hasPositiveInteger(teacherForm.requested_organization_id) ||
          hasPositiveInteger(teacherForm.organization_sponsor_id))) ||
      (teacherForm.requested_scope === "course" &&
        hasPositiveInteger(teacherForm.requested_course_id)));
  const canDecideTeacherApplicationForm = hasPositiveInteger(teacherDecision.application_id);
  const canSubmitRewardCandidateForm =
    hasPositiveInteger(rewardCourseId) && hasPositiveInteger(rewardStudentId);
  const canLoadRewardCandidatesForm = hasPositiveInteger(rewardCourseId);
  const canDecideStudentRewardForm =
    hasPositiveInteger(rewardCourseId) && hasPositiveInteger(rewardCandidateId);
  const canDecideRewardAmountForm =
    hasPositiveInteger(rewardCandidateId) &&
    (amountDecision.status !== "approved" || hasNonNegativeNumber(amountDecision.approved_amount));
  const canLoadOrganizationReportForm = hasPositiveInteger(organizationId);
  const canCreateFraudBlockTarget =
    (fraudBlock.scope_type === "teacher" && hasPositiveInteger(fraudBlock.teacher_user_id)) ||
    (fraudBlock.scope_type === "organization" && hasPositiveInteger(fraudBlock.organization_id)) ||
    (fraudBlock.scope_type === "course" && hasPositiveInteger(fraudBlock.course_id)) ||
    (fraudBlock.scope_type === "reward_policy" && hasPositiveInteger(fraudBlock.reward_policy_id));
  const canCreateFraudBlockForm = canCreateFraudBlockTarget && hasText(fraudBlock.reason);
  const canUseFraudBlockForm = hasPositiveInteger(fraudBlockId);
  const canGrantDelegationForm =
    hasPositiveInteger(delegation.grantee_user_id) &&
    (delegation.scope_type === "platform" ||
      (delegation.scope_type === "organization" &&
        hasPositiveInteger(delegation.organization_id)) ||
      (delegation.scope_type === "course" && hasPositiveInteger(delegation.course_id)));
  const canRevokeDelegationForm = hasPositiveInteger(delegationId);

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
    void sendApi("Submit teacher application", "/teacher-applications", "POST", {
      requested_scope: teacherForm.requested_scope,
      requested_organization_id: optionalNumber(teacherForm.requested_organization_id),
      requested_course_id: optionalNumber(teacherForm.requested_course_id),
      experience_summary: teacherForm.experience_summary,
      organization_sponsor_id: optionalNumber(teacherForm.organization_sponsor_id),
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
        status: teacherDecision.status,
        decision_reason: teacherDecision.decision_reason || undefined,
      }
    );
  }

  function submitRewardCandidate() {
    void sendApi("Submit reward candidate", `/courses/${rewardCourseId}/reward-candidates`, "POST", {
      student_user_id: optionalNumber(rewardStudentId),
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

  function loadPlatformExport(path: string, label: string) {
    void sendApi(label, path);
  }

  function createFraudBlock() {
    void sendApi("Create reward fraud block", "/reward-fraud-blocks", "POST", {
      scope_type: fraudBlock.scope_type,
      teacher_user_id:
        fraudBlock.scope_type === "teacher" ? optionalNumber(fraudBlock.teacher_user_id) : undefined,
      organization_id:
        fraudBlock.scope_type === "organization"
          ? optionalNumber(fraudBlock.organization_id)
          : undefined,
      course_id: fraudBlock.scope_type === "course" ? optionalNumber(fraudBlock.course_id) : undefined,
      reward_policy_id:
        fraudBlock.scope_type === "reward_policy"
          ? optionalNumber(fraudBlock.reward_policy_id)
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
      grantee_user_id: optionalNumber(delegation.grantee_user_id),
      permission: delegation.permission,
      scope_type: delegation.scope_type,
      organization_id:
        delegation.scope_type === "organization" ? optionalNumber(delegation.organization_id) : undefined,
      course_id: delegation.scope_type === "course" ? optionalNumber(delegation.course_id) : undefined,
      reason: delegation.reason || undefined,
      expires_at: delegation.expires_at || undefined,
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

        <section className={styles.sidebarSection} aria-labelledby="api-session-title">
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
          <label className={styles.fieldLabel}>
            JWT
            <textarea
              rows={4}
              value={token}
              onChange={(event) => setToken(event.target.value)}
              spellCheck={false}
            />
          </label>
          <p className={styles.statusMessage} aria-live="polite">
            {apiMessage}
          </p>
        </section>

        <section className={styles.sidebarSection} aria-labelledby="permissions-title">
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
            <strong>{flowStatus(canTeacherApply || canReviewTeachers, hasSessionToken)}</strong>
          </div>
          <div className={styles.metric}>
            <span>Reward flow</span>
            <strong>
              {flowStatus(canSubmitReward || canTeacherApproveReward || canApproveAmount, hasSessionToken)}
            </strong>
          </div>
          <div className={styles.metric}>
            <span>Audit flow</span>
            <strong>{flowStatus(canViewFraud || canDelegate || canExport, hasSessionToken)}</strong>
          </div>
        </section>

        <div className={styles.grid}>
          <section className={styles.panel} aria-labelledby="teacher-title">
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Teacher applications</p>
                <h2 id="teacher-title">Application and review</h2>
              </div>
              <GraduationCap size={22} aria-hidden />
            </div>

            {canTeacherApply && (
              <div className={styles.formGrid}>
                <label className={styles.fieldLabel}>
                  Scope
                  <select
                    value={teacherForm.requested_scope}
                    onChange={(event) =>
                      setTeacherForm((current) => ({ ...current, requested_scope: event.target.value }))
                    }
                  >
                    <option value="platform">{optionLabel("platform")}</option>
                    <option value="organization">{optionLabel("organization")}</option>
                    <option value="course">{optionLabel("course")}</option>
                  </select>
                </label>
                <label className={styles.fieldLabel}>
                  Organization id
                  <input
                    value={teacherForm.requested_organization_id}
                    onChange={(event) =>
                      setTeacherForm((current) => ({
                        ...current,
                        requested_organization_id: event.target.value,
                      }))
                    }
                  />
                </label>
                <label className={styles.fieldLabel}>
                  Course id
                  <input
                    value={teacherForm.requested_course_id}
                    onChange={(event) =>
                      setTeacherForm((current) => ({
                        ...current,
                        requested_course_id: event.target.value,
                      }))
                    }
                  />
                </label>
                <label className={styles.fieldLabel}>
                  Sponsor org id
                  <input
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
              <div className={styles.actionStrip}>
                <input
                  aria-label="Teacher application id"
                  placeholder="Application id"
                  value={teacherDecision.application_id}
                  onChange={(event) =>
                    setTeacherDecision((current) => ({ ...current, application_id: event.target.value }))
                  }
                />
                <select
                  aria-label="Teacher decision status"
                  value={teacherDecision.status}
                  onChange={(event) =>
                    setTeacherDecision((current) => ({ ...current, status: event.target.value }))
                  }
                >
                  <option value="approved">{optionLabel("approved")}</option>
                  <option value="rejected">{optionLabel("rejected")}</option>
                  <option value="needs_changes">{optionLabel("needs_changes")}</option>
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

            <div className={styles.actionStrip}>
              <input
                aria-label="Reward course id"
                placeholder="Course id"
                value={rewardCourseId}
                onChange={(event) => setRewardCourseId(event.target.value)}
              />
              <input
                aria-label="Reward candidate id"
                placeholder="Candidate id"
                value={rewardCandidateId}
                onChange={(event) => setRewardCandidateId(event.target.value)}
              />
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
            </div>

            {canSubmitReward && (
              <div className={styles.actionStrip}>
                <input
                  aria-label="Reward student user id"
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
            )}

            {canViewCourseRewards && (
              <button
                type="button"
                className={styles.secondaryButton}
                onClick={loadRewardCandidates}
                {...actionState(canLoadRewardCandidatesForm)}
              >
                <ClipboardList size={17} aria-hidden />
                <span>Load candidates</span>
              </button>
            )}

            {canTeacherApproveReward && (
              <div className={styles.actionStrip}>
                <select
                  aria-label="Teacher reward decision status"
                  value={teacherRewardDecision.status}
                  onChange={(event) =>
                    setTeacherRewardDecision((current) => ({ ...current, status: event.target.value }))
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
            )}

            {canApproveAmount && (
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
                <input
                  aria-label="Approved reward amount"
                  placeholder="Amount"
                  value={amountDecision.approved_amount}
                  onChange={(event) =>
                    setAmountDecision((current) => ({
                      ...current,
                      approved_amount: event.target.value,
                    }))
                  }
                />
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
                {...actionState(true, canViewCourseRewards, "View course rewards permission required")}
              >
                <History size={17} aria-hidden />
                <span>Load history</span>
              </button>
            </div>
          </section>

          <section className={styles.panel} aria-labelledby="report-title">
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Organization</p>
                <h2 id="report-title">Reward report</h2>
              </div>
              <Download size={22} aria-hidden />
            </div>
            <div className={styles.actionStrip}>
              <input
                aria-label="Report organization id"
                placeholder="Organization id"
                value={organizationId}
                onChange={(event) => setOrganizationId(event.target.value)}
              />
              <button
                type="button"
                className={styles.secondaryButton}
                onClick={() => loadOrganizationReport(false)}
                {...actionState(
                  canLoadOrganizationReportForm,
                  canViewOrgReports,
                  "View organization rewards permission required"
                )}
              >
                <ClipboardList size={17} aria-hidden />
                <span>Load</span>
              </button>
              <button
                type="button"
                className={styles.secondaryButton}
                onClick={() => loadOrganizationReport(true)}
                {...actionState(
                  canLoadOrganizationReportForm,
                  canViewOrgReports,
                  "View organization rewards permission required"
                )}
              >
                <Download size={17} aria-hidden />
                <span>CSV</span>
              </button>
            </div>
            {canExport && (
              <div className={styles.reportLinks}>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport(
                      "/reports/platform/reward-approvals.csv",
                      "Platform reward approvals CSV"
                    )
                  }
                  {...actionState()}
                >
                  reward approvals
                </button>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport("/reports/platform/token-payouts.csv", "Platform token payouts CSV")
                  }
                  {...actionState()}
                >
                  token payouts
                </button>
                <button
                  type="button"
                  onClick={() =>
                    loadPlatformExport(
                      "/reports/platform/delegated-permissions.csv",
                      "Platform delegated permissions CSV"
                    )
                  }
                  {...actionState()}
                >
                  delegations
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
            {canManageFraud && (
              <div className={styles.formGrid}>
                <label className={styles.fieldLabel}>
                  Scope
                  <select
                    value={fraudBlock.scope_type}
                    onChange={(event) =>
                      setFraudBlock((current) => ({ ...current, scope_type: event.target.value }))
                    }
                  >
                    <option value="teacher">{optionLabel("teacher")}</option>
                    <option value="organization">{optionLabel("organization")}</option>
                    <option value="course">{optionLabel("course")}</option>
                    <option value="reward_policy">{optionLabel("reward_policy")}</option>
                  </select>
                </label>
                <input
                  aria-label="Fraud block teacher user id"
                  placeholder="Teacher user id"
                  value={fraudBlock.teacher_user_id}
                  onChange={(event) =>
                    setFraudBlock((current) => ({ ...current, teacher_user_id: event.target.value }))
                  }
                />
                <input
                  aria-label="Fraud block organization id"
                  placeholder="Organization id"
                  value={fraudBlock.organization_id}
                  onChange={(event) =>
                    setFraudBlock((current) => ({ ...current, organization_id: event.target.value }))
                  }
                />
                <input
                  aria-label="Fraud block course id"
                  placeholder="Course id"
                  value={fraudBlock.course_id}
                  onChange={(event) =>
                    setFraudBlock((current) => ({ ...current, course_id: event.target.value }))
                  }
                />
                <input
                  aria-label="Fraud block policy id"
                  placeholder="Policy id"
                  value={fraudBlock.reward_policy_id}
                  onChange={(event) =>
                    setFraudBlock((current) => ({
                      ...current,
                      reward_policy_id: event.target.value,
                    }))
                  }
                />
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
                  {...actionState(canCreateFraudBlockForm)}
                >
                  <Ban size={17} aria-hidden />
                  <span>Create block</span>
                </button>
              </div>
            )}
            {canViewFraud && (
              <div className={styles.actionStrip}>
                <button
                  type="button"
                  className={styles.secondaryButton}
                  onClick={listFraudBlocks}
                  {...actionState()}
                >
                  <ClipboardList size={17} aria-hidden />
                  <span>Load active</span>
                </button>
                <input
                  aria-label="Fraud block id"
                  placeholder="Block id"
                  value={fraudBlockId}
                  onChange={(event) => setFraudBlockId(event.target.value)}
                />
                <button
                  type="button"
                  className={styles.secondaryButton}
                  onClick={loadFraudAudit}
                  {...actionState(canUseFraudBlockForm)}
                >
                  <History size={17} aria-hidden />
                  <span>Audit</span>
                </button>
                <button
                  type="button"
                  className={styles.secondaryButton}
                  onClick={revokeFraudBlock}
                  {...actionState(
                    canUseFraudBlockForm,
                    canManageFraud,
                    "Manage fraud blocks permission required"
                  )}
                >
                  <CheckCircle2 size={17} aria-hidden />
                  <span>Revoke</span>
                </button>
              </div>
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
            <fieldset className={styles.formGrid} disabled={!canDelegate}>
              <input
                aria-label="Delegation grantee user id"
                placeholder="Grantee user id"
                value={delegation.grantee_user_id}
                onChange={(event) =>
                  setDelegation((current) => ({ ...current, grantee_user_id: event.target.value }))
                }
              />
              <select
                aria-label="Delegated permission"
                value={delegation.permission}
                onChange={(event) =>
                  setDelegation((current) => ({ ...current, permission: event.target.value }))
                }
              >
                {delegatedPermissions.map((permission) => (
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
              <input
                aria-label="Delegation organization id"
                placeholder="Organization id"
                value={delegation.organization_id}
                onChange={(event) =>
                  setDelegation((current) => ({ ...current, organization_id: event.target.value }))
                }
              />
              <input
                aria-label="Delegation course id"
                placeholder="Course id"
                value={delegation.course_id}
                onChange={(event) =>
                  setDelegation((current) => ({ ...current, course_id: event.target.value }))
                }
              />
              <input
                aria-label="Delegation expiration"
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
            <div className={styles.actionStrip}>
              <input
                aria-label="Delegation id"
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
                  canDelegate,
                  "Delegation permission required"
                )}
              >
                <Ban size={17} aria-hidden />
                <span>Revoke</span>
              </button>
            </div>
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
