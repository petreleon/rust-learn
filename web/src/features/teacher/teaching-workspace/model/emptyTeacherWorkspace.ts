import { type TeacherApplicationSnapshot } from "@/lib/teacher/TeacherApplicationSnapshot";
import { type TeacherCoursesResponse } from "@/lib/teacher/TeacherCoursesResponse";

export const emptyApplicationSnapshot: TeacherApplicationSnapshot = {
  application: null,
  audit_events: [],
};

export const emptyCourses: TeacherCoursesResponse = {
  courses: [],
  lifecycle_status: null,
  limit: 25,
  offset: 0,
  search: null,
  total: 0,
};
