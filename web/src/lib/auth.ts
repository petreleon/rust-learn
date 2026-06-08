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

export type RegisterAccountOptions = {
  apiRoot?: string;
  dateOfBirth?: string;
  email: string;
  name: string;
  password: string;
  timeoutMs?: number;
};

export type VerifyEmailOptions = {
  apiRoot?: string;
  timeoutMs?: number;
  token: string;
};

export type VerifyEmailResult = {
  message: string;
  state: "verified" | "already_verified";
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

function verificationErrorFromResponse(response: Response, message: string) {
  const normalizedMessage = message.toLowerCase();

  if (response.status === 400 && normalizedMessage.includes("invalid")) {
    return new AuthRequestError(message, response.status, "invalid_token");
  }

  if (response.status === 400 && normalizedMessage.includes("expired")) {
    return new AuthRequestError(message, response.status, "expired_token");
  }

  if (response.status >= 500) {
    return new AuthRequestError(message || "Email verification failed.", response.status, "server_error");
  }

  return new AuthRequestError(message || "Email verification failed.", response.status, "verification_error");
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

async function registrationErrorFromResponse(response: Response) {
  const text = await response.text();
  const message = text || response.statusText || "Registration failed.";

  if (response.status === 409) {
    return new AuthRequestError(message, response.status, "duplicate_email");
  }

  if (response.status === 400 && message.toLowerCase().includes("password")) {
    return new AuthRequestError(message, response.status, "password_policy");
  }

  if (response.status >= 500) {
    return new AuthRequestError(message, response.status, "server_error");
  }

  return new AuthRequestError(message, response.status, "registration_error");
}
