import type { CurrentSession, OrganizationSessionScope } from "@/lib/session";

export type OrganizationCapabilityKey =
  | "courses"
  | "members"
  | "reports"
  | "wallet"
  | "teacher_applications"
  | "course_rewards"
  | "settings";

export type OrganizationCapability = {
  enabled: boolean;
  key: OrganizationCapabilityKey;
  label: string;
  permissions: string[];
};

export type OrganizationWorkspaceItem = {
  capabilities: OrganizationCapability[];
  delegatedPermissionCount: number;
  directPermissionCount: number;
  effectivePermissions: string[];
  effectivePermissionCount: number;
  id: number;
  name: string;
  permissionPreview: string[];
  roles: string[];
};

export type OrganizationWorkspaceSummary = {
  courseRewardScopeCount: number;
  delegatedOrganizationCount: number;
  managementScopeCount: number;
  organizations: OrganizationWorkspaceItem[];
  reportScopeCount: number;
  teacherNominationScopeCount: number;
  total: number;
  walletScopeCount: number;
};

export type OrganizationFilter = {
  capability: OrganizationCapabilityKey | "all" | "delegated";
  search: string;
};

export type TeacherApplicationDashboardSummary = {
  approved: number;
  needs_changes: number;
  rejected: number;
  submitted: number;
  total: number;
};

export type OrganizationCourseRewardDashboardRow = {
  approved_amount_total: string;
  approved_reward_count: number;
  course_id: number;
  course_title: string;
  reward_candidate_count: number;
};

export type OrganizationWalletBalanceRow = {
  balance: string;
  wallet_id: number;
};

export type OrganizationRewardDashboard = {
  approved_amount_total: string;
  approved_reward_count: number;
  course_reward_count: number;
  courses: OrganizationCourseRewardDashboardRow[];
  organization_id: number;
  organization_name: string;
  sponsored_teacher_applications: TeacherApplicationDashboardSummary;
  wallet_balance_total: string;
  wallets: OrganizationWalletBalanceRow[];
};

export type OrganizationCourseTeacher = {
  id: number;
  name: string;
};

export type OrganizationCourseContentSummary = {
  chapter_count: number;
  content_count: number;
  content_types: string[];
  has_content: boolean;
};

export type OrganizationCourseRewardSummary = {
  active_policy_count: number;
  available: boolean;
  event_types: string[];
  payment_strategies: string[];
  token_amounts: string[];
};

export type OrganizationCourseRosterSummary = {
  enrolled_student_count: number;
  pending_join_request_count: number;
  waitlisted_join_request_count: number;
};

export type OrganizationCourseRewardQueueSummary = {
  failed_count: number;
  pending_teacher_count: number;
  teacher_approved_count: number;
};

export type OrganizationCoursePermissionSummary = {
  can_create_courses: boolean;
  can_manage_course_settings: boolean;
  can_manage_enrollments: boolean;
  can_manage_reward_budget: boolean;
  can_submit_reward_events: boolean;
  can_view_courses: boolean;
  can_view_reward_reports: boolean;
};

export type OrganizationCourseListItem = {
  content: OrganizationCourseContentSummary;
  id: number;
  lifecycle_status: string;
  permissions: OrganizationCoursePermissionSummary;
  reward_queue: OrganizationCourseRewardQueueSummary;
  rewards: OrganizationCourseRewardSummary;
  roster: OrganizationCourseRosterSummary;
  teachers: OrganizationCourseTeacher[];
  title: string;
};

export type OrganizationCourseList = {
  courses: OrganizationCourseListItem[];
  lifecycle_status: string | null;
  limit: number;
  offset: number;
  organization: {
    id: number;
    name: string;
  };
  reward_available: boolean | null;
  search: string | null;
  total: number;
};

export type OrganizationCsvDownload = {
  body: string;
  filename: string;
};

export type OrganizationRequestOptions = {
  apiRoot?: string;
  timeoutMs?: number;
  token: string;
};

export type OrganizationReportOptions = OrganizationRequestOptions & {
  organizationId: number;
};

export type OrganizationCourseListOptions = OrganizationRequestOptions & {
  lifecycleStatus?: string | null;
  limit?: number;
  offset?: number;
  organizationId: number;
  rewardAvailable?: boolean | null;
  search?: string | null;
};

type OrganizationErrorEnvelope = {
  error?: {
    code?: string;
    message?: string;
  };
};

export class OrganizationRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "OrganizationRequestError";
    this.status = status;
    this.code = code;
  }
}

