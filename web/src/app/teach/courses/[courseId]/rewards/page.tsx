import { TeacherCourseRewardsRoute } from "@/components/teacher-routes";

export default async function TeachingCourseRewardsPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <TeacherCourseRewardsRoute courseId={courseId} />;
}
