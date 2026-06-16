"use client";

import { Loader2, RefreshCw, Search } from "lucide-react";
import { type TeacherApplicationStatus } from "@/lib/admin/TeacherApplicationStatus";
import { type LoadState } from "@/shared/route-state/LoadState";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { type TeacherApplicationFilters } from "../model/TeacherApplicationFilters";
import { teacherApplicationStatusOptions } from "../model/teacherApplicationDisplay";

export function TeacherApplicationFilterPanel({
  filters,
  onApply,
  onRefresh,
  onReset,
  onUpdate,
  state,
}: {
  filters: TeacherApplicationFilters;
  onApply: () => void;
  onRefresh: () => void;
  onReset: () => void;
  onUpdate: (patch: Partial<TeacherApplicationFilters>) => void;
  state: LoadState;
}) {
  return (
    <section className={styles.filterPanel} aria-label="Teacher application filters">
      <label>
        <span>Search</span>
        <span className={styles.inputWithIcon}>
          <Search size={17} aria-hidden />
          <input
            onChange={(event) => onUpdate({ searchInput: event.target.value })}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.preventDefault();
                onApply();
              }
            }}
            placeholder="Applicant, email, sponsor, course, status"
            type="search"
            value={filters.searchInput}
          />
        </span>
      </label>
      <label>
        <span>Status</span>
        <select
          onChange={(event) => onUpdate({ offset: 0, status: event.target.value as TeacherApplicationStatus | "" })}
          value={filters.status}
        >
          {teacherApplicationStatusOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <div className={styles.filterActions}>
        <button className={styles.primaryButton} onClick={onApply} type="button">
          <Search size={16} aria-hidden />
          Apply
        </button>
        <button className={styles.secondaryButton} onClick={onReset} type="button">
          Reset
        </button>
        <button className={styles.secondaryButton} disabled={state === "loading"} onClick={onRefresh} type="button">
          {state === "loading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <RefreshCw size={16} aria-hidden />}
          Refresh
        </button>
      </div>
    </section>
  );
}
