import { type CreateTeacherCoursePayload } from "./CreateTeacherCoursePayload";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type CreateTeacherCourseOptions = TeacherRequestOptions & {
  payload: CreateTeacherCoursePayload;
};
