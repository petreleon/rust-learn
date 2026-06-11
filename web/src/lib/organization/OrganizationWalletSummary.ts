export type OrganizationWalletSummary = {
  id: number;
  organization_id: number | null;
  owner_type: "organization" | "user";
  user_id: number | null;
  value: string;
};
