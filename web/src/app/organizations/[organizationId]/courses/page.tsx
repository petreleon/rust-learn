import { OrganizationCoursesRoute } from "@/components/organization-routes";

export default async function OrganizationCoursesPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationCoursesRoute organizationId={organizationId} />;
}
