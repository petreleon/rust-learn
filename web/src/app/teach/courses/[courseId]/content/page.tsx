import { TeacherCourseContentRoute } from "@/features/teacher/course-content/route/TeacherCourseContentRoute";

export default async function TeachingCourseContentPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseContentRoute courseId={courseId} />;
}
