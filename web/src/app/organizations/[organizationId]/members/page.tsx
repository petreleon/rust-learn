import { OrganizationMembersRoute } from "@/components/organization-routes";

export default async function OrganizationMembersPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationMembersRoute organizationId={organizationId} />;
}
