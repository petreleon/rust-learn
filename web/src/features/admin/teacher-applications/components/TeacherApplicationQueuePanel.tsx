"use client";

import { UserCheck } from "lucide-react";
import { type PlatformTeacherApplicationItem } from "@/lib/admin/PlatformTeacherApplicationItem";
import { type PlatformTeacherApplicationsResponse } from "@/lib/admin/PlatformTeacherApplicationsResponse";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/components/admin-routes/EmptyState";
import { formatDate } from "@/components/admin-routes/formatDate";
import { formatUnderscoreLabel } from "@/components/admin-routes/formatUnderscoreLabel";
import { PanelError } from "@/components/admin-routes/PanelError";
import { PanelLoading } from "@/components/admin-routes/PanelLoading";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { type TeacherApplicationFilters } from "../model/TeacherApplicationFilters";
import {
  teacherApplicationScopeTargetLabel,
  teacherApplicationStatusTone,
} from "../model/teacherApplicationDisplay";

export function TeacherApplicationQueuePanel({
  applications,
  error,
  filters,
  onPageOffset,
  onRefresh,
  onSelect,
  selectedApplicationId,
  state,
}: {
  applications: PlatformTeacherApplicationsResponse | null;
  error: RouteError | null;
  filters: TeacherApplicationFilters;
  onPageOffset: (offset: number) => void;
  onRefresh: () => void;
  onSelect: (applicationId: number) => void;
  selectedApplicationId: number | null;
  state: LoadState;
}) {
  if (state === "loading" || state === "idle") return <PanelLoading title="Loading teacher applications" />;
  if (state === "error") return <PanelError error={error} onRetry={onRefresh} title="Teacher application queue failed" />;
  if (!applications) return null;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <UserCheck size={20} aria-hidden />
        <div>
          <h2>Review queue</h2>
          <p>{applications.total} application{applications.total === 1 ? " matches" : "s match"} the current filters.</p>
        </div>
      </div>
      {applications.applications.length ? (
        <div className={styles.rowList}>
          {applications.applications.map((application) => (
            <TeacherApplicationQueueRow
              application={application}
              isSelected={selectedApplicationId === application.id}
              key={application.id}
              onSelect={onSelect}
            />
          ))}
        </div>
      ) : (
        <EmptyState text="No teacher applications match these filters." />
      )}
      <TeacherApplicationPagination applications={applications} filters={filters} onPageOffset={onPageOffset} />
    </section>
  );
}

function TeacherApplicationQueueRow({
  application,
  isSelected,
  onSelect,
}: {
  application: PlatformTeacherApplicationItem;
  isSelected: boolean;
  onSelect: (applicationId: number) => void;
}) {
  return (
    <article className={`${styles.compactRow} ${isSelected ? styles.selectedRow : ""}`}>
      <div>
        <strong>{application.applicant.name}</strong>
        <span>{application.applicant.email}</span>
        <small>
          {formatUnderscoreLabel(application.requested_scope)} · {teacherApplicationScopeTargetLabel(application)}
        </small>
        <small>{application.experience_summary}</small>
      </div>
      <div className={styles.rowMeta}>
        <StatusPill
          label={formatUnderscoreLabel(application.status)}
          tone={teacherApplicationStatusTone(application.status)}
        />
        <span>{formatDate(application.updated_at)}</span>
        <button
          aria-label={`Review ${application.applicant.name}`}
          className={styles.secondaryButton}
          onClick={() => onSelect(application.id)}
          type="button"
        >
          Review
        </button>
      </div>
    </article>
  );
}

function TeacherApplicationPagination({
  applications,
  filters,
  onPageOffset,
}: {
  applications: PlatformTeacherApplicationsResponse;
  filters: TeacherApplicationFilters;
  onPageOffset: (offset: number) => void;
}) {
  const start = applications.applications.length ? filters.offset + 1 : 0;
  const end = Math.min(filters.offset + applications.applications.length, applications.total);

  return (
    <div className={styles.paginationRow}>
      <button className={styles.secondaryButton} disabled={filters.offset <= 0} onClick={() => onPageOffset(filters.offset - applications.limit)} type="button">
        Previous
      </button>
      <span>
        Showing {start}-{end} of {applications.total}
      </span>
      <button
        className={styles.secondaryButton}
        disabled={filters.offset + applications.limit >= applications.total}
        onClick={() => onPageOffset(filters.offset + applications.limit)}
        type="button"
      >
        Next
      </button>
    </div>
  );
}
