import type { CurrentSession } from "@/lib/session";

export type PlatformCapabilityKey =
  | "summary"
  | "teacher_applications"
  | "reward_amount_review"
  | "fraud_blocks"
  | "delegations"
  | "exports"
  | "wallets"
  | "system";

export type PlatformCapability = {
  enabled: boolean;
  key: PlatformCapabilityKey;
  label: string;
  permissions: string[];
};

export type PlatformAdminWorkspace = {
  capabilities: PlatformCapability[];
  delegatedPermissionCount: number;
  directPermissionCount: number;
  effectivePermissionCount: number;
  effectivePermissions: string[];
  roles: string[];
};

export type PlatformReportSummary = {
  total_courses: number;
  total_notifications: number;
  total_organizations: number;
  total_users: number;
  total_wallets: number;
};

export type TeacherApplicationDashboardSummary = {
  approved: number;
  needs_changes: number;
  rejected: number;
  submitted: number;
  total: number;
};

export type RewardCandidateDashboardSummary = {
  amount_approved: number;
  amount_rejected: number;
  completed: number;
  failed: number;
  needs_reconciliation: number;
  notified: number;
  pending_teacher_approval: number;
  teacher_approved: number;
  teacher_rejected: number;
  token_confirmed: number;
  token_pending: number;
  total: number;
  wallet_credited: number;
};

export type RewardCandidateDashboardRow = {
  approved_amount: string | null;
  course_id: number;
  event_type: string;
  reward_candidate_id: number;
  source_organization_id: number | null;
  status: string;
  student_user_id: number;
  submitter_user_id: number;
  updated_at: string;
};

export type RewardExecutionFailureRow = {
  attempts: number;
  last_error: string | null;
  reward_candidate_id: number;
  reward_execution_job_id: number;
  status: string;
  updated_at: string;
};

export type RewardReconciliationMismatchRow = {
  approved_amount: string | null;
  course_id: number;
  mismatch_type: string;
  reward_candidate_id: number;
  status: string;
  student_user_id: number;
  updated_at: string;
};

export type PlatformRewardDashboard = {
  payout_failure_count: number;
  payout_failures: RewardExecutionFailureRow[];
  pending_amount_approval_count: number;
  pending_amount_approvals: RewardCandidateDashboardRow[];
  reconciliation_mismatch_count: number;
  reconciliation_mismatches: RewardReconciliationMismatchRow[];
  reward_candidates: RewardCandidateDashboardSummary;
  teacher_applications: TeacherApplicationDashboardSummary;
};

export type FraudBlockScopeSummary = {
  course: number;
  organization: number;
  reward_policy: number;
  teacher: number;
};

export type FraudBlockDashboardRow = {
  course_id: number | null;
  created_at: string;
  created_by_user_id: number;
  evidence_reference: string | null;
  expires_at: string | null;
  id: number;
  organization_id: number | null;
  reason: string;
  reward_policy_id: number | null;
  scope_type: string;
  teacher_user_id: number | null;
  updated_at: string;
};

export type PlatformFraudDashboard = {
  active_blocks: FraudBlockDashboardRow[];
  active_by_scope: FraudBlockScopeSummary;
  active_total: number;
};

export type FraudBlockStatus = "active" | "revoked" | "expired";

export type FraudBlockItem = {
  course_id: number | null;
  created_at: string;
  created_by_user_id: number;
  evidence_reference: string | null;
  expires_at: string | null;
  id: number;
  organization_id: number | null;
  reason: string;
  reward_policy_id: number | null;
  revoked_at: string | null;
  revoked_by_user_id: number | null;
  scope_type: string;
  teacher_user_id: number | null;
  updated_at: string;
};

export type FraudBlockAuditEvent = {
  actor_user_id: number | null;
  created_at: string;
  event_type: string;
  fraud_block_id: number;
  id: number;
  reason: string | null;
};

