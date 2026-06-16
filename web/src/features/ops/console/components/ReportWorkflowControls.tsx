"use client";

import { ClipboardList, Download, ShieldAlert } from "lucide-react";
import styles from "../ops-console.module.css";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { platformExportReports } from "../model/platformExportReports";
import { positiveIntegerInputProps } from "../model/positiveIntegerInputProps";
import { OpsServerDeniedNotice } from "./OpsServerDeniedNotice";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function ReportDeniedNotices({ controller }: { controller: OpsConsoleController }) {
  const { auth } = controller;

  return (
    <>
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.organizationSummary} action="Summary" />
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.organizationSummaryCsv} action="Summary CSV" />
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.organizationRewardReport} action="Reward report" />
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.organizationRewardCsv} action="Reward CSV" />
    </>
  );
}

export function OrganizationReportControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, report } = controller;

  return (
    <div className={styles.actionStrip}>
      <input
        aria-label="Report organization id"
        {...positiveIntegerInputProps}
        placeholder="Organization id"
        value={report.organizationId}
        onChange={(event) => report.setOrganizationId(event.target.value)}
      />
      {report.canViewSummaryReports && (
        <button
          type="button"
          className={styles.secondaryButton}
          onClick={() => report.loadOrganizationSummary(false)}
          {...auth.actionState(report.canLoadOrganizationReportForm, true, "Required permission", PROTECTED_ACTIONS.organizationSummary)}
        >
          <ClipboardList size={17} aria-hidden />
          <span>Summary</span>
        </button>
      )}
      {report.canGenerateReports && (
        <button
          type="button"
          className={styles.secondaryButton}
          onClick={() => report.loadOrganizationSummary(true)}
          {...auth.actionState(
            report.canLoadOrganizationReportForm,
            true,
            "Required permission",
            PROTECTED_ACTIONS.organizationSummaryCsv,
          )}
        >
          <Download size={17} aria-hidden />
          <span>Summary CSV</span>
        </button>
      )}
      {report.canViewOrgReports && <OrganizationRewardButtons controller={controller} />}
    </div>
  );
}

function OrganizationRewardButtons({ controller }: { controller: OpsConsoleController }) {
  const { auth, report } = controller;

  return (
    <>
      <button
        type="button"
        className={styles.secondaryButton}
        onClick={() => report.loadOrganizationReport(false)}
        {...auth.actionState(report.canLoadOrganizationReportForm, true, "Required permission", PROTECTED_ACTIONS.organizationRewardReport)}
      >
        <ClipboardList size={17} aria-hidden />
        <span>Reward report</span>
      </button>
      <button
        type="button"
        className={styles.secondaryButton}
        onClick={() => report.loadOrganizationReport(true)}
        {...auth.actionState(report.canLoadOrganizationReportForm, true, "Required permission", PROTECTED_ACTIONS.organizationRewardCsv)}
      >
        <Download size={17} aria-hidden />
        <span>Reward CSV</span>
      </button>
    </>
  );
}

export function PlatformExportControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, report } = controller;

  return (
    <>
      {platformExportReports.map((exportReport) => (
        <OpsServerDeniedNotice
          key={exportReport.path}
          auth={auth}
          actionKey={exportReport.resultLabel}
          action={exportReport.label}
        />
      ))}
      <div className={styles.reportLinks}>
        {platformExportReports.map((exportReport) => (
          <button
            key={exportReport.path}
            type="button"
            onClick={() => report.loadPlatformExport(exportReport.path, exportReport.resultLabel)}
            {...auth.actionState(true, true, "Required permission", exportReport.resultLabel)}
          >
            <Download size={16} aria-hidden />
            <span>{exportReport.label}</span>
          </button>
        ))}
      </div>
    </>
  );
}

export function PlatformSummaryControl({ controller }: { controller: OpsConsoleController }) {
  const { auth, report } = controller;

  return (
    <>
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.platformSummary} action="Platform summary" />
      <div className={styles.reportLinks}>
        <button
          type="button"
          onClick={() => report.loadPlatformExport("/reports/platform/summary", PROTECTED_ACTIONS.platformSummary)}
          {...auth.actionState(true, true, "Required permission", PROTECTED_ACTIONS.platformSummary)}
        >
          <ClipboardList size={16} aria-hidden />
          <span>Platform summary</span>
        </button>
      </div>
    </>
  );
}

export function PlatformDashboardControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, report } = controller;

  return (
    <>
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.platformRewardDashboard} action="Reward dashboard" />
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.platformFraudDashboard} action="Fraud dashboard" />
      <div className={styles.reportLinks}>
        <button
          type="button"
          onClick={() => report.loadPlatformExport("/reports/platform/reward-dashboard", PROTECTED_ACTIONS.platformRewardDashboard)}
          {...auth.actionState(true, true, "Required permission", PROTECTED_ACTIONS.platformRewardDashboard)}
        >
          <ClipboardList size={16} aria-hidden />
          <span>Reward dashboard</span>
        </button>
        <button
          type="button"
          onClick={() => report.loadPlatformExport("/reports/platform/fraud-dashboard", PROTECTED_ACTIONS.platformFraudDashboard)}
          {...auth.actionState(true, true, "Required permission", PROTECTED_ACTIONS.platformFraudDashboard)}
        >
          <ShieldAlert size={16} aria-hidden />
          <span>Fraud dashboard</span>
        </button>
      </div>
    </>
  );
}
