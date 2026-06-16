"use client";

import { RefreshCw, Search } from "lucide-react";
import { organizationTeacherApplicationStatusOptions } from "@/components/organization-routes/organizationTeacherApplicationStatusOptions";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function TeacherApplicationFilterPanel({
  draftSearch,
  onApplyFilters,
  onDraftSearchChange,
  onResetFilters,
  onStatusFilterChange,
  statusFilter,
}: {
  draftSearch: string;
  onApplyFilters: () => void;
  onDraftSearchChange: (value: string) => void;
  onResetFilters: () => void;
  onStatusFilterChange: (value: string) => void;
  statusFilter: string;
}) {
  return (
    <section className={styles.applicationFilterPanel} aria-label="Teacher application filters">
      <label>
        <span>Search applications</span>
        <span className={styles.inputWithIcon}>
          <Search size={17} aria-hidden />
          <input
            onChange={(event) => onDraftSearchChange(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.preventDefault();
                onApplyFilters();
              }
            }}
            placeholder="Applicant, email, scope, course, or status"
            type="search"
            value={draftSearch}
          />
        </span>
      </label>
      <label>
        <span>Status</span>
        <select aria-label="Teacher application status filter" onChange={(event) => onStatusFilterChange(event.target.value)} value={statusFilter}>
          {organizationTeacherApplicationStatusOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <div className={styles.filterActions}>
        <button className={styles.primaryButton} onClick={onApplyFilters} type="button">
          <Search size={17} aria-hidden />
          Apply
        </button>
        <button className={styles.secondaryButton} onClick={onResetFilters} type="button">
          <RefreshCw size={17} aria-hidden />
          Reset
        </button>
      </div>
    </section>
  );
}