export type DelegationItem = {
  course_id: number | null;
  created_at: string;
  expires_at: string | null;
  grantee_user_id: number;
  grantor_user_id: number;
  id: number;
  organization_id: number | null;
  permission: string;
  reason: string | null;
  revoked_at: string | null;
  revoked_by_user_id: number | null;
  revoke_reason: string | null;
  scope_type: string;
  updated_at: string;
};

export type DelegationListOptions = AdminRequestOptions & {
  active?: boolean | null;
  course_id?: number | null;
  grantee_user_id?: number | null;
  grantor_user_id?: number | null;
  limit?: number;
  offset?: number;
  organization_id?: number | null;
  permission?: string | null;
  scope_type?: string | null;
};

export type DelegationCreateOptions = AdminRequestOptions & {
  course_id?: number | null;
  expires_at?: string | null;
  grantee_user_id: number;
  organization_id?: number | null;
  permission: string;
  reason?: string | null;
  scope_type: string;
};

export type DelegationStatus = "active" | "expired" | "revoked";

export type FraudBlockListOptions = AdminRequestOptions & {
  scope_type?: string | null;
  active?: boolean | null;
  limit?: number;
  offset?: number;
};

export type FraudBlockCreateOptions = AdminRequestOptions & {
  scope_type: string;
  teacher_user_id?: number | null;
  organization_id?: number | null;
  course_id?: number | null;
  reward_policy_id?: number | null;
  reason: string;
  evidence_reference?: string | null;
  expires_at?: string | null;
};

export type FraudBlockRevokeOptions = AdminRequestOptions & {
  blockId: number;
};

export type TeacherApplicationStatus = "submitted" | "needs_changes" | "approved" | "rejected";

export type TeacherApplicationScope = "platform" | "organization" | "course";

export type TeacherApplication = {
  applicant_user_id: number;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  experience_summary: string;
  id: number;
  idempotency_key: string | null;
  organization_sponsor_id: number | null;
  portfolio_links: string[];
  requested_course_id: number | null;
  requested_organization_id: number | null;
  requested_scope: TeacherApplicationScope;
  reviewer_id: number | null;
  status: TeacherApplicationStatus;
  updated_at: string;
};

export type TeacherApplicationAuditEvent = {
  actor_user_id: number | null;
  application_id: number;
  created_at: string;
  event_type: string;
  from_status: string | null;
  id: number;
  reason: string | null;
  to_status: TeacherApplicationStatus;
};

export type PlatformTeacherApplicationUser = {
  email: string;
  id: number;
  name: string;
};

export type PlatformTeacherApplicationOrganization = {
  id: number;
  name: string;
};

export type PlatformTeacherApplicationCourse = {
  id: number;
  title: string;
};

export type PlatformTeacherApplicationAuditSummary = {
  event_count: number;
  latest_event_at: string | null;
  latest_event_type: string | null;
  latest_reason: string | null;
};

export type PlatformTeacherApplicationItem = {
  applicant: PlatformTeacherApplicationUser;
  audit: PlatformTeacherApplicationAuditSummary;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  experience_summary: string;
  id: number;
  portfolio_links: string[];
  requested_course: PlatformTeacherApplicationCourse | null;
  requested_organization: PlatformTeacherApplicationOrganization | null;
  requested_scope: TeacherApplicationScope;
  reviewer: PlatformTeacherApplicationUser | null;
  sponsor_organization: PlatformTeacherApplicationOrganization | null;
  status: TeacherApplicationStatus;
  updated_at: string;
};

export type PlatformTeacherApplicationPermissions = {
  can_approve_applications: boolean;
  can_reject_applications: boolean;
  can_request_changes: boolean;
  can_view_applications: boolean;
};

export type PlatformTeacherApplicationsResponse = {
  applications: PlatformTeacherApplicationItem[];
  limit: number;
  offset: number;
  operator_permissions: PlatformTeacherApplicationPermissions;
  search: string | null;
  status: TeacherApplicationStatus | null;
  summary: TeacherApplicationDashboardSummary;
  total: number;
};

