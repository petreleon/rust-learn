import { TeacherRequestError } from "./TeacherRequestError";

export async function teacherRawRequest({
  body,
  method,
  timeoutMs,
  token,
  url,
}: {
  body?: string;
  method: string;
  timeoutMs: number;
  token: string;
  url: string;
}) {
  const trimmedToken = token.trim();
  if (!trimmedToken) {
    throw new TeacherRequestError("A sign-in token is required.", 401, "missing_token");
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    return await fetch(url, {
      body,
      headers: {
        Authorization: `Bearer ${trimmedToken}`,
        ...(body ? { "Content-Type": "application/json" } : {}),
      },
      method,
      signal: controller.signal,
    });
  } catch (error) {
    if (error instanceof DOMException && error.name === "AbortError") {
      throw new TeacherRequestError("Teacher request timed out.", 0, "timeout");
    }

    throw new TeacherRequestError("Teacher request failed before the API responded.", 0, "network_error");
  } finally {
    clearTimeout(timeout);
  }
}
