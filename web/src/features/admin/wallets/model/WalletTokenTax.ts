export type WalletTokenTaxOperation = "deposit" | "retire";

export type WalletTokenTax = {
  operation: WalletTokenTaxOperation;
  tax_amount: string;
};

export type WalletTokenTaxSettings = {
  deposit: WalletTokenTax;
  retire: WalletTokenTax;
};

export type WalletTokenTaxAuditEvent = {
  id: number;
  actor_user_id: number | null;
  operation: WalletTokenTaxOperation;
  previous_tax_amount: string;
  new_tax_amount: string;
  created_at: string;
};

export type WalletTokenTaxDraft = Record<WalletTokenTaxOperation, string>;
