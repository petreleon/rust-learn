import { type TeacherRequestOptions } from "./TeacherRequestOptions";
import { type TeacherAssessmentPayload } from "./TeacherAssessmentPayload";

export type TeacherAssessmentListOptions = TeacherRequestOptions & {
  courseId: number | string;
};

export type TeacherAssessmentCreateOptions = TeacherAssessmentListOptions & {
  payload: TeacherAssessmentPayload;
};

export type TeacherAssessmentUpdateOptions = TeacherAssessmentCreateOptions & {
  assessmentId: number;
};
