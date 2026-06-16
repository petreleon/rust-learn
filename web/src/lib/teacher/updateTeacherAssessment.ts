import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherAssessmentUpdateOptions } from "./TeacherAssessmentOptions";
import { type TeacherAssessment } from "./TeacherAssessment";

export async function updateTeacherAssessment({
  apiRoot = "/api",
  assessmentId,
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherAssessmentUpdateOptions): Promise<TeacherAssessment> {
  return teacherJsonRequest<TeacherAssessment>({
    body: JSON.stringify(payload),
    method: "PUT",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/assessments/${assessmentId}/authoring`,
  });
}
