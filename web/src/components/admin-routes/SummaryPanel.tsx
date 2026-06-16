"use client";

import { FileText, Gauge, Landmark, Users, WalletCards } from "lucide-react";
import { type PlatformAdminWorkspace, type PlatformReportSummary } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { GatedPanel } from "./GatedPanel";
import { PanelError } from "./PanelError";
import { PanelLoading } from "./PanelLoading";
import { SummaryCard } from "./SummaryCard";
import { getCapability } from "./getCapability";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function SummaryPanel({
  canViewSummary,
  error,
  onRetry,
  state,
  summary,
  workspace,
}: {
  canViewSummary: boolean;
  error: RouteError | null;
  onRetry: () => void;
  state: SectionState;
  summary: PlatformReportSummary | null;
  workspace: PlatformAdminWorkspace;
}) {
  if (!canViewSummary) {
    return (
      <GatedPanel
        capability={getCapability(workspace, "summary")}
        icon={<Gauge size={20} aria-hidden />}
        title="Platform summary unavailable"
      />
    );
  }

  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading platform summary" />;
  }

  if (state === "error" || !summary) {
    return <PanelError error={error} onRetry={onRetry} title="Platform summary failed" />;
  }

  return (
    <section className={styles.summaryGrid} aria-label="Platform summary">
      <SummaryCard icon={<Users size={20} aria-hidden />} label="Users" value={summary.total_users} />
      <SummaryCard icon={<Landmark size={20} aria-hidden />} label="Organizations" value={summary.total_organizations} />
      <SummaryCard icon={<FileText size={20} aria-hidden />} label="Courses" value={summary.total_courses} />
      <SummaryCard icon={<WalletCards size={20} aria-hidden />} label="Wallets" value={summary.total_wallets} />
      <SummaryCard icon={<Gauge size={20} aria-hidden />} label="Notifications" value={summary.total_notifications} />
    </section>
  );
}
