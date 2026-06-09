import { OrganizationWalletRoute } from "@/components/organization-routes";

export default async function OrganizationWalletPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationWalletRoute organizationId={organizationId} />;
}
