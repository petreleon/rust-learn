"use client";

import { RefreshCw, Search } from "lucide-react";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { organizationCourseLifecycleOptions } from "@/components/organization-routes/organizationCourseLifecycleOptions";

export type CourseRewardFilter = "all" | "rewarded" | "unrewarded";

export function CourseFilterPanel({
  draftSearch,
  lifecycleStatus,
  onApplyFilters,
  onDraftSearchChange,
  onLifecycleStatusChange,
  onResetFilters,
  onRewardFilterChange,
  rewardFilter,
}: {
  draftSearch: string;
  lifecycleStatus: string;
  onApplyFilters: () => void;
  onDraftSearchChange: (value: string) => void;
  onLifecycleStatusChange: (value: string) => void;
  onResetFilters: () => void;
  onRewardFilterChange: (value: CourseRewardFilter) => void;
  rewardFilter: CourseRewardFilter;
}) {
  return (
    <section className={styles.courseFilterPanel} aria-label="Organization course filters">
      <label>
        <span>Search courses</span>
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
            placeholder="Title"
            type="search"
            value={draftSearch}
          />
        </span>
      </label>
      <label>
        <span>Lifecycle</span>
        <select aria-label="Course lifecycle filter" onChange={(event) => onLifecycleStatusChange(event.target.value)} value={lifecycleStatus}>
          {organizationCourseLifecycleOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <label>
        <span>Rewards</span>
        <select
          aria-label="Course reward filter"
          onChange={(event) => onRewardFilterChange(event.target.value as CourseRewardFilter)}
          value={rewardFilter}
        >
          <option value="all">All reward states</option>
          <option value="rewarded">Reward policy active</option>
          <option value="unrewarded">No active policy</option>
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
