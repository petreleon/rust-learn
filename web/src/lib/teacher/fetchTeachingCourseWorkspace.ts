import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherCourseWorkspaceOptions } from "./TeacherCourseWorkspaceOptions";
import { type TeacherCourseWorkspaceResponse } from "./TeacherCourseWorkspaceResponse";

export async function fetchTeachingCourseWorkspace({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherCourseWorkspaceOptions): Promise<TeacherCourseWorkspaceResponse> {
  return teacherJsonRequest<TeacherCourseWorkspaceResponse>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/teaching/${courseId}`,
  });
}
