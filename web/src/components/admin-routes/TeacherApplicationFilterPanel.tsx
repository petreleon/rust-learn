"use client";

import { Loader2, RefreshCw, Search } from "lucide-react";
import { type TeacherApplicationStatus } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { teacherApplicationStatusOptions } from "./teacherApplicationStatusOptions";
import { type SectionState } from "./SectionState";

export function TeacherApplicationFilterPanel({
  onApply,
  onRefresh,
  onReset,
  searchInput,
  setSearchInput,
  setStatusFilter,
  state,
  statusFilter,
}: {
  onApply: () => void;
  onRefresh: () => void;
  onReset: () => void;
  searchInput: string;
  setSearchInput: (value: string) => void;
  setStatusFilter: (value: TeacherApplicationStatus | "") => void;
  state: SectionState;
  statusFilter: TeacherApplicationStatus | "";
}) {
  return (
    <section className={styles.filterPanel} aria-label="Teacher application filters">
      <label>
        <span>Search</span>
        <span className={styles.inputWithIcon}>
          <Search size={17} aria-hidden />
          <input
            onChange={(event) => setSearchInput(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.preventDefault();
                onApply();
              }
            }}
            placeholder="Applicant, email, sponsor, course, status"
            type="search"
            value={searchInput}
          />
        </span>
      </label>
      <label>
        <span>Status</span>
        <select
          onChange={(event) => setStatusFilter(event.target.value as TeacherApplicationStatus | "")}
          value={statusFilter}
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
