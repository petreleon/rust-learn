import { type TeacherCourseDashboardItem } from "./TeacherCourseDashboardItem";
import { type TeacherCoursePublicationSummary } from "./TeacherCoursePublicationSummary";
import { type TeacherCourseWorkspaceChapter } from "./TeacherCourseWorkspaceChapter";

export type TeacherCourseWorkspaceResponse = {
  chapters: TeacherCourseWorkspaceChapter[];
  course: TeacherCourseDashboardItem;
  publication: TeacherCoursePublicationSummary;
  teacher_roles: string[];
};
