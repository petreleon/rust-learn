import { OrganizationDashboardRoute } from "@/features/organization/dashboard/route/OrganizationDashboardRoute";

export default async function OrganizationDashboardPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationDashboardRoute organizationId={organizationId} />;
}
