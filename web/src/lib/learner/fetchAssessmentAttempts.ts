import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type AssessmentAttemptResult } from "./AssessmentAttemptResult";
import { type CourseDetailOptions } from "./CourseDetailOptions";

export async function fetchAssessmentAttempts({
  apiRoot = "/api",
  assessmentId,
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions & { assessmentId: number }): Promise<AssessmentAttemptResult["attempt"][]> {
  return learnerJsonRequest<AssessmentAttemptResult["attempt"][]>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/assessments/${assessmentId}/attempts`,
  });
}
