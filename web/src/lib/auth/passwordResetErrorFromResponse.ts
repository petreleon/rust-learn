import { AuthRequestError } from "./AuthRequestError";

export async function passwordResetErrorFromResponse(response: Response) {
  const text = await response.text();
  const message = text || response.statusText || "Password reset failed.";
  const lower = message.toLowerCase();

  if (response.status === 400 && lower.includes("token")) {
    return new AuthRequestError(message, response.status, "invalid_reset_token");
  }

  if (response.status === 400 && lower.includes("password")) {
    return new AuthRequestError(message, response.status, "password_policy");
  }

  if (response.status >= 500) {
    return new AuthRequestError(message, response.status, "server_error");
  }

  return new AuthRequestError(message, response.status, "password_reset_error");
}
