import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type TeacherCourseEnrollmentOptions = TeacherRequestOptions & {
  courseId: number | string;
  limit?: number;
  offset?: number;
  status?: string;
};
