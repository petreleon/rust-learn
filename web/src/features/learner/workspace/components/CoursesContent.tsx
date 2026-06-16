"use client";

import { BookOpen, CheckCircle, RefreshCw, Search, Trophy } from "lucide-react";
import { type FormEvent } from "react";
import { type CourseCatalogItem, type CourseCatalogResponse } from "@/lib/learner";
import { type CurrentSession } from "@/lib/session";
import { catalogVisibleRange } from "../model/catalogVisibleRange";
import styles from "../learner-workspace.module.css";
import { CourseCatalogCard } from "./CourseCatalogCard";
import { EmptyState } from "./EmptyState";
import { StatusPill } from "./StatusPill";
import { SummaryCard } from "./SummaryCard";
import { enrollmentFilterOptions } from "./enrollmentFilterOptions";
import { type EnrollmentStatusFilter } from "./EnrollmentStatusFilter";

export function CoursesContent({
  catalog,
  enrollmentFilter,
  joiningCourseId,
  onApplySearch,
  onChangeEnrollmentFilter,
  onChangeRewardOnly,
  onChangeSearchInput,
  onClearFilters,
  onRefresh,
  onRequestJoin,
  rewardOnly,
  search,
  searchInput,
  session,
}: {
  catalog: CourseCatalogResponse | null;
  enrollmentFilter: EnrollmentStatusFilter;
  joiningCourseId: number | null;
  onApplySearch: (event: FormEvent<HTMLFormElement>) => void;
  onChangeEnrollmentFilter: (filter: EnrollmentStatusFilter) => void;
  onChangeRewardOnly: (checked: boolean) => void;
  onChangeSearchInput: (value: string) => void;
  onClearFilters: () => void;
  onRefresh: () => void;
  onRequestJoin: (course: CourseCatalogItem) => void;
  rewardOnly: boolean;
  search: string;
  searchInput: string;
  session: CurrentSession;
}) {
  const courses = catalog?.courses || [];
  const visibleRange = catalogVisibleRange(catalog);
  const rewardCourseCount = courses.filter((course) => course.rewards.available).length;
  const activeFilterCount = [search ? 1 : 0, enrollmentFilter !== "all" ? 1 : 0, rewardOnly ? 1 : 0].reduce(
    (total, value) => total + value,
    0,
  );

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Catalog matches" value={catalog?.total ?? 0} />
        <SummaryCard icon={<CheckCircle size={20} aria-hidden />} label="Current courses" value={session.courses.length} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward-ready" value={rewardCourseCount} />
      </section>

      <section className={styles.catalogPanel}>
        <div className={styles.panelHeader}>
          <Search size={20} aria-hidden />
          <h2>Course catalog</h2>
          <button className={styles.iconAction} aria-label="Refresh courses" type="button" onClick={onRefresh}>
            <RefreshCw size={18} aria-hidden />
          </button>
        </div>

        <form className={styles.searchForm} onSubmit={onApplySearch}>
          <label className={styles.searchField}>
            <Search size={18} aria-hidden />
            <span className={styles.srOnly}>Search courses</span>
            <input
              placeholder="Search courses"
              type="search"
              value={searchInput}
              onChange={(event) => onChangeSearchInput(event.target.value)}
            />
          </label>
          <button className={styles.primaryLink} type="submit">
            <Search size={18} aria-hidden />
            Search
          </button>
        </form>

        <div className={styles.filterBar} aria-label="Course filters">
          {enrollmentFilterOptions.map((option) => (
            <button
              aria-pressed={enrollmentFilter === option.value}
              className={`${styles.filterButton} ${enrollmentFilter === option.value ? styles.activeFilter : ""}`}
              key={option.value}
              type="button"
              onClick={() => onChangeEnrollmentFilter(option.value)}
            >
              {option.label}
            </button>
          ))}
          <label className={`${styles.filterToggle} ${rewardOnly ? styles.activeToggle : ""}`}>
            <input
              checked={rewardOnly}
              type="checkbox"
              onChange={(event) => onChangeRewardOnly(event.target.checked)}
            />
            <Trophy size={17} aria-hidden />
            Rewards
          </label>
          {activeFilterCount ? (
            <button className={styles.secondaryButton} type="button" onClick={onClearFilters}>
              Clear
            </button>
          ) : null}
        </div>
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Courses</h2>
          <StatusPill label={visibleRange.label} tone="neutral" />
        </div>
        {visibleRange.detail ? <p className={styles.muted}>{visibleRange.detail}</p> : null}
        {courses.length ? (
          <div className={styles.itemGrid}>
            {courses.map((course) => (
              <CourseCatalogCard
                course={course}
                emailVerified={session.user.email_verified}
                joining={joiningCourseId === course.id}
                key={course.id}
                onRequestJoin={onRequestJoin}
              />
            ))}
          </div>
        ) : (
          <EmptyState
            action={
              activeFilterCount ? (
                <button className={styles.secondaryButton} type="button" onClick={onClearFilters}>
                  Clear filters
                </button>
              ) : undefined
            }
            detail={
              activeFilterCount
                ? "No visible courses match these filters."
                : "Visible courses will appear after they are published or assigned to you."
            }
            title={activeFilterCount ? "No matching courses" : "No courses visible"}
          />
        )}
      </section>

      {session.courses.length ? (
        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2>Current access</h2>
            <StatusPill label={`${session.courses.length} scopes`} tone="neutral" />
          </div>
          <div className={styles.compactList}>
            {session.courses.slice(0, 4).map((course) => (
              <span key={course.id}>
                {course.title} - {course.roles.length ? course.roles.join(", ") : "Direct access"}
              </span>
            ))}
          </div>
        </section>
      ) : null}
    </>
  );
}
