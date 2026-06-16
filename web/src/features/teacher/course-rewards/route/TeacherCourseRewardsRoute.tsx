"use client";

import { TeacherCourseRewardsView } from "../view/TeacherCourseRewardsView";
import { useTeacherCourseRewardsRoute } from "./useTeacherCourseRewardsRoute";

export function TeacherCourseRewardsRoute({ courseId }: { courseId: string }) {
  const route = useTeacherCourseRewardsRoute(courseId);

  return <TeacherCourseRewardsView courseId={courseId} route={route} />;
}
