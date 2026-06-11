import { type SubmitTeacherApplicationPayload } from "./SubmitTeacherApplicationPayload";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type SubmitTeacherApplicationOptions = TeacherRequestOptions & {
  payload: SubmitTeacherApplicationPayload;
};
