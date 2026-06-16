"use client";

import { TeacherCourseWorkspaceView } from "../view/TeacherCourseWorkspaceView";
import { useTeacherCourseWorkspaceRoute } from "./useTeacherCourseWorkspaceRoute";

export function TeacherCourseWorkspaceRoute({ courseId }: { courseId: string }) {
  const route = useTeacherCourseWorkspaceRoute(courseId);

  return <TeacherCourseWorkspaceView courseId={courseId} route={route} />;
}
