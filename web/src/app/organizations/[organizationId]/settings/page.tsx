import { OrganizationSettingsRoute } from "@/components/organization-routes";

export default async function OrganizationSettingsPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationSettingsRoute organizationId={organizationId} />;
}