export type PlatformRewardCandidateUser = {
  email: string;
  id: number;
  name: string;
};

export type PlatformRewardCandidateCourse = {
  id: number;
  title: string;
};

export type PlatformRewardCandidateItem = {
  approved_amount: string | null;
  course: PlatformRewardCandidateCourse;
  created_at: string;
  event_type: string;
  id: number;
  source_organization_id: number | null;
  source_scope: string;
  status: string;
  student: PlatformRewardCandidateUser;
  submitter: PlatformRewardCandidateUser;
  teacher_approver: PlatformRewardCandidateUser | null;
  teacher_decision_reason: string | null;
  updated_at: string;
};

export type PlatformRewardCandidatePermissions = {
  can_approve_amount: boolean;
  can_view_candidates: boolean;
};

export type PlatformRewardCandidatesResponse = {
  candidates: PlatformRewardCandidateItem[];
  limit: number;
  offset: number;
  operator_permissions: PlatformRewardCandidatePermissions;
  search: string | null;
  status: string | null;
  total: number;
};

export type RewardAuditEvent = {
  actor_user_id: number | null;
  created_at: string;
  event_type: string;
  from_status: string | null;
  id: number;
  metadata: Record<string, unknown>;
  reason: string | null;
  reward_candidate_id: number;
  to_status: string;
};

export type RewardCandidateAmountDecisionStatus = "approved" | "rejected";

export type SystemLiveness = {
  status: string;
};

export type SystemDependencyCheck = {
  message: string | null;
  name: string;
  status: string;
};

export type SystemReadiness = {
  checks: SystemDependencyCheck[];
  status: string;
};

export type PlatformSystemStatus = {
  liveness: SystemLiveness;
  readiness: SystemReadiness;
};

export type PlatformCsvReport =
  | "summary"
  | "reward_dashboard"
  | "fraud_dashboard"
  | "teacher_applications"
  | "reward_approvals"
  | "token_payouts"
  | "wallet_credits"
  | "delegated_permissions";

export type PlatformCsvDownload = {
  body: string;
  filename: string;
};

export class AdminRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "AdminRequestError";
    this.status = status;
    this.code = code;
  }
}

type AdminErrorEnvelope = {
  error?: {
    code?: string;
    message?: string;
  };
};

type AdminRequestOptions = {
  apiRoot?: string;
  timeoutMs?: number;
  token?: string;
};

export type PlatformTeacherApplicationListOptions = AdminRequestOptions & {
  limit?: number;
  offset?: number;
  search?: string | null;
  status?: TeacherApplicationStatus | "" | null;
};

export type TeacherApplicationDecisionOptions = AdminRequestOptions & {
  applicationId: number;
  decisionReason?: string | null;
  status: Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">;
};

const DEFAULT_TIMEOUT_MS = 10000;

export const platformCapabilityDefinitions: Array<{
  key: PlatformCapabilityKey;
  label: string;
  permissions: string[];
}> = [
  {
    key: "summary",
    label: "Platform summary",
    permissions: ["VIEW_REPORT"],
  },
  {
    key: "teacher_applications",
    label: "Teacher review",
    permissions: ["REVIEW_TEACHER_APPLICATIONS", "APPROVE_TEACHER_APPLICATION", "REJECT_TEACHER_APPLICATION"],
  },
  {
    key: "reward_amount_review",
    label: "Reward amount review",
    permissions: ["APPROVE_REWARD_AMOUNT", "VIEW_REWARD_AUDIT"],
  },
  {
    key: "fraud_blocks",
    label: "Fraud controls",
    permissions: ["MANAGE_REWARD_FRAUD_BLOCKS", "BLOCK_REWARD_TEACHER", "BLOCK_REWARD_ORGANIZATION"],
  },
  {
    key: "delegations",
    label: "Delegations",
    permissions: ["MANAGE_ROLE_PERMISSIONS", "VIEW_ROLE_ASSIGNMENTS"],
  },
  {
    key: "exports",
    label: "Exports",
    permissions: ["EXPORT_DATA"],
  },
  {
    key: "wallets",
    label: "Wallet audit",
    permissions: ["MANAGE_WALLETS", "VIEW_TRANSACTIONS", "VIEW_SENSITIVE_TRANSACTIONS"],
  },
  {
    key: "system",
    label: "System status",
    permissions: ["VIEW_REPORT", "VIEW_AUDIT_LOGS", "VIEW_ANALYTICS_DASHBOARD", "MANAGE_TEST_SUITES"],
  },
];

