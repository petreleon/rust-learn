import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export type TeacherCoursesOptions = TeacherRequestOptions & {
  lifecycleStatus?: string;
  limit?: number;
  offset?: number;
  search?: string;
};
