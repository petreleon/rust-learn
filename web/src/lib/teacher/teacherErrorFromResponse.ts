import { TeacherRequestError } from "./TeacherRequestError";
import { type TeacherErrorEnvelope } from "./TeacherErrorEnvelope";

export async function teacherErrorFromResponse(response: Response, fallbackMessage: string) {
  const fallbackCode =
    response.status === 401
      ? "unauthorized"
      : response.status === 403
        ? "permission_denied"
        : response.status === 404
          ? "not_found"
          : response.status === 409
            ? "conflict"
            : response.status >= 500
              ? "server_error"
              : "teacher_error";
  const contentType = response.headers.get("content-type") || "";

  if (contentType.includes("application/json")) {
    const body = (await response.json()) as TeacherErrorEnvelope;
    return new TeacherRequestError(
      body.error?.message || fallbackMessage,
      response.status,
      body.error?.code || fallbackCode,
    );
  }

  const text = await response.text();
  return new TeacherRequestError(text || response.statusText || fallbackMessage, response.status, fallbackCode);
}
