import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherCourseResponse } from "./TeacherCourseResponse";
import { type UpdateTeacherCourseLifecycleOptions } from "./UpdateTeacherCourseLifecycleOptions";

export async function updateTeacherCourseLifecycle({
  apiRoot = "/api",
  courseId,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: UpdateTeacherCourseLifecycleOptions): Promise<TeacherCourseResponse> {
  return teacherJsonRequest<TeacherCourseResponse>({
    body: JSON.stringify({ status }),
    method: "PUT",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/lifecycle`,
  });
}
