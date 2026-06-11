import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type DecideTeacherJoinRequestOptions } from "./DecideTeacherJoinRequestOptions";
import { type TeacherCourseJoinDecisionResponse } from "./TeacherCourseJoinDecisionResponse";

export async function decideTeacherJoinRequest({
  apiRoot = "/api",
  courseId,
  payload,
  requestId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: DecideTeacherJoinRequestOptions): Promise<TeacherCourseJoinDecisionResponse> {
  return teacherJsonRequest<TeacherCourseJoinDecisionResponse>({
    body: JSON.stringify(payload),
    method: "PUT",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/join-requests/${requestId}/decision`,
  });
}
