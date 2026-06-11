import { AuthRequestError } from "./AuthRequestError";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { verificationErrorFromResponse } from "./verificationErrorFromResponse";
import { type VerifyEmailOptions } from "./VerifyEmailOptions";
import { type VerifyEmailResult } from "./VerifyEmailResult";

export async function verifyEmailToken({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: VerifyEmailOptions): Promise<VerifyEmailResult> {
  const normalizedToken = token.trim();
  if (!normalizedToken) {
    throw new AuthRequestError("Verification token is required.", 400, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(
      `${apiRoot}/auth/verify-email?token=${encodeURIComponent(normalizedToken)}`,
      { signal: controller.signal },
    );
    const message = await response.text();

    if (!response.ok) {
      throw verificationErrorFromResponse(response, message);
    }

    return {
      message,
      state: message.toLowerCase().includes("already") ? "already_verified" : "verified",
    };
  } catch (error) {
    if (error instanceof AuthRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new AuthRequestError("Email verification timed out.", 0, "timeout");
    }

    throw new AuthRequestError("Email verification failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