const platformCsvEndpoints: Record<PlatformCsvReport, { filename: string; path: string }> = {
  delegated_permissions: {
    filename: "platform-delegated-permissions.csv",
    path: "/reports/platform/delegated-permissions.csv",
  },
  fraud_dashboard: {
    filename: "platform-fraud-dashboard.csv",
    path: "/reports/platform/fraud-dashboard.csv",
  },
  reward_approvals: {
    filename: "platform-reward-approvals.csv",
    path: "/reports/platform/reward-approvals.csv",
  },
  reward_dashboard: {
    filename: "platform-reward-dashboard.csv",
    path: "/reports/platform/reward-dashboard.csv",
  },
  summary: {
    filename: "platform-summary.csv",
    path: "/reports/platform/summary.csv",
  },
  teacher_applications: {
    filename: "platform-teacher-applications.csv",
    path: "/reports/platform/teacher-applications.csv",
  },
  token_payouts: {
    filename: "platform-token-payouts.csv",
    path: "/reports/platform/token-payouts.csv",
  },
  wallet_credits: {
    filename: "platform-wallet-credits.csv",
    path: "/reports/platform/wallet-credits.csv",
  },
};

export function buildPlatformAdminWorkspace(session: CurrentSession): PlatformAdminWorkspace {
  return {
    capabilities: platformCapabilityDefinitions.map((capability) => ({
      enabled: capability.permissions.some((permission) => session.platform.effective_permissions.includes(permission)),
      key: capability.key,
      label: capability.label,
      permissions: capability.permissions,
    })),
    delegatedPermissionCount: session.platform.delegated_permissions.length,
    directPermissionCount: session.platform.direct_permissions.length,
    effectivePermissionCount: session.platform.effective_permissions.length,
    effectivePermissions: [...session.platform.effective_permissions],
    roles: [...session.platform.roles],
  };
}

export function platformCapabilityEnabled(workspace: PlatformAdminWorkspace, key: PlatformCapabilityKey) {
  return workspace.capabilities.some((capability) => capability.key === key && capability.enabled);
}

export function missingPlatformPermissions(capability: PlatformCapability) {
  return capability.permissions;
}

