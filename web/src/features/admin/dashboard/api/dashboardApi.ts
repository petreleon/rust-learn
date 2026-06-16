import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { downloadPlatformCsv } from "@/lib/admin/downloadPlatformCsv";
import { fetchPlatformFraudDashboard } from "@/lib/admin/fetchPlatformFraudDashboard";
import { fetchPlatformRewardDashboard } from "@/lib/admin/fetchPlatformRewardDashboard";
import { fetchPlatformSummary } from "@/lib/admin/fetchPlatformSummary";
import { fetchPlatformSystemStatus } from "@/lib/admin/fetchPlatformSystemStatus";
import { type PlatformCsvDownload } from "@/lib/admin/PlatformCsvDownload";
import { type PlatformCsvReport } from "@/lib/admin/PlatformCsvReport";
import { type PlatformFraudDashboard } from "@/lib/admin/PlatformFraudDashboard";
import { type PlatformReportSummary } from "@/lib/admin/PlatformReportSummary";
import { type PlatformRewardDashboard } from "@/lib/admin/PlatformRewardDashboard";
import { type PlatformSystemStatus } from "@/lib/admin/PlatformSystemStatus";

export function loadAdminDashboardSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function loadPlatformSummary({ token }: { token: string }): Promise<PlatformReportSummary> {
  return fetchPlatformSummary({ token });
}

export function loadPlatformRewardOperations({ token }: { token: string }): Promise<PlatformRewardDashboard> {
  return fetchPlatformRewardDashboard({ token });
}

export function loadPlatformFraudControls({ token }: { token: string }): Promise<PlatformFraudDashboard> {
  return fetchPlatformFraudDashboard({ token });
}

export function loadPlatformSystemStatus(): Promise<PlatformSystemStatus> {
  return fetchPlatformSystemStatus();
}

export function downloadAdminDashboardCsv({
  report,
  token,
}: {
  report: PlatformCsvReport;
  token: string;
}): Promise<PlatformCsvDownload> {
  return downloadPlatformCsv({ report, token });
}
