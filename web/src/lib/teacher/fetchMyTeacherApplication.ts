import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherApplicationSnapshot } from "./TeacherApplicationSnapshot";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export async function fetchMyTeacherApplication({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherRequestOptions): Promise<TeacherApplicationSnapshot> {
  return teacherJsonRequest<TeacherApplicationSnapshot>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/teacher-applications/me`,
  });
}
