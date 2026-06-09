import { OrganizationTeacherApplicationsRoute } from "@/components/organization-routes";

export default async function OrganizationTeacherApplicationsPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationTeacherApplicationsRoute organizationId={organizationId} />;
}
