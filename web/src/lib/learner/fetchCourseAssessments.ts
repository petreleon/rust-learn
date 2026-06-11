import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type AssessmentItem } from "./AssessmentItem";
import { type CourseDetailOptions } from "./CourseDetailOptions";

export async function fetchCourseAssessments({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions): Promise<AssessmentItem[]> {
  return learnerJsonRequest<AssessmentItem[]>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/assessments`,
  });
}
