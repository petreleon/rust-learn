import { LearnerCourseLearnRoute } from "@/components/learner-routes";

export default async function CourseLearnPage({
  params,
}: {
  params: Promise<{ courseId: string }>;
}) {
  const { courseId } = await params;
  return <LearnerCourseLearnRoute courseId={courseId} />;
}
