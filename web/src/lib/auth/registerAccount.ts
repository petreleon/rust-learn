import { AuthRequestError } from "./AuthRequestError";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { registrationErrorFromResponse } from "./registrationErrorFromResponse";
import { type RegisterAccountOptions } from "./RegisterAccountOptions";

export async function registerAccount({
  apiRoot = "/api",
  dateOfBirth,
  email,
  name,
  password,
  timeoutMs = DEFAULT_TIMEOUT_MS,
}: RegisterAccountOptions): Promise<string> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(`${apiRoot}/auth/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        date_of_birth: dateOfBirth || undefined,
        email,
        name,
        password,
      }),
      signal: controller.signal,
    });

    if (!response.ok) {
      throw await registrationErrorFromResponse(response);
    }

    return await response.text();
  } catch (error) {
    if (error instanceof AuthRequestError) {
      throw error;
    }

    if (error instanceof DOMException && error.name === "AbortError") {
      throw new AuthRequestError("Registration timed out.", 0, "timeout");
    }

    throw new AuthRequestError("Registration failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
