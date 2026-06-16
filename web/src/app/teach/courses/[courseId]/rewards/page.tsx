import { TeacherCourseRewardsRoute } from "@/features/teacher/course-rewards/route/TeacherCourseRewardsRoute";

export default async function TeachingCourseRewardsPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseRewardsRoute courseId={courseId} />;
}
