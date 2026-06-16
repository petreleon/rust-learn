"use client";

import { TeacherCourseContentView } from "../view/TeacherCourseContentView";
import { useTeacherCourseContentRoute } from "./useTeacherCourseContentRoute";

export function TeacherCourseContentRoute({ courseId }: { courseId: string }) {
  const route = useTeacherCourseContentRoute(courseId);

  return <TeacherCourseContentView courseId={courseId} route={route} />;
}
