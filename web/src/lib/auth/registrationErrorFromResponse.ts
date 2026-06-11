import { AuthRequestError } from "./AuthRequestError";

export async function registrationErrorFromResponse(response: Response) {
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
