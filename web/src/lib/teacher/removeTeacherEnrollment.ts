import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type RemoveTeacherEnrollmentOptions } from "./RemoveTeacherEnrollmentOptions";
import { type TeacherEnrollmentRemovalResponse } from "./TeacherEnrollmentRemovalResponse";

export async function removeTeacherEnrollment({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
  userId,
}: RemoveTeacherEnrollmentOptions): Promise<TeacherEnrollmentRemovalResponse> {
  return teacherJsonRequest<TeacherEnrollmentRemovalResponse>({
    method: "DELETE",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/enrollments/${userId}`,
  });
}
