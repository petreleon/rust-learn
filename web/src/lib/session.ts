export const SESSION_TOKEN_KEY = "rustlearn.session.jwt";

export type CurrentSession = {
  user: {
    id: number;
    name: string;
    email: string;
    email_verified: boolean;
    kyc_verified: boolean;
  };
  platform: PlatformSessionScope;
  organizations: OrganizationSessionScope[];
  courses: CourseSessionScope[];
  delegated_permissions: DelegatedPermissionSession[];
};

export type PlatformSessionScope = {
  roles: string[];
  direct_permissions: string[];
  delegated_permissions: string[];
  effective_permissions: string[];
};

export type OrganizationSessionScope = PlatformSessionScope & {
  id: number;
  name: string;
};

export type CourseSessionScope = PlatformSessionScope & {
  id: number;
  title: string;
  lifecycle_status: string;
};

export type DelegatedPermissionSession = {
  id: number;
  grantor_user_id: number;
  permission: string;
  scope_type: string;
  organization_id: number | null;
  organization_name: string | null;
  course_id: number | null;
  course_title: string | null;
  course_lifecycle_status: string | null;
  expires_at: string | null;
};

type SessionErrorEnvelope = {
  error?: {
    code?: string;
    message?: string;
  };
};

export class SessionRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "SessionRequestError";
    this.status = status;
    this.code = code;
  }
}

export type FetchCurrentSessionOptions = {
  apiRoot?: string;
  token: string;
  timeoutMs?: number;
};

const DEFAULT_TIMEOUT_MS = 10000;

export function readStoredSessionToken() {
  if (typeof window === "undefined") {
    return null;
  }

  return window.sessionStorage.getItem(SESSION_TOKEN_KEY);
}

export function storeSessionToken(token: string) {
  if (typeof window === "undefined") {
    return;
  }

  window.sessionStorage.setItem(SESSION_TOKEN_KEY, token);
}

export function clearStoredSessionToken() {
  if (typeof window === "undefined") {
    return;
  }

  window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
}

export async function fetchCurrentSession({
  apiRoot = "/api",
  token,
  timeoutMs = DEFAULT_TIMEOUT_MS,
}: FetchCurrentSessionOptions): Promise<CurrentSession> {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new SessionRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/me`, {
      headers: {
        Authorization: `Bearer ${trimmedToken}`,
      },
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await sessionErrorFromResponse(response);
    }

    return (await response.json()) as CurrentSession;
  } catch (error) {
    if (error instanceof SessionRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new SessionRequestError("Session request timed out.", 0, "timeout");
    }

    throw new SessionRequestError("Session request failed.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}

async function sessionErrorFromResponse(response: Response) {
  const fallback = new SessionRequestError(
    response.statusText || "Session request failed.",
    response.status,
    response.status === 401 ? "unauthorized" : "session_error",
  );

  const contentType = response.headers.get("content-type") || "";
  if (!contentType.includes("application/json")) {
    const text = await response.text();
    return new SessionRequestError(text || fallback.message, response.status, fallback.code);
  }

  const body = (await response.json()) as SessionErrorEnvelope;
  return new SessionRequestError(
    body.error?.message || fallback.message,
    response.status,
    body.error?.code || fallback.code,
  );
}

export type NotificationPreferences = {
  email_enabled: boolean;
  push_enabled: boolean;
  updated_at: string;
  user_id: number;
};

export async function fetchNotificationPreferences({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: {
  apiRoot?: string;
  timeoutMs?: number;
  token: string;
}): Promise<NotificationPreferences> {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new SessionRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/me/preferences`, {
      headers: { Authorization: `Bearer ${trimmedToken}` },
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await sessionErrorFromResponse(response);
    }

    return (await response.json()) as NotificationPreferences;
  } catch (error) {
    if (error instanceof SessionRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new SessionRequestError("Preferences request timed out.", 0, "timeout");
    }
    throw new SessionRequestError(
      error instanceof Error ? error.message : "Preferences request failed.",
      0,
      "network_error",
    );
  } finally {
    clearTimeout(timeout);
  }
}

export async function saveNotificationPreferences({
  apiRoot = "/api",
  emailEnabled,
  pushEnabled,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: {
  apiRoot?: string;
  emailEnabled: boolean;
  pushEnabled: boolean;
  timeoutMs?: number;
  token: string;
}): Promise<NotificationPreferences> {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new SessionRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/me/preferences`, {
      body: JSON.stringify({ email_enabled: emailEnabled, push_enabled: pushEnabled }),
      headers: {
        Authorization: `Bearer ${trimmedToken}`,
        "Content-Type": "application/json",
      },
      method: "PUT",
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await sessionErrorFromResponse(response);
    }

    return (await response.json()) as NotificationPreferences;
  } catch (error) {
    if (error instanceof SessionRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new SessionRequestError("Preferences request timed out.", 0, "timeout");
    }
    throw new SessionRequestError(
      error instanceof Error ? error.message : "Preferences save failed.",
      0,
      "network_error",
    );
  } finally {
    clearTimeout(timeout);
  }
}

type FetchOptions = { apiRoot?: string; timeoutMs?: number; token: string };

export type NotificationItem = {
  body: string;
  created_at: string;
  id: number;
  read: boolean;
  title: string;
  user_id: number | null;
};

export async function fetchNotifications({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FetchOptions): Promise<NotificationItem[]> {
  const trimmedToken = token.trim();
  if (!trimmedToken) throw new SessionRequestError("Token required.", 401, "missing_token");
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(`${apiRoot}/me/notifications`, {
      headers: { Authorization: `Bearer ${trimmedToken}` },
      signal: controller.signal,
    });
    if (!response.ok) throw await sessionErrorFromResponse(response);
    return (await response.json()) as NotificationItem[];
  } catch (error) {
    if (error instanceof SessionRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") throw new SessionRequestError("Timed out.", 0, "timeout");
    throw new SessionRequestError(error instanceof Error ? error.message : "Failed.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}

export async function markNotificationRead({
  apiRoot = "/api",
  notificationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FetchOptions & { notificationId: number }): Promise<void> {
  const trimmedToken = token.trim();
  if (!trimmedToken) throw new SessionRequestError("Token required.", 401, "missing_token");
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(`${apiRoot}/me/notifications/${notificationId}/read`, {
      headers: { Authorization: `Bearer ${trimmedToken}` },
      method: "PUT",
      signal: controller.signal,
    });
    if (!response.ok) throw await sessionErrorFromResponse(response);
  } catch (error) {
    if (error instanceof SessionRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") throw new SessionRequestError("Timed out.", 0, "timeout");
    throw new SessionRequestError(error instanceof Error ? error.message : "Failed.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}

export async function clearNotifications({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FetchOptions): Promise<void> {
  const trimmedToken = token.trim();
  if (!trimmedToken) throw new SessionRequestError("Token required.", 401, "missing_token");
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(`${apiRoot}/me/notifications`, {
      headers: { Authorization: `Bearer ${trimmedToken}` },
      method: "DELETE",
      signal: controller.signal,
    });
    if (!response.ok) throw await sessionErrorFromResponse(response);
  } catch (error) {
    if (error instanceof SessionRequestError) throw error;
    if (error instanceof DOMException && error.name === "AbortError") throw new SessionRequestError("Timed out.", 0, "timeout");
    throw new SessionRequestError(error instanceof Error ? error.message : "Failed.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
