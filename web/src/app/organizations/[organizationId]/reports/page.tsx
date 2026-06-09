import { OrganizationReportsRoute } from "@/components/organization-routes";

export default async function OrganizationReportsPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationReportsRoute organizationId={organizationId} />;
}
