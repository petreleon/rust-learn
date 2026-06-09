import { TeacherCourseWorkspaceRoute } from "@/components/teacher-routes";

export default async function TeachingCourseWorkspacePage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseWorkspaceRoute courseId={courseId} />;
}
