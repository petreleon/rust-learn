import { type TeacherCourseJoinRequestItem } from "./TeacherCourseJoinRequestItem";

export type TeacherCourseJoinRequestPage = {
  limit: number;
  offset: number;
  requests: TeacherCourseJoinRequestItem[];
  status: string | null;
  total: number;
};
