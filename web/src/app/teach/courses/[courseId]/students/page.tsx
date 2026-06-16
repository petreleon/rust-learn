import { TeacherCourseStudentsRoute } from "@/features/teacher/course-students/route/TeacherCourseStudentsRoute";

export default async function TeachingCourseStudentsPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseStudentsRoute courseId={courseId} />;
}
