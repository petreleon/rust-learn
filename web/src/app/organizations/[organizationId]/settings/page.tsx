import { OrganizationSettingsRoute } from "@/features/organization/settings/route/OrganizationSettingsRoute";

export default async function OrganizationSettingsPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationSettingsRoute organizationId={organizationId} />;
}
