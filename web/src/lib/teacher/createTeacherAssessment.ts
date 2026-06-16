import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherAssessmentCreateOptions } from "./TeacherAssessmentOptions";
import { type TeacherAssessment } from "./TeacherAssessment";

export async function createTeacherAssessment({
  apiRoot = "/api",
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherAssessmentCreateOptions): Promise<TeacherAssessment> {
  return teacherJsonRequest<TeacherAssessment>({
    body: JSON.stringify(payload),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/assessments/authoring`,
  });
}
