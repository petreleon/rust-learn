import { TeacherCourseEnrollmentsRoute } from "@/features/teacher/course-enrollments/route/TeacherCourseEnrollmentsRoute";

export default async function TeachingCourseEnrollmentsPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseEnrollmentsRoute courseId={courseId} />;
}
