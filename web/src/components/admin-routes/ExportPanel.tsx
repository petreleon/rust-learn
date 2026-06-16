"use client";

import { Download, Loader2 } from "lucide-react";
import { type PlatformAdminWorkspace, type PlatformCsvReport } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { GatedPanel } from "./GatedPanel";
import { exportReports } from "./exportReports";
import { getCapability } from "./getCapability";
import { type CsvState } from "./CsvState";

export function ExportPanel({
  canExportData,
  csvState,
  onDownload,
  workspace,
}: {
  canExportData: boolean;
  csvState: CsvState;
  onDownload: (report: PlatformCsvReport, label: string) => void;
  workspace: PlatformAdminWorkspace;
}) {
  if (!canExportData) {
    return (
      <GatedPanel
        capability={getCapability(workspace, "exports")}
        icon={<Download size={20} aria-hidden />}
        title="CSV exports unavailable"
      />
    );
  }

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Download size={20} aria-hidden />
        <div>
          <h2>CSV exports</h2>
          <p>Downloads use the platform export endpoints and keep failures visible on the page.</p>
        </div>
      </div>
      <div className={styles.exportList}>
        {exportReports.map((report) => (
          <article className={styles.exportRow} key={report.report}>
            <div>
              <strong>{report.label}</strong>
              <span>{report.description}</span>
            </div>
            <button
              className={styles.secondaryButton}
              disabled={csvState === "downloading"}
              onClick={() => onDownload(report.report, report.label)}
              type="button"
            >
              {csvState === "downloading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Download size={16} aria-hidden />}
              CSV
            </button>
          </article>
        ))}
      </div>
    </section>
  );
}
