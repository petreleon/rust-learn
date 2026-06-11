import { type TeacherCourseWorkspaceContent } from "./TeacherCourseWorkspaceContent";

export type TeacherCourseWorkspaceChapter = {
  contents: TeacherCourseWorkspaceContent[];
  id: number;
  order: number;
  title: string;
};
