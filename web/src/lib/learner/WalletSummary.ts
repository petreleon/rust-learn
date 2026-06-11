export type WalletSummary = {
  id: number;
  organization_id: number | null;
  owner_type: "user" | "organization";
  user_id: number | null;
  value: string;
};
