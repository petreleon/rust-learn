"use client";

import { BookOpen, FileText, Loader2, Trophy, Users } from "lucide-react";
import { type OrganizationCourseList } from "@/lib/organization/OrganizationCourseList";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/components/organization-routes.module.css";
import { CourseErrorState } from "@/components/organization-routes/CourseErrorState";
import { SummaryCard } from "@/components/organization-routes/SummaryCard";
import { CourseDirectoryHero } from "./CourseDirectoryHero";
import { CourseFilterPanel, type CourseRewardFilter } from "./CourseFilterPanel";
import { CourseListPanel } from "./CourseListPanel";
import { organizationCourseCounts } from "./courseCounts";

type Props = {
  courses: OrganizationCourseList | null;
  draftSearch: string;
  lifecycleStatus: string;
  loadState: LoadState;
  onApplyFilters: () => void;
  onDraftSearchChange: (value: string) => void;
  onLifecycleStatusChange: (value: string) => void;
  onPageChange: (page: number) => void;
  onRefresh: () => void;
  onResetFilters: () => void;
  onRewardFilterChange: (value: CourseRewardFilter) => void;
  organization: OrganizationWorkspaceItem;
  page: number;
  routeError: RouteError | null;
  rewardFilter: CourseRewardFilter;
};

export function OrganizationCoursesContent(props: Props) {
  if (props.loadState === "loading" || props.loadState === "idle") {
    return <CourseLoadingState />;
  }

  if (props.loadState === "error") {
    return <CourseErrorState error={props.routeError} onRetry={props.onRefresh} />;
  }

  if (!props.courses) {
    return null;
  }

  const counts = organizationCourseCounts(props.courses, props.page);

  return (
    <>
      <CourseDirectoryHero onRefresh={props.onRefresh} organization={props.organization} />
      <section className={styles.summaryGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Matching courses" value={props.courses.total} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Reward policies" value={counts.activePolicyCount} />
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Pending joins" value={counts.pendingJoinCount} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward queue" value={counts.pendingRewardCount} />
      </section>
      <CourseFilterPanel
        draftSearch={props.draftSearch}
        lifecycleStatus={props.lifecycleStatus}
        onApplyFilters={props.onApplyFilters}
        onDraftSearchChange={props.onDraftSearchChange}
        onLifecycleStatusChange={props.onLifecycleStatusChange}
        onResetFilters={props.onResetFilters}
        onRewardFilterChange={props.onRewardFilterChange}
        rewardFilter={props.rewardFilter}
      />
      <CourseListPanel courses={props.courses} onPageChange={props.onPageChange} page={props.page} />
    </>
  );
}

function CourseLoadingState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Loading organization courses</h2>
      </div>
      <div className={styles.skeletonGrid} aria-hidden>
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
      </div>
    </section>
  );
}
