import { downloadPlatformCsv } from "@/lib/admin/downloadPlatformCsv";
import { fetchPlatformSummary } from "@/lib/admin/fetchPlatformSummary";
import { fetchPlatformWalletReconciliation } from "@/lib/admin/fetchPlatformWalletReconciliation";
import { type PlatformCsvDownload } from "@/lib/admin/PlatformCsvDownload";
import { type PlatformReportSummary } from "@/lib/admin/PlatformReportSummary";
import { type PlatformWalletReconciliation } from "@/lib/admin/PlatformWalletReconciliation";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";

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
