import { OrganizationDashboardRoute } from "@/components/organization-routes";

export default async function OrganizationDashboardPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationDashboardRoute organizationId={organizationId} />;
}
