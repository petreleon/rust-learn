import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type CreateTeacherCourseOptions } from "./CreateTeacherCourseOptions";
import { type TeacherCourseResponse } from "./TeacherCourseResponse";

export async function createTeacherCourse({
  apiRoot = "/api",
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CreateTeacherCourseOptions): Promise<TeacherCourseResponse> {
  return teacherJsonRequest<TeacherCourseResponse>({
    body: JSON.stringify(payload),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses`,
  });
}
