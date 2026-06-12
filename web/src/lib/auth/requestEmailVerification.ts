import { AuthRequestError } from "./AuthRequestError";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { type RequestEmailVerificationOptions } from "./RequestEmailVerificationOptions";

export async function requestEmailVerification({
  apiRoot = "/api",
  email,
  timeoutMs = DEFAULT_TIMEOUT_MS,
}: RequestEmailVerificationOptions): Promise<string> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/auth/resend-verification`, {
      body: JSON.stringify({ email }),
      headers: { "Content-Type": "application/json" },
      method: "POST",
      signal: controller.signal,
    });
    const message = await response.text();

    if (!response.ok) {
      const code = response.status === 400 ? "missing_email" : "verification_request_error";
      throw new AuthRequestError(message || "Verification request failed.", response.status, code);
    }

    return message;
  } catch (error) {
    if (error instanceof AuthRequestError) {
      throw error;
    }
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new AuthRequestError("Verification email request timed out.", 0, "timeout");
    }
    throw new AuthRequestError("Verification email request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