export async function fetchPlatformSummary({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions): Promise<PlatformReportSummary> {
  return adminJsonRequest({
    apiRoot,
    path: "/reports/platform/summary",
    timeoutMs,
    token,
  });
}

export async function fetchPlatformRewardDashboard({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions): Promise<PlatformRewardDashboard> {
  return adminJsonRequest({
    apiRoot,
    path: "/reports/platform/reward-dashboard",
    timeoutMs,
    token,
  });
}

export async function fetchPlatformFraudDashboard({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions): Promise<PlatformFraudDashboard> {
  return adminJsonRequest({
    apiRoot,
    path: "/reports/platform/fraud-dashboard",
    timeoutMs,
    token,
  });
}

export async function fetchDelegations({
  apiRoot = "/api",
  active,
  course_id,
  grantee_user_id,
  grantor_user_id,
  limit,
  offset,
  organization_id,
  permission,
  scope_type,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: DelegationListOptions): Promise<{
  delegations: DelegationItem[];
  limit: number;
  offset: number;
  total: number;
}> {
  const query = new URLSearchParams();
  if (typeof active === "boolean") {
    query.set("active", String(active));
  }
  if (typeof course_id === "number") {
    query.set("course_id", String(course_id));
  }
  if (typeof grantee_user_id === "number") {
    query.set("grantee_user_id", String(grantee_user_id));
  }
  if (typeof grantor_user_id === "number") {
    query.set("grantor_user_id", String(grantor_user_id));
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  if (typeof organization_id === "number") {
    query.set("organization_id", String(organization_id));
  }
  if (permission) {
    query.set("permission", permission);
  }
  if (scope_type) {
    query.set("scope_type", scope_type);
  }
  const suffix = query.toString();
  return adminJsonRequest({
    apiRoot,
    path: `/delegated-permissions${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}

export async function createDelegation({
  apiRoot = "/api",
  course_id,
  expires_at,
  grantee_user_id,
  organization_id,
  permission,
  reason,
  scope_type,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: DelegationCreateOptions): Promise<DelegationItem> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({
      course_id: course_id ?? null,
      expires_at: expires_at ?? null,
      grantee_user_id,
      organization_id: organization_id ?? null,
      permission,
      reason: reason?.trim() || null,
      scope_type,
    }),
    method: "POST",
    path: "/delegated-permissions",
    timeoutMs,
    token,
  });
}

export async function revokeDelegation({
  apiRoot = "/api",
  delegationId,
  revokeReason,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { delegationId: number; revokeReason?: string | null }): Promise<DelegationItem> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({ revoke_reason: revokeReason?.trim() || null }),
    method: "PUT",
    path: `/delegated-permissions/${delegationId}/revoke`,
    timeoutMs,
    token,
  });
}

export async function fetchFraudBlocks({
  apiRoot = "/api",
  scope_type,
  active,
  limit,
  offset,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FraudBlockListOptions): Promise<{ blocks: FraudBlockItem[]; limit: number; offset: number; total: number }> {
  const query = new URLSearchParams();
  if (scope_type) {
    query.set("scope_type", scope_type);
  }
  if (typeof active === "boolean") {
    query.set("active", String(active));
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  const suffix = query.toString();
  return adminJsonRequest({
    apiRoot,
    path: `/reward-fraud-blocks${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}

export async function createFraudBlock({
  apiRoot = "/api",
  course_id,
  evidence_reference,
  expires_at,
  organization_id,
  reason,
  reward_policy_id,
  scope_type,
  teacher_user_id,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FraudBlockCreateOptions): Promise<FraudBlockItem> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({
      course_id: course_id ?? null,
      evidence_reference: evidence_reference?.trim() || null,
      expires_at: expires_at ?? null,
      organization_id: organization_id ?? null,
      reason: reason.trim(),
      reward_policy_id: reward_policy_id ?? null,
      scope_type,
      teacher_user_id: teacher_user_id ?? null,
    }),
    method: "POST",
    path: "/reward-fraud-blocks",
    timeoutMs,
    token,
  });
}

export async function revokeFraudBlock({
  apiRoot = "/api",
  blockId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FraudBlockRevokeOptions): Promise<FraudBlockItem> {
  return adminJsonRequest({
    apiRoot,
    method: "PUT",
    path: `/reward-fraud-blocks/${blockId}/revoke`,
    timeoutMs,
    token,
  });
}

export async function fetchFraudBlockAudit({
  apiRoot = "/api",
  blockId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { blockId: number }): Promise<FraudBlockAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/reward-fraud-blocks/${blockId}/audit`,
    timeoutMs,
    token,
  });
}

export async function fetchPlatformTeacherApplications({
  apiRoot = "/api",
  limit,
  offset,
  search,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: PlatformTeacherApplicationListOptions): Promise<PlatformTeacherApplicationsResponse> {
  const query = new URLSearchParams();
  const normalizedSearch = search?.trim();
  if (normalizedSearch) {
    query.set("search", normalizedSearch);
  }
  if (status) {
    query.set("status", status);
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }

  const suffix = query.toString();
  return adminJsonRequest({
    apiRoot,
    path: `/teacher-applications/review${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}

export async function fetchTeacherApplicationAudit({
  apiRoot = "/api",
  applicationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { applicationId: number }): Promise<TeacherApplicationAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/teacher-applications/${applicationId}/audit`,
    timeoutMs,
    token,
  });
}

export async function decideTeacherApplication({
  apiRoot = "/api",
  applicationId,
  decisionReason,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherApplicationDecisionOptions): Promise<TeacherApplication> {
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify({
      decision_reason: decisionReason?.trim() || null,
      status,
    }),
    method: "PUT",
    path: `/teacher-applications/${applicationId}/decision`,
    timeoutMs,
    token,
  });
}

export async function fetchPlatformRewardCandidates({
  apiRoot = "/api",
  limit,
  offset,
  search,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & {
  limit?: number;
  offset?: number;
  search?: string | null;
  status?: string | null;
}): Promise<PlatformRewardCandidatesResponse> {
  const query = new URLSearchParams();
  const normalizedSearch = search?.trim();
  if (normalizedSearch) {
    query.set("search", normalizedSearch);
  }
  if (status) {
    query.set("status", status);
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }

  const suffix = query.toString();
  return adminJsonRequest({
    apiRoot,
    path: `/reward-candidates/review${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}

export async function decideRewardAmount({
  apiRoot = "/api",
  candidateId,
  status,
  approvedAmount,
  decisionReason,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & {
  candidateId: number;
  status: RewardCandidateAmountDecisionStatus;
  approvedAmount?: string | null;
  decisionReason?: string | null;
}): Promise<PlatformRewardCandidateItem> {
  const body: Record<string, unknown> = {
    status,
    decision_reason: decisionReason?.trim() || null,
  };
  if (status === "approved" && approvedAmount != null) {
    body.approved_amount = approvedAmount;
  }
  return adminJsonRequest({
    apiRoot,
    body: JSON.stringify(body),
    method: "PUT",
    path: `/reward-candidates/${candidateId}/amount-decision`,
    timeoutMs,
    token,
  });
}

export async function fetchRewardCandidateAudit({
  apiRoot = "/api",
  candidateId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { candidateId: number }): Promise<RewardAuditEvent[]> {
  return adminJsonRequest({
    apiRoot,
    path: `/reward-candidates/${candidateId}/audit`,
    timeoutMs,
    token,
  });
}

export type PlatformWalletReconciliationRow = {
  balance: string;
  external_transaction_count: number;
  internal_transaction_count: number;
  missing_credit_count: number;
  missing_notification_count: number;
  missing_payout_count: number;
  needs_reconciliation_count: number;
  organization_id: number | null;
  owner_type: string;
  reward_record_count: number;
  user_id: number | null;
  wallet_id: number;
};

export type PlatformWalletReconciliation = {
  total_external_transactions: number;
  total_internal_transactions: number;
  total_needs_reconciliation: number;
  total_reward_records: number;
  total_wallets: number;
  wallets: PlatformWalletReconciliationRow[];
};

export async function downloadPlatformCsv({
  apiRoot = "/api",
  report,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { report: PlatformCsvReport }): Promise<PlatformCsvDownload> {
  const endpoint = platformCsvEndpoints[report];
  const response = await adminRawRequest({
    accept: "text/csv, text/plain",
    apiRoot,
    path: endpoint.path,
    timeoutMs,
    token,
  });
  const body = await response.text();

  return {
    body,
    filename: filenameFromContentDisposition(response.headers.get("content-disposition")) || endpoint.filename,
  };
}

export async function fetchPlatformWalletReconciliation({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions): Promise<PlatformWalletReconciliation> {
  return adminJsonRequest({
    apiRoot,
    path: "/reports/platform/wallet-reconciliation",
    timeoutMs,
    token,
  });
}

export async function fetchPlatformSystemStatus({
  apiRoot = "",
  timeoutMs = DEFAULT_TIMEOUT_MS,
}: Omit<AdminRequestOptions, "token"> = {}): Promise<PlatformSystemStatus> {
  const [liveness, readiness] = await Promise.all([
    systemJsonRequest<SystemLiveness>({ apiRoot, path: "/health", timeoutMs }),
    systemJsonRequest<SystemReadiness>({
      acceptedStatuses: [503],
      apiRoot,
      path: "/ready",
      timeoutMs,
    }),
  ]);

  return {
    liveness,
    readiness,
  };
}

async function adminJsonRequest<T>({
  accept = "application/json, text/plain",
  apiRoot,
  body,
  method,
  path,
  timeoutMs,
  token,
}: AdminRequestOptions & {
  accept?: string;
  body?: string;
  method?: string;
  path: string;
}): Promise<T> {
  const response = await adminRawRequest({ accept, apiRoot, body, method, path, timeoutMs, token });
  return (await response.json()) as T;
}

async function adminRawRequest({
  accept = "application/json, text/plain",
  apiRoot = "/api",
  body,
  method = "GET",
  path,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & {
  accept?: string;
  body?: string;
  method?: string;
  path: string;
}): Promise<Response> {
  const trimmedToken = token?.trim() || "";
  if (!trimmedToken) {
    throw new AdminRequestError("A sign-in token is required.", 401, "missing_token");
  }

  return rawRequest({
    accept,
    apiRoot,
    body,
    headers: {
      Authorization: `Bearer ${trimmedToken}`,
    },
    method,
    path,
    timeoutMs,
  });
}

async function systemJsonRequest<T>({
  acceptedStatuses = [],
  apiRoot,
  path,
  timeoutMs,
}: {
  acceptedStatuses?: number[];
  apiRoot: string;
  path: string;
  timeoutMs: number;
}): Promise<T> {
  const response = await rawRequest({
    accept: "application/json, text/plain",
    acceptedStatuses,
    apiRoot,
    method: "GET",
    path,
    timeoutMs,
  });
  return (await response.json()) as T;
}

async function rawRequest({
  accept,
  acceptedStatuses = [],
  apiRoot,
  body,
  headers = {},
  method,
  path,
  timeoutMs,
}: {
  accept: string;
  acceptedStatuses?: number[];
  apiRoot: string;
  body?: string;
  headers?: Record<string, string>;
  method: string;
  path: string;
  timeoutMs: number;
}): Promise<Response> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}${path}`, {
      body,
      headers: {
        Accept: accept,
        ...(body ? { "Content-Type": "application/json" } : {}),
        ...headers,
      },
      method,
      signal: controller.signal,
    });

    if (!response.ok && !acceptedStatuses.includes(response.status)) {
      throw await adminErrorFromResponse(response);
    }

    return response;
  } catch (error) {
    if (error instanceof AdminRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new AdminRequestError("Admin request timed out.", 0, "timeout");
    }

    throw new AdminRequestError("Admin request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}

async function adminErrorFromResponse(response: Response) {
  const fallbackCode = codeFromStatus(response.status);
  const contentType = response.headers.get("content-type") || "";

  if (contentType.includes("application/json")) {
    const body = (await response.json()) as AdminErrorEnvelope;
    return new AdminRequestError(
      body.error?.message || response.statusText || "Admin request failed.",
      response.status,
      body.error?.code || fallbackCode,
    );
  }

  const text = await response.text();
  return new AdminRequestError(text || response.statusText || "Admin request failed.", response.status, fallbackCode);
}

function codeFromStatus(status: number) {
  if (status === 401) {
    return "unauthorized";
  }
  if (status === 403) {
    return "permission_denied";
  }
  if (status === 404) {
    return "not_found";
  }
  if (status === 409) {
    return "conflict";
  }
  if (status >= 500) {
    return "server_error";
  }
  return "admin_error";
}

function filenameFromContentDisposition(header: string | null) {
  if (!header) {
    return null;
  }

  const quoted = header.match(/filename="([^"]+)"/i);
  if (quoted?.[1]) {
    return quoted[1];
  }

  const bare = header.match(/filename=([^;]+)/i);
  return bare?.[1]?.trim() || null;
}
