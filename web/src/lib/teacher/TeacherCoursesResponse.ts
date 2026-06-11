import { type TeacherCourseDashboardItem } from "./TeacherCourseDashboardItem";

export type TeacherCoursesResponse = {
  courses: TeacherCourseDashboardItem[];
  lifecycle_status: string | null;
  limit: number;
  offset: number;
  search: string | null;
  total: number;
};
