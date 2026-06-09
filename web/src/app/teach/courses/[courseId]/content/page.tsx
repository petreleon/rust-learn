import { TeacherCourseContentRoute } from "@/components/teacher-routes";

export default async function TeachingCourseContentPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseContentRoute courseId={courseId} />;
}
