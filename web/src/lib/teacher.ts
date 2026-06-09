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

export type TeacherCourseEnrollmentWorkspaceResponse = {
  course: TeacherCourseDashboardItem;
  join_requests: TeacherCourseJoinRequestPage;
  progress_supported: boolean;
  reward_eligibility_supported: boolean;
  roster: TeacherCourseRosterPage;
  teacher_roles: string[];
};

export type TeacherCourseJoinRequestPage = {
  limit: number;
  offset: number;
  requests: TeacherCourseJoinRequestItem[];
  status: string | null;
  total: number;
};

export type TeacherCourseJoinRequestItem = {
  can_decide: boolean;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  id: number;
  requester: TeacherEnrollmentUserSummary;
  reviewer: TeacherEnrollmentUserSummary | null;
  status: string;
  updated_at: string;
};

export type TeacherCourseJoinDecisionResponse = {
  course_id: number;
  created_at: string;
  decided_at: string | null;
  decision_reason: string | null;
  id: number;
  requester_user_id: number;
  reviewer_user_id: number | null;
  status: string;
  updated_at: string;
};

export type TeacherCourseRosterPage = {
  learners: TeacherCourseRosterLearner[];
  total: number;
};

export type TeacherCourseRosterLearner = {
  access_state: string;
  can_remove: boolean;
  latest_join_request_status: string | null;
  progress_supported: boolean;
  reward_eligibility_supported: boolean;
  roles: string[];
  user: TeacherEnrollmentUserSummary;
};

export type TeacherEnrollmentUserSummary = {
  email: string;
  email_verified: boolean;
  id: number;
  kyc_verified: boolean;
  name: string;
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

export type TeacherChapter = {
  course_id: number;
  id: number;
  order: number;
  title: string;
};

export type TeacherContent = {
  chapter_id: number;
  content_type: string;
  data: string | null;
  id: number;
  order: number;
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

export type TeacherCourseEnrollmentOptions = TeacherRequestOptions & {
  courseId: number | string;
  limit?: number;
  offset?: number;
  status?: string;
};

export type CreateTeacherChapterPayload = {
  order: number;
  title: string;
};

export type CreateTeacherChapterOptions = TeacherRequestOptions & {
  courseId: number | string;
  payload: CreateTeacherChapterPayload;
};

export type CreateTeacherContentPayload = {
  content_type: string;
  data?: string | null;
  order: number;
};

export type CreateTeacherContentOptions = TeacherRequestOptions & {
  chapterId: number | string;
  courseId: number | string;
  payload: CreateTeacherContentPayload;
};

export type DecideTeacherJoinRequestPayload = {
  decision_reason?: string | null;
  status: "approved" | "rejected" | "waitlisted";
};

export type DecideTeacherJoinRequestOptions = TeacherRequestOptions & {
  courseId: number | string;
  payload: DecideTeacherJoinRequestPayload;
  requestId: number | string;
};

export type RemoveTeacherEnrollmentOptions = TeacherRequestOptions & {
  courseId: number | string;
  userId: number | string;
};

export type TeacherEnrollmentRemovalResponse = {
  course_id: number;
  removed: boolean;
  user_id: number;
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

export async function fetchTeachingCourseEnrollments({
  apiRoot = "/api",
  courseId,
  limit = 25,
  offset,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherCourseEnrollmentOptions): Promise<TeacherCourseEnrollmentWorkspaceResponse> {
  const query = new URLSearchParams();
  query.set("limit", String(limit));
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  const trimmedStatus = status?.trim();
  if (trimmedStatus) {
    query.set("status", trimmedStatus);
  }

  const suffix = query.toString();
  return teacherJsonRequest<TeacherCourseEnrollmentWorkspaceResponse>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/teaching/${courseId}/enrollments${suffix ? `?${suffix}` : ""}`,
  });
}

export async function createTeacherChapter({
  apiRoot = "/api",
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CreateTeacherChapterOptions): Promise<TeacherChapter> {
  return teacherJsonRequest<TeacherChapter>({
    body: JSON.stringify(payload),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters`,
  });
}

export async function createTeacherContent({
  apiRoot = "/api",
  chapterId,
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CreateTeacherContentOptions): Promise<TeacherContent> {
  return teacherJsonRequest<TeacherContent>({
    body: JSON.stringify(payload),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents`,
  });
}

export async function decideTeacherJoinRequest({
  apiRoot = "/api",
  courseId,
  payload,
  requestId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: DecideTeacherJoinRequestOptions): Promise<TeacherCourseJoinDecisionResponse> {
  return teacherJsonRequest<TeacherCourseJoinDecisionResponse>({
    body: JSON.stringify(payload),
    method: "PUT",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/join-requests/${requestId}/decision`,
  });
}

export async function removeTeacherEnrollment({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
  userId,
}: RemoveTeacherEnrollmentOptions): Promise<TeacherEnrollmentRemovalResponse> {
  return teacherJsonRequest<TeacherEnrollmentRemovalResponse>({
    method: "DELETE",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/enrollments/${userId}`,
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
