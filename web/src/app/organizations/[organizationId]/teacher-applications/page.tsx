import { OrganizationTeacherApplicationsRoute } from "@/features/organization/teacher-applications/route/OrganizationTeacherApplicationsRoute";

export default async function OrganizationTeacherApplicationsPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationTeacherApplicationsRoute organizationId={organizationId} />;
}
