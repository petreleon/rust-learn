import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";

const unhealthyLifecycleStatuses = new Set(["archived", "needs_changes", "suspended"]);

export function dashboardTotals(courses: TeacherCourseDashboardItem[]) {
  return courses.reduce(
    (totals, course) => {
      const pendingEnrollmentCount =
        course.roster.pending_join_request_count + course.roster.waitlisted_join_request_count;
      totals.courseCount += 1;
      totals.contentCount += course.content.content_count;
      totals.pendingEnrollmentCount += pendingEnrollmentCount;
      totals.pendingRewardCount += course.reward_queue.pending_teacher_count;
      if (!course.content.has_content || unhealthyLifecycleStatuses.has(course.lifecycle_status)) {
        totals.unhealthyCourseCount += 1;
      }
      return totals;
    },
    {
      contentCount: 0,
      courseCount: 0,
      pendingEnrollmentCount: 0,
      pendingRewardCount: 0,
      unhealthyCourseCount: 0,
    },
  );
}

export type TeacherDashboardTotals = ReturnType<typeof dashboardTotals>;
