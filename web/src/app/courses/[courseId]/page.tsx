import { LearnerCourseDetailRoute } from "@/features/learner/workspace/components/LearnerCourseDetailRoute";

export default async function CourseDetailPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <LearnerCourseDetailRoute courseId={courseId} />;
}
