"use client";

import { type OrganizationWalletAudit } from "@/lib/organization";

export function tokenExplorerUrl(transaction: OrganizationWalletAudit["external_transactions"][number]) {
  if (!transaction.transaction_hash) {
    return null;
  }

  if (transaction.chain_id === 1 || transaction.chain_id === null) {
    return `https://etherscan.io/tx/${transaction.transaction_hash}`;
  }

  if (transaction.chain_id === 11155111) {
    return `https://sepolia.etherscan.io/tx/${transaction.transaction_hash}`;
  }

  return null;
}
