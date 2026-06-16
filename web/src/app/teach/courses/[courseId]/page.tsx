import { TeacherCourseWorkspaceRoute } from "@/features/teacher/course-workspace/route/TeacherCourseWorkspaceRoute";

export default async function TeachingCourseWorkspacePage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseWorkspaceRoute courseId={courseId} />;
}
