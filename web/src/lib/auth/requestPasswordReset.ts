import { AuthRequestError } from "./AuthRequestError";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { passwordResetErrorFromResponse } from "./passwordResetErrorFromResponse";
import { type RequestPasswordResetOptions } from "./RequestPasswordResetOptions";

export async function requestPasswordReset({
  apiRoot = "/api",
  email,
  timeoutMs = DEFAULT_TIMEOUT_MS,
}: RequestPasswordResetOptions): Promise<string> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/auth/forgot-password`, {
      body: JSON.stringify({ email }),
      headers: { "Content-Type": "application/json" },
      method: "POST",
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await passwordResetErrorFromResponse(response);
    }

    return await response.text();
  } catch (error) {
    if (error instanceof AuthRequestError) {
      throw error;
    }
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new AuthRequestError("Password reset request timed out.", 0, "timeout");
    }
    throw new AuthRequestError("Password reset request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
