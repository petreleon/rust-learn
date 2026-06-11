import { type OrganizationWalletCompensationRecordAudit } from "./OrganizationWalletCompensationRecordAudit";
import { type OrganizationWalletExternalTransactionAudit } from "./OrganizationWalletExternalTransactionAudit";
import { type OrganizationWalletInternalTransactionAudit } from "./OrganizationWalletInternalTransactionAudit";
import { type OrganizationWalletRewardRecordAudit } from "./OrganizationWalletRewardRecordAudit";
import { type OrganizationWalletSummary } from "./OrganizationWalletSummary";

export type OrganizationWalletAudit = {
  compensation_records: OrganizationWalletCompensationRecordAudit[];
  external_transactions: OrganizationWalletExternalTransactionAudit[];
  internal_transactions: OrganizationWalletInternalTransactionAudit[];
  reward_records: OrganizationWalletRewardRecordAudit[];
  wallet: OrganizationWalletSummary;
};
