import { OrganizationMembersRoute } from "@/features/organization/members/route/OrganizationMembersRoute";

export default async function OrganizationMembersPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationMembersRoute organizationId={organizationId} />;
}
