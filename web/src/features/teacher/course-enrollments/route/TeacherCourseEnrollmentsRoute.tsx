"use client";

import { TeacherCourseEnrollmentsView } from "../view/TeacherCourseEnrollmentsView";
import { useTeacherCourseEnrollmentsRoute } from "./useTeacherCourseEnrollmentsRoute";

export function TeacherCourseEnrollmentsRoute({ courseId }: { courseId: string }) {
  const route = useTeacherCourseEnrollmentsRoute(courseId);
  return <TeacherCourseEnrollmentsView courseId={courseId} route={route} />;
}
