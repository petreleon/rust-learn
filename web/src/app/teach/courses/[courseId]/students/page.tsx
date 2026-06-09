import { TeacherCourseStudentsRoute } from "@/components/teacher-routes";

export default async function TeachingCourseStudentsPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseStudentsRoute courseId={courseId} />;
}
