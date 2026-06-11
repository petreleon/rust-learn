import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type CourseDetailOptions } from "./CourseDetailOptions";
import { type CourseProgress } from "./CourseProgress";

export async function saveCourseProgress({
  apiRoot = "/api",
  contentId,
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions & { contentId: number }): Promise<CourseProgress> {
  return learnerJsonRequest<CourseProgress>({
    body: JSON.stringify({ content_id: contentId }),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/progress`,
  });
}
