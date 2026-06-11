import { type TeacherCourseDashboardItem } from "./TeacherCourseDashboardItem";
import { type TeacherCourseStudentProgressItem } from "./TeacherCourseStudentProgressItem";

export type TeacherCourseStudentsResponse = {
  course: TeacherCourseDashboardItem;
  progress_supported: boolean;
  reward_evidence_supported: boolean;
  students: TeacherCourseStudentProgressItem[];
  teacher_roles: string[];
  total: number;
};
