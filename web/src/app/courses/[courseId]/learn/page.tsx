import { LearnerCourseLearnRoute } from "@/features/learner/workspace/components/LearnerCourseLearnRoute";

export default async function CourseLearnPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <LearnerCourseLearnRoute courseId={courseId} />;
}
