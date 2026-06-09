import { TeacherCourseEnrollmentsRoute } from "@/components/teacher-routes";

export default async function TeachingCourseEnrollmentsPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseEnrollmentsRoute courseId={courseId} />;
}
