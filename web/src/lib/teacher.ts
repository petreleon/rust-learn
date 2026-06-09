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

export type TeacherApplicationSnapshot = {
  application: TeacherApplication | null;
  audit_events: TeacherApplicationAuditEvent[];
};

export type TeacherCoursesResponse = {
  courses: TeacherCourseDashboardItem[];
  lifecycle_status: string | null;
  limit: number;
  offset: number;
  search: string | null;
  total: number;
};

export type TeacherCourseDashboardItem = {
  content: TeacherCourseContentSummary;
  id: number;
  lifecycle_status: string;
  organizations: TeacherCourseOrganization[];
  permissions: TeacherCoursePermissionSummary;
  reward_queue: TeacherCourseRewardQueueSummary;
  rewards: TeacherCourseRewardSummary;
  roster: TeacherCourseRosterSummary;
  title: string;
};

export type TeacherCourseWorkspaceResponse = {
  chapters: TeacherCourseWorkspaceChapter[];
  course: TeacherCourseDashboardItem;
  publication: TeacherCoursePublicationSummary;
  teacher_roles: string[];
};

export type TeacherCoursePublicationSummary = {
  content_publication_status_supported: boolean;
  course_lifecycle_status: string;
};

export type TeacherCourseWorkspaceChapter = {
  contents: TeacherCourseWorkspaceContent[];
  id: number;
  order: number;
  title: string;
};

export type TeacherCourseWorkspaceContent = {
  content_type: string;
  data_present: boolean;
  display_state: string;
  id: number;
  order: number;
  processing_error: string | null;
  processing_status: string | null;
  publication_status: string;
};

export type TeacherCourseOrganization = {
  id: number;
  name: string;
};

export type TeacherCourseContentSummary = {
  chapter_count: number;
  content_count: number;
  content_types: string[];
  has_content: boolean;
};

export type TeacherCourseRewardSummary = {
  active_policy_count: number;
  available: boolean;
  event_types: string[];
  payment_strategies: string[];
  token_amounts: string[];
};

export type TeacherCourseRosterSummary = {
  enrolled_student_count: number;
  pending_join_request_count: number;
  waitlisted_join_request_count: number;
};

export type TeacherCourseRewardQueueSummary = {
  failed_count: number;
  pending_teacher_count: number;
  teacher_approved_count: number;
};

export type TeacherCoursePermissionSummary = {
  can_approve_reward_candidates: boolean;
  can_manage_content: boolean;
  can_manage_enrollments: boolean;
  can_manage_reward_rules: boolean;
  can_manage_settings: boolean;
  can_view_reward_candidates: boolean;
};

export type SubmitTeacherApplicationPayload = {
  experience_summary: string;
  idempotency_key?: string;
  organization_sponsor_id?: number;
  portfolio_links?: string[];
  requested_course_id?: number;
  requested_organization_id?: number;
  requested_scope: TeacherApplicationScope;
};

type TeacherErrorEnvelope = {
  error?: {
    code?: string;
    message?: string;
  };
};

export class TeacherRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "TeacherRequestError";
    this.status = status;
    this.code = code;
  }
}

export type TeacherRequestOptions = {
  apiRoot?: string;
  timeoutMs?: number;
  token: string;
};

export type TeacherCoursesOptions = TeacherRequestOptions & {
  lifecycleStatus?: string;
  limit?: number;
  offset?: number;
  search?: string;
};

export type TeacherCourseWorkspaceOptions = TeacherRequestOptions & {
  courseId: number | string;
};

export type SubmitTeacherApplicationOptions = TeacherRequestOptions & {
  payload: SubmitTeacherApplicationPayload;
};

const DEFAULT_TIMEOUT_MS = 10000;

export async function fetchMyTeacherApplication({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherRequestOptions): Promise<TeacherApplicationSnapshot> {
  return teacherJsonRequest<TeacherApplicationSnapshot>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/teacher-applications/me`,
  });
}

export async function fetchTeachingCourses({
  apiRoot = "/api",
  lifecycleStatus,
  limit = 25,
  offset,
  search,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherCoursesOptions): Promise<TeacherCoursesResponse> {
  const query = new URLSearchParams();
  query.set("limit", String(limit));
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  if (lifecycleStatus && lifecycleStatus !== "all") {
    query.set("lifecycle_status", lifecycleStatus);
  }
  const trimmedSearch = search?.trim();
  if (trimmedSearch) {
    query.set("search", trimmedSearch);
  }

  const suffix = query.toString();
  return teacherJsonRequest<TeacherCoursesResponse>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/teaching${suffix ? `?${suffix}` : ""}`,
  });
}

export async function fetchTeachingCourseWorkspace({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherCourseWorkspaceOptions): Promise<TeacherCourseWorkspaceResponse> {
  return teacherJsonRequest<TeacherCourseWorkspaceResponse>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/teaching/${courseId}`,
  });
}

export async function submitTeacherApplication({
  apiRoot = "/api",
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: SubmitTeacherApplicationOptions): Promise<TeacherApplication> {
  return teacherJsonRequest<TeacherApplication>({
    body: JSON.stringify(payload),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/teacher-applications`,
  });
}

async function teacherJsonRequest<T>({
  body,
  method,
  timeoutMs,
  token,
  url,
}: {
  body?: string;
  method: string;
  timeoutMs: number;
  token: string;
  url: string;
}): Promise<T> {
  const response = await teacherRawRequest({ body, method, timeoutMs, token, url });
  if (!response.ok) {
    throw await teacherErrorFromResponse(response, "Teacher request failed.");
  }

  return (await response.json()) as T;
}

async function teacherRawRequest({
  body,
  method,
  timeoutMs,
  token,
  url,
}: {
  body?: string;
  method: string;
  timeoutMs: number;
  token: string;
  url: string;
}) {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new TeacherRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    return await fetch(url, {
      body,
      headers: {
        Authorization: `Bearer ${trimmedToken}`,
        ...(body ? { "Content-Type": "application/json" } : {}),
      },
      method,
      signal: controller.signal,
    });
  } catch (error) {
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new TeacherRequestError("Teacher request timed out.", 0, "timeout");
    }

    throw new TeacherRequestError("Teacher request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}

async function teacherErrorFromResponse(response: Response, fallbackMessage: string) {
  const fallbackCode =
    response.status === 401
      ? "unauthorized"
      : response.status === 403
        ? "permission_denied"
        : response.status === 404
          ? "not_found"
          : response.status === 409
            ? "conflict"
            : response.status >= 500
              ? "server_error"
              : "teacher_error";
  const contentType = response.headers.get("content-type") || "";

  if (contentType.includes("application/json")) {
    const body = (await response.json()) as TeacherErrorEnvelope;
    return new TeacherRequestError(
      body.error?.message || fallbackMessage,
      response.status,
      body.error?.code || fallbackCode,
    );
  }

  const text = await response.text();
  return new TeacherRequestError(text || response.statusText || fallbackMessage, response.status, fallbackCode);
}
