import { type OrganizationCourseList } from "@/lib/organization/OrganizationCourseList";

export function organizationCourseCounts(courses: OrganizationCourseList, page: number) {
  return {
    activePolicyCount: courses.courses.reduce((sum, course) => sum + course.rewards.active_policy_count, 0),
    canGoBack: courses.offset > 0,
    canGoForward: courses.offset + courses.limit < courses.total,
    pageLabel: `Page ${page + 1} of ${Math.max(1, Math.ceil(courses.total / courses.limit))}`,
    pendingJoinCount: courses.courses.reduce((sum, course) => sum + course.roster.pending_join_request_count, 0),
    pendingRewardCount: courses.courses.reduce((sum, course) => sum + course.reward_queue.pending_teacher_count, 0),
    rangeLabel:
      courses.total === 0
        ? "0 courses"
        : `${courses.offset + 1}-${Math.min(courses.offset + courses.limit, courses.total)} of ${courses.total}`,
  };
}
