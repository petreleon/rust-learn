import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherCourseResponse } from "./TeacherCourseResponse";
import { type UpdateTeacherCourseOptions } from "./UpdateTeacherCourseOptions";

export async function updateTeacherCourse({
  apiRoot = "/api",
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: UpdateTeacherCourseOptions): Promise<TeacherCourseResponse> {
  return teacherJsonRequest<TeacherCourseResponse>({
    body: JSON.stringify(payload),
    method: "PUT",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}`,
  });
}
