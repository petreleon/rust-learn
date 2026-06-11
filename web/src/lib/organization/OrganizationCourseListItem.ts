import { type OrganizationCourseContentSummary } from "./OrganizationCourseContentSummary";
import { type OrganizationCoursePermissionSummary } from "./OrganizationCoursePermissionSummary";
import { type OrganizationCourseRewardQueueSummary } from "./OrganizationCourseRewardQueueSummary";
import { type OrganizationCourseRewardSummary } from "./OrganizationCourseRewardSummary";
import { type OrganizationCourseRosterSummary } from "./OrganizationCourseRosterSummary";
import { type OrganizationCourseTeacher } from "./OrganizationCourseTeacher";

export type OrganizationCourseListItem = {
  content: OrganizationCourseContentSummary;
  id: number;
  lifecycle_status: string;
  permissions: OrganizationCoursePermissionSummary;
  reward_queue: OrganizationCourseRewardQueueSummary;
  rewards: OrganizationCourseRewardSummary;
  roster: OrganizationCourseRosterSummary;
  teachers: OrganizationCourseTeacher[];
  title: string;
};
