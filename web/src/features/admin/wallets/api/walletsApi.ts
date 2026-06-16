import { downloadPlatformCsv } from "@/lib/admin/downloadPlatformCsv";
import { adminJsonRequest } from "@/lib/admin/adminJsonRequest";
import { fetchPlatformSummary } from "@/lib/admin/fetchPlatformSummary";
import { fetchPlatformWalletReconciliation } from "@/lib/admin/fetchPlatformWalletReconciliation";
import { type PlatformCsvDownload } from "@/lib/admin/PlatformCsvDownload";
import { type PlatformReportSummary } from "@/lib/admin/PlatformReportSummary";
import { type PlatformWalletReconciliation } from "@/lib/admin/PlatformWalletReconciliation";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type WalletTokenTax, type WalletTokenTaxOperation, type WalletTokenTaxSettings } from "../model/WalletTokenTax";

export function loadAdminWalletSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function loadAdminWalletSummary({ token }: { token: string }): Promise<PlatformReportSummary> {
  return fetchPlatformSummary({ token });
}

export function loadAdminWalletReconciliation({
  token,
}: {
  token: string;
}): Promise<PlatformWalletReconciliation> {
  return fetchPlatformWalletReconciliation({ token });
}

export function downloadWalletCreditsCsv({ token }: { token: string }): Promise<PlatformCsvDownload> {
  return downloadPlatformCsv({ report: "wallet_credits", token });
}

export function loadWalletTokenTaxes({ token }: { token: string }): Promise<WalletTokenTaxSettings> {
  return adminJsonRequest({ apiRoot: "/api", path: "/wallets/token-taxes", token });
}

export function saveWalletTokenTax({
  operation,
  taxAmount,
  token,
}: {
  operation: WalletTokenTaxOperation;
  taxAmount: string;
  token: string;
}): Promise<WalletTokenTax> {
  return adminJsonRequest({
    apiRoot: "/api",
    body: JSON.stringify({ tax_amount: taxAmount }),
    method: "PUT",
    path: `/wallets/token-taxes/${operation}`,
    token,
  });
}
