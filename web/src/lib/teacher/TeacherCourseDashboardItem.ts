import { type TeacherCourseContentSummary } from "./TeacherCourseContentSummary";
import { type TeacherCourseOrganization } from "./TeacherCourseOrganization";
import { type TeacherCoursePermissionSummary } from "./TeacherCoursePermissionSummary";
import { type TeacherCourseRewardQueueSummary } from "./TeacherCourseRewardQueueSummary";
import { type TeacherCourseRewardSummary } from "./TeacherCourseRewardSummary";
import { type TeacherCourseRosterSummary } from "./TeacherCourseRosterSummary";

export type TeacherCourseDashboardItem = {
  content: TeacherCourseContentSummary;
  description: string | null;
  id: number;
  lifecycle_status: string;
  organizations: TeacherCourseOrganization[];
  prerequisites: string | null;
  permissions: TeacherCoursePermissionSummary;
  reward_queue: TeacherCourseRewardQueueSummary;
  rewards: TeacherCourseRewardSummary;
  roster: TeacherCourseRosterSummary;
  title: string;
  topics: string | null;
};
