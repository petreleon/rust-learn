"use client";

import { Download } from "lucide-react";
import styles from "../ops-console.module.css";
import { PermissionNotice } from "./PermissionNotice";
import { RequirementNotice } from "./RequirementNotice";
import {
  OrganizationReportControls,
  PlatformDashboardControls,
  PlatformExportControls,
  PlatformSummaryControl,
  ReportDeniedNotices,
} from "./ReportWorkflowControls";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function ReportWorkflowPanel({ controller }: { controller: OpsConsoleController }) {
  const { auth, report } = controller;

  return (
    <section id="report-workflow" className={styles.panel} aria-labelledby="report-title">
      <div className={styles.panelHeader}>
        <div>
          <p className={styles.eyebrow}>Organization</p>
          <h2 id="report-title">Reward report</h2>
        </div>
        <Download size={22} aria-hidden />
      </div>
      {!report.canUseReportWorkflow && (
        <PermissionNotice
          title="Reporting permissions disabled"
          detail="Enable summary, organization reward, reward audit, generate report, or export permissions to use reporting actions."
        />
      )}
      {auth.hasSessionToken && report.canUseOrganizationReportControls && report.hasOrganizationReportDraft && (
        <RequirementNotice action="Load organization reports" fields={report.organizationReportMissingFields} />
      )}
      <ReportDeniedNotices controller={controller} />
      {report.canUseOrganizationReportControls && <OrganizationReportControls controller={controller} />}
      {report.canExport && <PlatformExportControls controller={controller} />}
      {report.canViewSummaryReports && <PlatformSummaryControl controller={controller} />}
      {report.canViewPlatformDashboards && <PlatformDashboardControls controller={controller} />}
    </section>
  );
}
