import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherCourseEnrollmentOptions } from "./TeacherCourseEnrollmentOptions";
import { type TeacherCourseEnrollmentWorkspaceResponse } from "./TeacherCourseEnrollmentWorkspaceResponse";

export async function fetchTeachingCourseEnrollments({
  apiRoot = "/api",
  courseId,
  limit = 25,
  offset,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherCourseEnrollmentOptions): Promise<TeacherCourseEnrollmentWorkspaceResponse> {
  const query = new URLSearchParams();
  query.set("limit", String(limit));
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  const trimmedStatus = status?.trim();
  if (trimmedStatus) {
    query.set("status", trimmedStatus);
  }

  const suffix = query.toString();
  return teacherJsonRequest<TeacherCourseEnrollmentWorkspaceResponse>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/teaching/${courseId}/enrollments${suffix ? `?${suffix}` : ""}`,
  });
}
