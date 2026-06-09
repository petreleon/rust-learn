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
  path,
  timeoutMs,
  token,
}: AdminRequestOptions & {
  accept?: string;
  path: string;
}): Promise<T> {
  const response = await adminRawRequest({ accept, apiRoot, path, timeoutMs, token });
  return (await response.json()) as T;
}

async function adminRawRequest({
  accept = "application/json, text/plain",
  apiRoot = "/api",
  method = "GET",
  path,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & {
  accept?: string;
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
  headers = {},
  method,
  path,
  timeoutMs,
}: {
  accept: string;
  acceptedStatuses?: number[];
  apiRoot: string;
  headers?: Record<string, string>;
  method: string;
  path: string;
  timeoutMs: number;
}): Promise<Response> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}${path}`, {
      headers: {
        Accept: accept,
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
