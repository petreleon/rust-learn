export type WalletTokenTaxOperation = "deposit" | "retire";

export type WalletTokenTax = {
  operation: WalletTokenTaxOperation;
  tax_amount: string;
};

export type WalletTokenTaxSettings = {
  deposit: WalletTokenTax;
  retire: WalletTokenTax;
};

export type WalletTokenTaxDraft = Record<WalletTokenTaxOperation, string>;
