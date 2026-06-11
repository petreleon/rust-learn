import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherCourseStudentsResponse } from "./TeacherCourseStudentsResponse";
import { type TeacherCourseWorkspaceOptions } from "./TeacherCourseWorkspaceOptions";

export async function fetchTeachingCourseStudents({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherCourseWorkspaceOptions): Promise<TeacherCourseStudentsResponse> {
  return teacherJsonRequest<TeacherCourseStudentsResponse>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/teaching/${courseId}/students`,
  });
}
