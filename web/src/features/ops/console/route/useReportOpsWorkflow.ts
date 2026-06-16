"use client";

import { useState } from "react";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { hasPositiveInteger } from "../model/hasPositiveInteger";
import { hasText } from "../model/hasText";
import { missingFields } from "../model/missingFields";
import { platformExportReports } from "../model/platformExportReports";
import { type HasOpsPermission, type SendOpsApi } from "../model/opsConsoleTypes";

export function useReportOpsWorkflow({
  hasPermission,
  sendApi,
}: {
  hasPermission: HasOpsPermission;
  sendApi: SendOpsApi;
}) {
  const [organizationId, setOrganizationId] = useState("");
  const canViewOrgReports = hasPermission("VIEW_ORG_REWARD_REPORTS");
  const canViewSummaryReports = hasPermission("VIEW_REPORT");
  const canGenerateReports = hasPermission("GENERATE_REPORT");
  const canViewPlatformDashboards = hasPermission("VIEW_REWARD_AUDIT");
  const canExport = hasPermission("EXPORT_DATA");
  const canUseOrganizationReportControls = canViewSummaryReports || canGenerateReports || canViewOrgReports;
  const canUseReportWorkflow = canUseOrganizationReportControls || canViewPlatformDashboards || canExport;
  const canLoadOrganizationReportForm = hasPositiveInteger(organizationId);
  const hasOrganizationReportDraft = hasText(organizationId);
  const organizationReportMissingFields = missingFields([
    ["Organization id", hasPositiveInteger(organizationId)],
  ]);
  const visibleActions = [
    ...(canViewSummaryReports ? [PROTECTED_ACTIONS.organizationSummary, PROTECTED_ACTIONS.platformSummary] : []),
    ...(canGenerateReports ? [PROTECTED_ACTIONS.organizationSummaryCsv] : []),
    ...(canViewOrgReports ? [PROTECTED_ACTIONS.organizationRewardReport, PROTECTED_ACTIONS.organizationRewardCsv] : []),
    ...(canExport ? platformExportReports.map((report) => report.resultLabel) : []),
    ...(canViewPlatformDashboards
      ? [PROTECTED_ACTIONS.platformRewardDashboard, PROTECTED_ACTIONS.platformFraudDashboard]
      : []),
  ];

  function loadOrganizationReport(csv = false) {
    void sendApi(
      csv ? PROTECTED_ACTIONS.organizationRewardCsv : PROTECTED_ACTIONS.organizationRewardReport,
      `/reports/organizations/${organizationId}/reward-dashboard${csv ? ".csv" : ""}`,
    );
  }

  function loadOrganizationSummary(csv = false) {
    void sendApi(
      csv ? PROTECTED_ACTIONS.organizationSummaryCsv : PROTECTED_ACTIONS.organizationSummary,
      `/reports/organizations/${organizationId}/summary${csv ? ".csv" : ""}`,
    );
  }

  function loadPlatformExport(path: string, label: string) {
    void sendApi(label, path);
  }

  return {
    canExport,
    canGenerateReports,
    canLoadOrganizationReportForm,
    canUseOrganizationReportControls,
    canUseReportWorkflow,
    canViewOrgReports,
    canViewPlatformDashboards,
    canViewSummaryReports,
    hasOrganizationReportDraft,
    loadOrganizationReport,
    loadOrganizationSummary,
    loadPlatformExport,
    organizationId,
    organizationReportMissingFields,
    setOrganizationId,
    visibleActions,
  };
}
