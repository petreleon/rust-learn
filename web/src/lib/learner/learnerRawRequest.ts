import { LearnerRequestError } from "./LearnerRequestError";

export async function learnerRawRequest({
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
    throw new LearnerRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const headers: Record<string, string> = {
      Authorization: `Bearer ${trimmedToken}`,
    };
    if (body) {
      headers["Content-Type"] = "application/json";
    }
    return await fetch(url, {
      body,
      headers,
      method,
      signal: controller.signal,
    });
  } catch (error) {
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new LearnerRequestError("Learner request timed out.", 0, "timeout");
    }

    throw new LearnerRequestError("Learner request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