const DEFAULT_TIMEOUT_MS = 10000;

const capabilityPermissions: Array<{
  key: OrganizationCapabilityKey;
  label: string;
  permissions: string[];
}> = [
  {
    key: "courses",
    label: "Courses",
    permissions: ["VIEW_ORGANIZATION"],
  },
  {
    key: "members",
    label: "Members",
    permissions: ["ASSIGN_ROLES_TO_ORG_USERS", "INVITE_USER_TO_ORGANIZATION", "MANAGE_ORG_MEMBERS"],
  },
  {
    key: "reports",
    label: "Reports",
    permissions: ["VIEW_ORG_REWARD_REPORTS"],
  },
  {
    key: "wallet",
    label: "Wallet",
    permissions: ["MANAGE_ORG_BILLING", "MANAGE_ORG_REWARD_BUDGET", "MANAGE_ORG_WALLETS"],
  },
  {
    key: "teacher_applications",
    label: "Teacher nominations",
    permissions: ["NOMINATE_TEACHER_FOR_PLATFORM_REVIEW", "VIEW_ORG_TEACHER_APPLICATIONS"],
  },
  {
    key: "course_rewards",
    label: "Course rewards",
    permissions: ["SUBMIT_ORG_COURSE_REWARD_EVENT"],
  },
  {
    key: "settings",
    label: "Settings",
    permissions: ["MANAGE_ORG_SETTINGS", "VIEW_ORGANIZATION"],
  },
];

export function buildOrganizationWorkspace(session: CurrentSession): OrganizationWorkspaceSummary {
  const organizations = session.organizations
    .filter(hasOrganizationScopeSignal)
    .map(buildOrganizationWorkspaceItem)
    .sort((left, right) => left.name.localeCompare(right.name));

  return {
    delegatedOrganizationCount: organizations.filter((organization) => organization.delegatedPermissionCount > 0).length,
    courseRewardScopeCount: organizations.filter((organization) =>
      organization.capabilities.some((capability) => capability.key === "course_rewards" && capability.enabled),
    ).length,
    managementScopeCount: organizations.filter((organization) =>
      organization.capabilities.some(
        (capability) =>
          capability.enabled &&
          (capability.key === "members" ||
            capability.key === "teacher_applications" ||
            capability.key === "settings"),
      ),
    ).length,
    organizations,
    reportScopeCount: organizations.filter((organization) =>
      organization.capabilities.some((capability) => capability.key === "reports" && capability.enabled),
    ).length,
    teacherNominationScopeCount: organizations.filter((organization) =>
      organization.capabilities.some((capability) => capability.key === "teacher_applications" && capability.enabled),
    ).length,
    total: organizations.length,
    walletScopeCount: organizations.filter((organization) =>
      organization.capabilities.some((capability) => capability.key === "wallet" && capability.enabled),
    ).length,
  };
}

export function findOrganizationWorkspaceItem(
  session: CurrentSession,
  organizationId: number,
): OrganizationWorkspaceItem | null {
  return buildOrganizationWorkspace(session).organizations.find((organization) => organization.id === organizationId) || null;
}

export function organizationMatchesCapability(
  organization: OrganizationWorkspaceItem,
  capability: OrganizationCapabilityKey | "all" | "delegated",
) {
  if (capability === "all") {
    return true;
  }

  if (capability === "delegated") {
    return organization.delegatedPermissionCount > 0;
  }

  return organization.capabilities.some((item) => item.key === capability && item.enabled);
}

export function filterOrganizationWorkspace(
  organizations: OrganizationWorkspaceItem[],
  filter: OrganizationFilter,
) {
  const normalizedSearch = filter.search.trim().toLocaleLowerCase();

  return organizations.filter((organization) => {
    const matchesCapability = organizationMatchesCapability(organization, filter.capability);
    const matchesSearch =
      normalizedSearch.length === 0 ||
      organization.name.toLocaleLowerCase().includes(normalizedSearch) ||
      organization.roles.some((role) => role.toLocaleLowerCase().includes(normalizedSearch)) ||
      organization.effectivePermissions.some((permission) =>
        permission.toLocaleLowerCase().includes(normalizedSearch),
      );

    return matchesCapability && matchesSearch;
  });
}

export function enabledOrganizationCapabilities(organization: OrganizationWorkspaceItem) {
  return organization.capabilities.filter((capability) => capability.enabled);
}

export function missingOrganizationPermissions(capability: OrganizationCapability) {
  return capability.permissions;
}

