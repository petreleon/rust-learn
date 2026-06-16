"use client";

import { Loader2 } from "lucide-react";
import { type OrganizationRewardDashboard } from "@/lib/organization/OrganizationRewardDashboard";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { ReportErrorState } from "@/components/organization-routes/ReportErrorState";
import styles from "@/components/organization-routes.module.css";
import { type CsvState } from "../model/CsvState";
import { CourseRewardVolumePanel } from "./CourseRewardVolumePanel";
import { ReportDashboardHero } from "./ReportDashboardHero";
import { ReportSummaryGrid } from "./ReportSummaryGrid";
import { SponsoredApplicationsPanel } from "./SponsoredApplicationsPanel";
import { WalletBalancesPanel } from "./WalletBalancesPanel";

type Props = {
  csvError: RouteError | null;
  csvState: CsvState;
  dateFrom: string;
  dateTo: string;
  onDateFromChange: (value: string) => void;
  onDateToChange: (value: string) => void;
  onDownloadCsv: () => void;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
  report: OrganizationRewardDashboard | null;
  reportError: RouteError | null;
  reportLoadState: LoadState;
};

export function OrganizationReportsContent(props: Props) {
  if (props.reportLoadState === "loading" || props.reportLoadState === "idle") {
    return <ReportLoadingState />;
  }

  if (props.reportLoadState === "error") {
    return <ReportErrorState error={props.reportError} onRetry={props.onRefresh} />;
  }

  if (!props.report) {
    return null;
  }

  return (
    <>
      <ReportDashboardHero
        csvError={props.csvError}
        csvState={props.csvState}
        dateFrom={props.dateFrom}
        dateTo={props.dateTo}
        onDateFromChange={props.onDateFromChange}
        onDateToChange={props.onDateToChange}
        onDownloadCsv={props.onDownloadCsv}
        onRefresh={props.onRefresh}
        organization={props.organization}
      />
      <ReportSummaryGrid report={props.report} />
      <section className={styles.twoColumn}>
        <SponsoredApplicationsPanel summary={props.report.sponsored_teacher_applications} />
        <WalletBalancesPanel wallets={props.report.wallets} />
      </section>
      <CourseRewardVolumePanel courses={props.report.courses} />
    </>
  );
}

function ReportLoadingState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Loading reward reports</h2>
      </div>
      <div className={styles.skeletonGrid} aria-hidden>
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
      </div>
    </section>
  );
}
