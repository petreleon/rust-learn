export class AuthRequestError extends Error {
  status: number;
  code: string;

  constructor(message: string, status: number, code: string) {
    super(message);
    this.name = "AuthRequestError";
    this.status = status;
    this.code = code;
  }
}

export type LoginWithPasswordOptions = {
  apiRoot?: string;
  email: string;
  password: string;
  timeoutMs?: number;
};

const DEFAULT_TIMEOUT_MS = 10000;

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

async function authErrorFromResponse(response: Response) {
  const text = await response.text();
  const message = text || response.statusText || "Login failed.";

  if (response.status === 401) {
    return new AuthRequestError(message, response.status, "invalid_credentials");
  }

  if (response.status === 403 && message.toLowerCase().includes("email verification")) {
    return new AuthRequestError(message, response.status, "unverified_email");
  }

  if (response.status >= 500) {
    return new AuthRequestError(message, response.status, "server_error");
  }

  return new AuthRequestError(message, response.status, "login_error");
}
