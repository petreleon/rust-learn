"use client";

import { Download, Loader2 } from "lucide-react";
import { useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace, downloadPlatformCsv, type PlatformCsvReport } from "@/lib/admin";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { AdminDeniedState } from "@/features/admin/shared/route-kit/AdminDeniedState";
import { GatedPanel } from "@/features/admin/shared/route-kit/GatedPanel";
import { LoadingState } from "@/features/admin/shared/route-kit/LoadingState";
import { SessionErrorState } from "@/features/admin/shared/route-kit/SessionErrorState";
import { SignedOutState } from "@/features/admin/shared/route-kit/SignedOutState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { emptyWorkspace } from "@/features/admin/shared/route-kit/emptyWorkspace";
import { exportReports } from "@/features/admin/shared/route-kit/exportReports";
import { getCapability } from "@/features/admin/shared/route-kit/getCapability";
import { hasPlatformCapability } from "@/features/admin/shared/route-kit/hasPlatformCapability";
import { normalizeRouteError } from "@/features/admin/shared/route-kit/normalizeRouteError";
import { startCsvDownload } from "@/features/admin/shared/route-kit/startCsvDownload";
import { useAdminSession } from "@/features/admin/shared/route-kit/useAdminSession";
import { type CsvState } from "@/features/admin/shared/route-kit/CsvState";
import { type RouteError } from "@/features/admin/shared/route-kit/RouteError";

export function AdminExportsRoute() {
  const route = useAdminSession();
  const workspace = useMemo(
    () => (route.session ? buildPlatformAdminWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const allowed = route.session ? hasPlatformAdminAccess(route.session) : false;
  const canExport = hasPlatformCapability(workspace, "exports");

  const [csvState, setCsvState] = useState<CsvState>("idle");
  const [csvError, setCsvError] = useState<RouteError | null>(null);
  const [csvFilename, setCsvFilename] = useState<string | null>(null);

  const handleDownload = async (report: PlatformCsvReport, label: string) => {
    const token = route.token;
    if (!token || !canExport) return;
    setCsvState("downloading");
    setCsvError(null);
    setCsvFilename(null);
    try {
      const csv = await downloadPlatformCsv({ report, token });
      startCsvDownload(csv);
      setCsvFilename(csv.filename);
      setCsvState("success");
      window.setTimeout(() => setCsvState("idle"), 3000);
    } catch (error) {
      setCsvError(normalizeRouteError(error, `${label} CSV could not be downloaded.`));
      setCsvState("error");
    }
  };

  const notice: ShellNotice | null =
    csvState === "success" && csvFilename
      ? { message: `Downloaded ${csvFilename}.`, title: "CSV ready", tone: "success" }
      : csvState === "error" && csvError
        ? { message: csvError.message, title: "Download failed", tone: csvError.status === 403 ? "warn" : "error" }
        : null;

  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[
        { label: "Admin", href: "/admin" },
        { label: "Exports" },
      ]}
      description="Download platform reports as CSV from the export endpoints."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : canExport ? "Export access" : "Export gated"} />
          <StatusPill label={`${exportReports.length} reports`} />
        </>
      }
      title="Report exports"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !allowed ? <AdminDeniedState workspace={workspace} /> : null}

      {route.session && allowed ? (
        canExport ? (
          <div className={styles.stack}>
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
                      onClick={() => handleDownload(report.report, report.label)}
                      type="button"
                    >
                      {csvState === "downloading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Download size={16} aria-hidden />}
                      CSV
                    </button>
                  </article>
                ))}
              </div>
            </section>
          </div>
        ) : (
          <GatedPanel
            capability={getCapability(workspace, "exports")}
            icon={<Download size={20} aria-hidden />}
            title="CSV exports unavailable"
          />
        )
      ) : null}
    </ProductShell>
  );
}
