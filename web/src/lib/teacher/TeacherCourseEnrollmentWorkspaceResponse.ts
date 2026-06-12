import { type TeacherCourseDashboardItem } from "./TeacherCourseDashboardItem";
import { type TeacherCourseJoinRequestPage } from "./TeacherCourseJoinRequestPage";
import { type TeacherCourseRewardEligibilitySummary } from "./TeacherCourseRewardEligibilitySummary";
import { type TeacherCourseRosterPage } from "./TeacherCourseRosterPage";

export type TeacherCourseEnrollmentWorkspaceResponse = {
  course: TeacherCourseDashboardItem;
  join_requests: TeacherCourseJoinRequestPage;
  progress_supported: boolean;
  reward_eligibility: TeacherCourseRewardEligibilitySummary;
  reward_eligibility_supported: boolean;
  roster: TeacherCourseRosterPage;
  teacher_roles: string[];
};
