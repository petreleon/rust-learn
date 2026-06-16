import { OrganizationWalletRoute } from "@/features/organization/wallet/route/OrganizationWalletRoute";

export default async function OrganizationWalletPage({
  params,
}: {
  params: Promise<{ organizationId: string }>;
}) {
  const { organizationId } = await params;
  return <OrganizationWalletRoute organizationId={organizationId} />;
}
