"use client";

import { AlertTriangle, ArrowLeft, CheckCircle2, Download, Loader2, RefreshCw } from "lucide-react";
import Link from "next/link";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import { type CsvState } from "../model/CsvState";

export function ReportDashboardHero({
  csvError,
  csvState,
  dateFrom,
  dateTo,
  onDateFromChange,
  onDateToChange,
  onDownloadCsv,
  onRefresh,
  organization,
}: {
  csvError: RouteError | null;
  csvState: CsvState;
  dateFrom: string;
  dateTo: string;
  onDateFromChange: (value: string) => void;
  onDateToChange: (value: string) => void;
  onDownloadCsv: () => void;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  return (
    <section className={styles.workspaceHero}>
      <div className={styles.workspaceTitleBlock}>
        <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
          <ArrowLeft size={17} aria-hidden />
          {organization.name}
        </Link>
        <p className={styles.eyebrow}>Reward reports</p>
        <h2>Reward dashboard</h2>
        <p className={styles.muted}>
          All-time snapshot by default. Use date filters to narrow the window. Pagination, payout-failure drill-downs, and
          reconciliation rows remain open report-contract work.
        </p>
      </div>
      <div className={styles.filterPanel} style={{ marginTop: "0.75rem" }}>
        <label>
          <span>From</span>
          <input onChange={(event) => onDateFromChange(event.target.value)} type="date" value={dateFrom} />
        </label>
        <label>
          <span>To</span>
          <input onChange={(event) => onDateToChange(event.target.value)} type="date" value={dateTo} />
        </label>
      </div>
      <div className={styles.actionRow}>
        <button className={styles.secondaryButton} onClick={onRefresh} type="button">
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
        <button className={styles.primaryButton} disabled={csvState === "downloading"} onClick={onDownloadCsv} type="button">
          {csvState === "downloading" ? <Loader2 className={styles.spin} size={17} aria-hidden /> : <Download size={17} aria-hidden />}
          Export CSV
        </button>
      </div>
      {csvState === "success" ? <StatusPill icon={<CheckCircle2 size={16} aria-hidden />} label="CSV ready" tone="good" /> : null}
      {csvError ? (
        <section className={styles.inlineError} role="status">
          <AlertTriangle size={17} aria-hidden />
          <span>{csvError.message}</span>
        </section>
      ) : null}
    </section>
  );
}
