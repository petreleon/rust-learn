import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type TeacherCourseWorkspaceOptions = TeacherRequestOptions & {
  courseId: number | string;
};