export async function fetchOrganizationRewardDashboard({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationReportOptions): Promise<OrganizationRewardDashboard> {
  return organizationJsonRequest({
    apiRoot,
    path: `/reports/organizations/${organizationId}/reward-dashboard`,
    timeoutMs,
    token,
  });
}

export async function downloadOrganizationRewardDashboardCsv({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationReportOptions): Promise<OrganizationCsvDownload> {
  const response = await organizationRawRequest({
    apiRoot,
    path: `/reports/organizations/${organizationId}/reward-dashboard.csv`,
    timeoutMs,
    token,
    accept: "text/csv, text/plain",
  });
  const body = await response.text();

  return {
    body,
    filename: filenameFromContentDisposition(response.headers.get("content-disposition")) ||
      `organization-${organizationId}-reward-dashboard.csv`,
  };
}

export async function fetchOrganizationCourses({
  apiRoot = "/api",
  lifecycleStatus,
  limit,
  offset,
  organizationId,
  rewardAvailable,
  search,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationCourseListOptions): Promise<OrganizationCourseList> {
  const query = new URLSearchParams();
  const normalizedSearch = search?.trim();
  if (normalizedSearch) {
    query.set("search", normalizedSearch);
  }
  const normalizedLifecycleStatus = lifecycleStatus?.trim();
  if (normalizedLifecycleStatus) {
    query.set("lifecycle_status", normalizedLifecycleStatus);
  }
  if (typeof rewardAvailable === "boolean") {
    query.set("reward_available", String(rewardAvailable));
  }
  if (typeof limit === "number") {
    query.set("limit", String(limit));
  }
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }

  const suffix = query.toString();
  return organizationJsonRequest({
    apiRoot,
    path: `/organizations/${organizationId}/courses${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}

async function organizationJsonRequest<T>({
  accept = "application/json, text/plain",
  apiRoot,
  path,
  timeoutMs,
  token,
}: OrganizationRequestOptions & {
  accept?: string;
  path: string;
}): Promise<T> {
  const response = await organizationRawRequest({ accept, apiRoot, path, timeoutMs, token });
  return (await response.json()) as T;
}

async function organizationRawRequest({
  accept = "application/json, text/plain",
  apiRoot = "/api",
  path,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationRequestOptions & {
  accept?: string;
  path: string;
}): Promise<Response> {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new OrganizationRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}${path}`, {
      headers: {
        Accept: accept,
        Authorization: `Bearer ${trimmedToken}`,
      },
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await organizationErrorFromResponse(response);
    }

    return response;
  } catch (error) {
    if (error instanceof OrganizationRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new OrganizationRequestError("Organization request timed out.", 0, "timeout");
    }

    throw new OrganizationRequestError("Organization request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}

async function organizationErrorFromResponse(response: Response) {
  const fallbackCode = codeFromStatus(response.status);
  const contentType = response.headers.get("content-type") || "";

  if (contentType.includes("application/json")) {
    const body = (await response.json()) as OrganizationErrorEnvelope;
    return new OrganizationRequestError(
      body.error?.message || response.statusText || "Organization request failed.",
      response.status,
      body.error?.code || fallbackCode,
    );
  }

  const text = await response.text();
  return new OrganizationRequestError(
    text || response.statusText || "Organization request failed.",
    response.status,
    fallbackCode,
  );
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
  if (status >= 500) {
    return "server_error";
  }
  return "organization_error";
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

function buildOrganizationWorkspaceItem(organization: OrganizationSessionScope): OrganizationWorkspaceItem {
  return {
    capabilities: capabilityPermissions.map((capability) => ({
      enabled: capability.permissions.some((permission) => organization.effective_permissions.includes(permission)),
      key: capability.key,
      label: capability.label,
      permissions: capability.permissions,
    })),
    delegatedPermissionCount: organization.delegated_permissions.length,
    directPermissionCount: organization.direct_permissions.length,
    effectivePermissions: [...organization.effective_permissions],
    effectivePermissionCount: organization.effective_permissions.length,
    id: organization.id,
    name: organization.name,
    permissionPreview: organization.effective_permissions.slice(0, 5),
    roles: organization.roles,
  };
}

function hasOrganizationScopeSignal(organization: OrganizationSessionScope) {
  return (
    organization.roles.length > 0 ||
    organization.direct_permissions.length > 0 ||
    organization.delegated_permissions.length > 0 ||
    organization.effective_permissions.length > 0
  );
}
