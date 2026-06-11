"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { buildOrganizationWorkspace, downloadOrganizationRewardDashboardCsv, fetchOrganizationRewardDashboard, findOrganizationWorkspaceItem, type OrganizationRewardDashboard } from "@/lib/organization";
import { clearStoredSessionToken, readStoredSessionToken } from "@/lib/session";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";
import { MissingOrganizationState } from "./MissingOrganizationState";
import { OrganizationReportsContent } from "./OrganizationReportsContent";
import { OrganizationStatus } from "./OrganizationStatus";
import { ReportDeniedState } from "./ReportDeniedState";
import { SignedOutState } from "./SignedOutState";
import { emptyWorkspace } from "./emptyWorkspace";
import { type CsvState } from "./CsvState";
import { type ReportLoadState } from "./ReportLoadState";
import { type RouteError } from "./RouteError";
import { normalizeRouteError } from "./normalizeRouteError";
import { organizationNotice } from "./organizationNotice";
import { triggerCsvDownload } from "./triggerCsvDownload";
import { useOrganizationSession } from "./useOrganizationSession";

export function OrganizationReportsRoute({ organizationId }: { organizationId: string }) {
  const route = useOrganizationSession();
  const numericOrganizationId = Number.parseInt(organizationId, 10);
  const invalidOrganizationId = !/^\d+$/.test(organizationId) || !Number.isFinite(numericOrganizationId);
  const organization = useMemo(
    () =>
      route.session && !invalidOrganizationId
        ? findOrganizationWorkspaceItem(route.session, numericOrganizationId)
        : null,
    [invalidOrganizationId, numericOrganizationId, route.session],
  );
  const workspace = useMemo(
    () => (route.session ? buildOrganizationWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const reportCapability = organization?.capabilities.find((capability) => capability.key === "reports");
  const canViewReports = Boolean(reportCapability?.enabled);
  const [csvError, setCsvError] = useState<RouteError | null>(null);
  const [csvState, setCsvState] = useState<CsvState>("idle");
  const [report, setReport] = useState<OrganizationRewardDashboard | null>(null);
  const [reportError, setReportError] = useState<RouteError | null>(null);
  const [reportLoadState, setReportLoadState] = useState<ReportLoadState>("idle");
  const [dateFrom, setDateFrom] = useState("");
  const [dateTo, setDateTo] = useState("");
  const notice = organizationNotice(route.error || reportError || csvError);

  const loadReport = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canViewReports) {
      return;
    }

    setReportError(null);
    setReportLoadState("loading");

    try {
      const nextReport = await fetchOrganizationRewardDashboard({
        from: dateFrom || undefined,
        organizationId: organization.id,
        to: dateTo || undefined,
        token,
      });
      setReport(nextReport);
      setReportLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
      }
      setReport(null);
      setReportError(routeError);
      setReportLoadState("error");
    }
  }, [canViewReports, dateFrom, dateTo, invalidOrganizationId, organization]);

  useEffect(() => {
    if (route.session && organization && canViewReports) {
      const timeout = window.setTimeout(() => {
        setCsvError(null);
        setCsvState("idle");
        setReport(null);
        setReportError(null);
        setReportLoadState("idle");
        void loadReport();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canViewReports, loadReport, organization, route.session]);

  useEffect(() => {
    function handleVisible() {
      if (document.visibilityState === "visible" && route.session && organization && canViewReports) {
        void loadReport();
      }
    }
    document.addEventListener("visibilitychange", handleVisible);
    return () => document.removeEventListener("visibilitychange", handleVisible);
  }, [canViewReports, loadReport, organization, route.session]);

  async function downloadCsv() {
    const token = readStoredSessionToken();
    if (!token || !organization) {
      setCsvError({ code: "missing_token", message: "Sign in again before exporting reports.", status: 401 });
      setCsvState("error");
      return;
    }

    setCsvError(null);
    setCsvState("downloading");

    try {
      const csv = await downloadOrganizationRewardDashboardCsv({
        from: dateFrom || undefined,
        organizationId: organization.id,
        to: dateTo || undefined,
        token,
      });
      triggerCsvDownload(csv.body, csv.filename);
      setCsvState("success");
    } catch (nextError) {
      setCsvError(normalizeRouteError(nextError));
      setCsvState("error");
    }
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: organization ? `/organizations/${organization.id}` : undefined,
          label: organization?.name || "Organization",
        },
        { label: "Reports" },
      ]}
      description="Organization reward volume, sponsored teacher application summary, wallet balances, and export state."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} reports` : "Organization reports"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/reports`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/reports`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canViewReports ? (
        <ReportDeniedState capability={reportCapability} organizationName={organization.name} />
      ) : null}
      {route.session && organization && canViewReports ? (
          <OrganizationReportsContent
            csvError={csvError}
            csvState={csvState}
            dateFrom={dateFrom}
            dateTo={dateTo}
            onDateFromChange={setDateFrom}
            onDateToChange={setDateTo}
            onDownloadCsv={downloadCsv}
            onRefresh={loadReport}
            organization={organization}
            report={report}
          reportError={reportError}
          reportLoadState={reportLoadState}
        />
      ) : null}
    </ProductShell>
  );
}
