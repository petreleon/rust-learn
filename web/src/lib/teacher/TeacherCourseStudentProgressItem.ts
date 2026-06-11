import { type TeacherEnrollmentUserSummary } from "./TeacherEnrollmentUserSummary";
import { type TeacherStudentProgressSummary } from "./TeacherStudentProgressSummary";
import { type TeacherStudentRewardProgressSummary } from "./TeacherStudentRewardProgressSummary";

export type TeacherCourseStudentProgressItem = {
  access_state: string;
  latest_join_request_status: string | null;
  progress: TeacherStudentProgressSummary;
  rewards: TeacherStudentRewardProgressSummary;
  roles: string[];
  user: TeacherEnrollmentUserSummary;
};
