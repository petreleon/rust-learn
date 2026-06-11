import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type RemoveTeacherEnrollmentOptions = TeacherRequestOptions & {
  courseId: number | string;
  userId: number | string;
};
