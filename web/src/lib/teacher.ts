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
