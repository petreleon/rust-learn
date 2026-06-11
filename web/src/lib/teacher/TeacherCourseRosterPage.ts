import { type TeacherCourseRosterLearner } from "./TeacherCourseRosterLearner";

export type TeacherCourseRosterPage = {
  learners: TeacherCourseRosterLearner[];
  total: number;
};
