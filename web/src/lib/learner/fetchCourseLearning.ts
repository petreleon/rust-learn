import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type CourseDetailOptions } from "./CourseDetailOptions";
import { type CourseLearningResponse } from "./CourseLearningResponse";

export async function fetchCourseLearning({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions): Promise<CourseLearningResponse> {
  return learnerJsonRequest<CourseLearningResponse>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/catalog/${courseId}/learn`,
  });
}
