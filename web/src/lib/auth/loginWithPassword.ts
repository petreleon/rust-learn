import { AuthRequestError } from "./AuthRequestError";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { authErrorFromResponse } from "./authErrorFromResponse";
import { type LoginWithPasswordOptions } from "./LoginWithPasswordOptions";

export async function loginWithPassword({
  apiRoot = "/api",
  email,
  password,
  timeoutMs = DEFAULT_TIMEOUT_MS,
}: LoginWithPasswordOptions): Promise<string> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/auth/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await authErrorFromResponse(response);
    }

    const token = (await response.json()) as unknown;
    if (typeof token !== "string" || !token.trim()) {
      throw new AuthRequestError("Login response did not include a token.", 500, "invalid_login_response");
    }

    return token;
  } catch (error) {
    if (error instanceof AuthRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new AuthRequestError("Login timed out.", 0, "timeout");
    }

    throw new AuthRequestError("Login failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
