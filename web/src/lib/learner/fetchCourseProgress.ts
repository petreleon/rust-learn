import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type CourseDetailOptions } from "./CourseDetailOptions";
import { type CourseProgress } from "./CourseProgress";

export async function fetchCourseProgress({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions): Promise<CourseProgress> {
  return learnerJsonRequest<CourseProgress>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/progress`,
  });
}
