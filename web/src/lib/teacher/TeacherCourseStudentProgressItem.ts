import { type TeacherEnrollmentUserSummary } from "./TeacherEnrollmentUserSummary";
import { type TeacherStudentProgressSummary } from "./TeacherStudentProgressSummary";
import { type TeacherStudentRewardEligibilitySummary } from "./TeacherStudentRewardEligibilitySummary";
import { type TeacherStudentRewardProgressSummary } from "./TeacherStudentRewardProgressSummary";

export type TeacherCourseStudentProgressItem = {
  access_state: string;
  latest_join_request_status: string | null;
  progress: TeacherStudentProgressSummary;
  reward_eligibility: TeacherStudentRewardEligibilitySummary;
  rewards: TeacherStudentRewardProgressSummary;
  roles: string[];
  user: TeacherEnrollmentUserSummary;
};
