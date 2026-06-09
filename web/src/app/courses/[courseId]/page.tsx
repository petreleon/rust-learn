import { LearnerCourseDetailRoute } from "@/components/learner-routes";

export default async function CourseDetailPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <LearnerCourseDetailRoute courseId={courseId} />;
}
