import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { SessionRequestError } from "./SessionRequestError";
import { sessionErrorFromResponse } from "./sessionErrorFromResponse";
import { type CurrentSession } from "./CurrentSession";
import { type FetchCurrentSessionOptions } from "./FetchCurrentSessionOptions";

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
