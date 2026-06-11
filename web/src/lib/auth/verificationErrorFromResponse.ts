import { AuthRequestError } from "./AuthRequestError";

export function verificationErrorFromResponse(response: Response, message: string) {
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
