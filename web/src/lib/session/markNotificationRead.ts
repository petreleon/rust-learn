import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { SessionRequestError } from "./SessionRequestError";
import { sessionErrorFromResponse } from "./sessionErrorFromResponse";
import { type FetchOptions } from "./FetchOptions";

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
