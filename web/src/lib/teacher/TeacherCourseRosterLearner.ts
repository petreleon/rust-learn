import { type TeacherEnrollmentUserSummary } from "./TeacherEnrollmentUserSummary";

export type TeacherCourseRosterLearner = {
  access_state: string;
  can_remove: boolean;
  latest_join_request_status: string | null;
  progress_supported: boolean;
  reward_eligibility_supported: boolean;
  roles: string[];
  user: TeacherEnrollmentUserSummary;
};
