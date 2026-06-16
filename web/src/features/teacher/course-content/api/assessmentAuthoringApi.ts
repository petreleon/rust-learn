import {
  createTeacherAssessment,
  updateTeacherAssessment,
  type TeacherAssessment,
  type TeacherAssessmentPayload,
} from "@/lib/teacher";

export function createTeacherCourseAssessment({
  courseId,
  payload,
  token,
}: {
  courseId: string;
  payload: TeacherAssessmentPayload;
  token: string;
}): Promise<TeacherAssessment> {
  return createTeacherAssessment({ courseId, payload, token });
}

export function updateTeacherCourseAssessment({
  assessmentId,
  courseId,
  payload,
  token,
}: {
  assessmentId: number;
  courseId: string;
  payload: TeacherAssessmentPayload;
  token: string;
}): Promise<TeacherAssessment> {
  return updateTeacherAssessment({ assessmentId, courseId, payload, token });
}
