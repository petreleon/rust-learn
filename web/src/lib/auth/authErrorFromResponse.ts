import { AuthRequestError } from "./AuthRequestError";

export async function authErrorFromResponse(response: Response) {
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
