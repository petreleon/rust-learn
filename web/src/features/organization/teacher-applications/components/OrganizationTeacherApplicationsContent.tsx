"use client";

import { Loader2 } from "lucide-react";
import { type OrganizationTeacherApplicationList } from "@/lib/organization/OrganizationTeacherApplicationList";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { NominationPanel } from "@/components/organization-routes/NominationPanel";
import { TeacherApplicationErrorState } from "@/components/organization-routes/TeacherApplicationErrorState";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { TeacherApplicationFilterPanel } from "./TeacherApplicationFilterPanel";
import { TeacherApplicationListPanel } from "./TeacherApplicationListPanel";
import { TeacherApplicationSummaryGrid } from "./TeacherApplicationSummaryGrid";
import { TeacherApplicationTrackingHero } from "./TeacherApplicationTrackingHero";

type Props = {
  applications: OrganizationTeacherApplicationList | null;
  draftSearch: string;
  loadState: LoadState;
  onApplyFilters: () => void;
  onDraftSearchChange: (value: string) => void;
  onPageChange: (page: number) => void;
  onRefresh: () => void;
  onResetFilters: () => void;
  onStatusFilterChange: (value: string) => void;
  organization: OrganizationWorkspaceItem;
  page: number;
  routeError: RouteError | null;
  statusFilter: string;
};

export function OrganizationTeacherApplicationsContent(props: Props) {
  if (props.loadState === "loading" || props.loadState === "idle") {
    return <TeacherApplicationLoadingState />;
  }

  if (props.loadState === "error") {
    return <TeacherApplicationErrorState error={props.routeError} onRetry={props.onRefresh} />;
  }

  if (!props.applications) {
    return null;
  }

  return (
    <>
      <TeacherApplicationTrackingHero applications={props.applications} onRefresh={props.onRefresh} organization={props.organization} />
      <TeacherApplicationSummaryGrid applications={props.applications} />
      <TeacherApplicationFilterPanel
        draftSearch={props.draftSearch}
        onApplyFilters={props.onApplyFilters}
        onDraftSearchChange={props.onDraftSearchChange}
        onResetFilters={props.onResetFilters}
        onStatusFilterChange={props.onStatusFilterChange}
        statusFilter={props.statusFilter}
      />
      <TeacherApplicationListPanel applications={props.applications} onPageChange={props.onPageChange} page={props.page} />
      <NominationPanel organization={props.organization} onSuccess={props.onRefresh} />
    </>
  );
}

function TeacherApplicationLoadingState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Loading teacher applications</h2>
      </div>
      <div className={styles.skeletonGrid} aria-hidden>
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
      </div>
    </section>
  );
}
