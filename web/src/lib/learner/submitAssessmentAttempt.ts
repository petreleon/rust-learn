import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type AssessmentAttemptResult } from "./AssessmentAttemptResult";
import { type CourseDetailOptions } from "./CourseDetailOptions";

export async function submitAssessmentAttempt({
  apiRoot = "/api",
  answers,
  assessmentId,
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions & { assessmentId: number; answers: Record<number, string> }): Promise<AssessmentAttemptResult> {
  return learnerJsonRequest<AssessmentAttemptResult>({
    body: JSON.stringify({ answers }),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/assessments/${assessmentId}/submit`,
  });
}
