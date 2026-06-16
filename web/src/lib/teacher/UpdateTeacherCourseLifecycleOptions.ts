import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type UpdateTeacherCourseLifecycleOptions = TeacherRequestOptions & {
  courseId: number | string;
  status: string;
};
