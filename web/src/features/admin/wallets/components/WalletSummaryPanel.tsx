"use client";

import { WalletCards } from "lucide-react";
import { type PlatformReportSummary } from "@/lib/admin/PlatformReportSummary";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { PanelError } from "@/features/admin/shared/route-kit/PanelError";
import { PanelLoading } from "@/features/admin/shared/route-kit/PanelLoading";
import { SummaryCard } from "@/features/admin/shared/route-kit/SummaryCard";

export function WalletSummaryPanel({
  error,
  onRetry,
  state,
  summary,
}: {
  error: RouteError | null;
  onRetry: () => void;
  state: LoadState;
  summary: PlatformReportSummary | null;
}) {
  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading wallet summary" />;
  }

  if (state === "error" || !summary) {
    return <PanelError error={error} onRetry={onRetry} title="Wallet summary failed" />;
  }

  return (
    <section className={styles.summaryGrid} aria-label="Wallet summary">
      <SummaryCard icon={<WalletCards size={20} aria-hidden />} label="Total wallets" value={summary.total_wallets} />
    </section>
  );
}
