"use client";

import { Filter, Search } from "lucide-react";
import { type FormEvent } from "react";
import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { type ActionState } from "@/shared/route-state/ActionState";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type CourseCreationDraft, type CourseCreationTarget } from "../model/courseCreationModel";
import { type CourseQuery } from "../model/CourseQuery";
import { CourseCreationPanel } from "./CourseCreationPanel";
import { CourseGrid } from "./CourseGrid";

export function CoursesView({
  courseCreation,
  courses,
  onApplyFilters,
  onQueryChange,
  query,
  total,
}: {
  courseCreation: {
    actionMessage: string | null;
    actionState: ActionState;
    submitCourseCreation: (draft: CourseCreationDraft) => void;
    targets: CourseCreationTarget[];
  };
  courses: TeacherCourseDashboardItem[];
  onApplyFilters: (event: FormEvent<HTMLFormElement>) => void;
  onQueryChange: (query: CourseQuery) => void;
  query: CourseQuery;
  total: number;
}) {
  const creationKey = courseCreation.targets.map((target) => target.value).join(":");
  return (
    <>
      <CourseCreationPanel
        actionMessage={courseCreation.actionMessage}
        actionState={courseCreation.actionState}
        key={creationKey}
        onSubmit={courseCreation.submitCourseCreation}
        targets={courseCreation.targets}
      />
      <form className={styles.filterPanel} onSubmit={onApplyFilters}>
        <label>
          <span>Search</span>
          <div className={styles.inputWithIcon}>
            <Search size={17} aria-hidden />
            <input
              onChange={(event) => onQueryChange({ ...query, search: event.target.value })}
              placeholder="Course title"
              type="search"
              value={query.search}
            />
          </div>
        </label>
        <label>
          <span>Lifecycle</span>
          <select
            onChange={(event) => onQueryChange({ ...query, lifecycleStatus: event.target.value })}
            value={query.lifecycleStatus}
          >
            <option value="all">All states</option>
            <option value="draft">Draft</option>
            <option value="submitted">Submitted</option>
            <option value="needs_changes">Needs changes</option>
            <option value="approved">Approved</option>
            <option value="published">Published</option>
            <option value="archived">Archived</option>
            <option value="suspended">Suspended</option>
          </select>
        </label>
        <button className={styles.primaryButton} type="submit">
          <Filter size={17} aria-hidden />
          Apply filters
        </button>
      </form>
      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Visible teaching courses</h2>
            <p className={styles.muted}>
              {total} course{total === 1 ? "" : "s"} {total === 1 ? "matches" : "match"} the current teaching scope.
            </p>
          </div>
        </div>
        <CourseGrid courses={courses} emptyDetail="No teaching courses match the current filters." />
      </section>
    </>
  );
}
