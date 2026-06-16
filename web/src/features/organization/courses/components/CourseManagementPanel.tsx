"use client";

import { Settings, ShieldCheck } from "lucide-react";
import { type FormEvent } from "react";
import { type OrganizationCourseListItem } from "@/lib/organization";
import { type ActionState } from "@/shared/route-state/ActionState";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { organizationCourseLifecycleOptions } from "@/features/organization/shared/route-kit/organizationCourseLifecycleOptions";
import { StatusPill } from "@/features/organization/shared/route-kit/StatusPill";

export type CourseManagementPanelController = {
  actionMessage: string | null;
  actionState: ActionState;
  canManageOrganizationCourse: (course: OrganizationCourseListItem | null) => boolean;
  draft: { title: string };
  handleLifecycleSubmit: (event: FormEvent<HTMLFormElement>) => void;
  handleSettingsSubmit: (event: FormEvent<HTMLFormElement>) => void;
  lifecycleDraft: string;
  selectCourse: (course: OrganizationCourseListItem | null) => void;
  selectedCourseId: number | null;
  setDraft: (draft: { title: string }) => void;
  setLifecycleDraft: (value: string) => void;
};

export function CourseManagementPanel({
  courses,
  management,
}: {
  courses: OrganizationCourseListItem[];
  management: CourseManagementPanelController;
}) {
  const selectedCourse = courses.find((course) => course.id === management.selectedCourseId) ?? null;
  const canManage = management.canManageOrganizationCourse(selectedCourse);
  const isSaving = management.actionState === "saving";

  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <Settings size={18} aria-hidden />
        <h2>Manage course</h2>
        <StatusPill label={canManage ? "Settings access" : "Settings gated"} tone={canManage ? "good" : "neutral"} />
      </div>
      {management.actionMessage ? <p className={styles.muted} role="status">{management.actionMessage}</p> : null}
      <label className={styles.filterPanel}>
        <span>Course to manage</span>
        <select
          aria-label="Course to manage"
          onChange={(event) => management.selectCourse(courseById(courses, event.target.value))}
          value={management.selectedCourseId ?? ""}
        >
          <option value="">Select course...</option>
          {courses.map((course) => (
            <option key={course.id} value={course.id}>
              {course.title}
            </option>
          ))}
        </select>
      </label>
      <form className={styles.filterPanel} onSubmit={management.handleSettingsSubmit}>
        <label>
          <span>Title</span>
          <input
            disabled={!canManage || isSaving}
            onChange={(event) => management.setDraft({ ...management.draft, title: event.target.value })}
            value={management.draft.title}
          />
        </label>
        <button className={styles.primaryButton} disabled={!canManage || isSaving} type="submit">
          <Settings size={17} aria-hidden />
          Save course settings
        </button>
      </form>
      <form className={styles.filterPanel} onSubmit={management.handleLifecycleSubmit}>
        <label>
          <span>Lifecycle target</span>
          <select
            disabled={!canManage || isSaving}
            onChange={(event) => management.setLifecycleDraft(event.target.value)}
            value={management.lifecycleDraft}
          >
            {organizationCourseLifecycleOptions.filter((option) => option.value).map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <button className={styles.secondaryButton} disabled={!canManage || isSaving} type="submit">
          <ShieldCheck size={17} aria-hidden />
          Update lifecycle
        </button>
      </form>
    </section>
  );
}

function courseById(courses: OrganizationCourseListItem[], courseId: string) {
  const numericCourseId = Number.parseInt(courseId, 10);
  return courses.find((course) => course.id === numericCourseId) ?? null;
}
