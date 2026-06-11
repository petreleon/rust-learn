import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { SessionRequestError } from "./SessionRequestError";
import { sessionErrorFromResponse } from "./sessionErrorFromResponse";
import { type NotificationPreferences } from "./NotificationPreferences";

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
