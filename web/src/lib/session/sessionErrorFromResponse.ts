import { SessionRequestError } from "./SessionRequestError";
import { type SessionErrorEnvelope } from "./SessionErrorEnvelope";

export async function sessionErrorFromResponse(response: Response) {
  const fallback = new SessionRequestError(
    response.statusText || "Session request failed.",
    response.status,
    response.status === 401 ? "unauthorized" : "session_error",
  );

  const contentType = response.headers.get("content-type") || "";
  if (!contentType.includes("application/json")) {
    const text = await response.text();
    return new SessionRequestError(text || fallback.message, response.status, fallback.code);
  }

  const body = (await response.json()) as SessionErrorEnvelope;
  return new SessionRequestError(
    body.error?.message || fallback.message,
    response.status,
    body.error?.code || fallback.code,
  );
}
