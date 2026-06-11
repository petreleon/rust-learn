import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type CourseDetailOptions } from "./CourseDetailOptions";
import { type CourseJoinRequest } from "./CourseJoinRequest";

export async function requestCourseJoin({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions): Promise<CourseJoinRequest> {
  return learnerJsonRequest<CourseJoinRequest>({
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/join-requests`,
  });
}
