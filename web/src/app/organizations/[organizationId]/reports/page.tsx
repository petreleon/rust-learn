import { OrganizationReportsRoute } from "@/features/organization/reports/route/OrganizationReportsRoute";

export default async function OrganizationReportsPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationReportsRoute organizationId={organizationId} />;
}
