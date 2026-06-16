import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherAssessmentListOptions } from "./TeacherAssessmentOptions";
import { type TeacherAssessment } from "./TeacherAssessment";

export async function fetchTeacherAssessments({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherAssessmentListOptions): Promise<TeacherAssessment[]> {
  return teacherJsonRequest<TeacherAssessment[]>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/assessments/authoring`,
  });
}
