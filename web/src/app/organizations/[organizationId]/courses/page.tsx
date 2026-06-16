import { OrganizationCoursesRoute } from "@/features/organization/courses/route/OrganizationCoursesRoute";

export default async function OrganizationCoursesPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationCoursesRoute organizationId={organizationId} />;
}
