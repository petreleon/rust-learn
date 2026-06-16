import { type TeacherRequestOptions } from "./TeacherRequestOptions";
import { type UpdateTeacherCoursePayload } from "./UpdateTeacherCoursePayload";

export type UpdateTeacherCourseOptions = TeacherRequestOptions & {
  courseId: number | string;
  payload: UpdateTeacherCoursePayload;
};
