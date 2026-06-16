"use client";

import { Settings, ShieldCheck } from "lucide-react";
import { type FormEvent, useState } from "react";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { type ActionState } from "@/shared/route-state/ActionState";
import { DetailLine } from "@/features/teacher/shared/route-kit/DetailLine";
import { statusLabel } from "@/features/teacher/shared/route-kit/statusLabel";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { courseLifecycleOptions } from "../model/courseLifecycleOptions";
import { courseSettingsDraft, type CourseSettingsDraft } from "../model/courseSettingsModel";

export function CourseSettingsPanel({
  actionMessage,
  actionState,
  onLifecycleSubmit,
  onSettingsSubmit,
  organizationNames,
  workspace,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  onLifecycleSubmit: (status: string) => void;
  onSettingsSubmit: (draft: CourseSettingsDraft) => void;
  organizationNames: string;
  workspace: TeacherCourseWorkspaceResponse;
}) {
  const [draft, setDraft] = useState(() => courseSettingsDraft(workspace.course));
  const [targetStatus, setTargetStatus] = useState(workspace.course.lifecycle_status);
  const canManageSettings = workspace.course.permissions.can_manage_settings;

  function submitSettings(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    onSettingsSubmit(draft);
  }

  function submitLifecycle(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    onLifecycleSubmit(targetStatus);
  }

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Settings size={20} aria-hidden />
        <h2>Course settings</h2>
      </div>

      {actionMessage ? <p className={styles.muted} role="status">{actionMessage}</p> : null}

      <form className={styles.decisionForm} onSubmit={submitSettings}>
        <label>
          <span>Title</span>
          <input
            disabled={!canManageSettings || actionState === "saving"}
            onChange={(event) => setDraft({ ...draft, title: event.target.value })}
            value={draft.title}
          />
        </label>
        <label>
          <span>Description</span>
          <textarea
            disabled={!canManageSettings || actionState === "saving"}
            onChange={(event) => setDraft({ ...draft, description: event.target.value })}
            rows={3}
            value={draft.description}
          />
        </label>
        <label>
          <span>Topics</span>
          <textarea
            disabled={!canManageSettings || actionState === "saving"}
            onChange={(event) => setDraft({ ...draft, topics: event.target.value })}
            rows={2}
            value={draft.topics}
          />
        </label>
        <label>
          <span>Prerequisites</span>
          <textarea
            disabled={!canManageSettings || actionState === "saving"}
            onChange={(event) => setDraft({ ...draft, prerequisites: event.target.value })}
            rows={2}
            value={draft.prerequisites}
          />
        </label>
        <button className={styles.primaryButton} disabled={!canManageSettings || actionState === "saving"} type="submit">
          <Settings size={17} aria-hidden />
          Save course settings
        </button>
      </form>

      <form className={styles.decisionForm} onSubmit={submitLifecycle}>
        <label>
          <span>Lifecycle</span>
          <select
            disabled={!canManageSettings || actionState === "saving"}
            onChange={(event) => setTargetStatus(event.target.value)}
            value={targetStatus}
          >
            {courseLifecycleOptions.map((option) => (
              <option key={option.value} value={option.value}>{option.label}</option>
            ))}
          </select>
        </label>
        <button className={styles.secondaryButton} disabled={!canManageSettings || actionState === "saving"} type="submit">
          <ShieldCheck size={17} aria-hidden />
          Update lifecycle
        </button>
      </form>

      <div className={styles.detailList}>
        <DetailLine label="Current lifecycle" value={statusLabel(workspace.publication.course_lifecycle_status)} />
        <DetailLine label="Teacher roles" value={workspace.teacher_roles.join(", ") || "Delegated permission"} />
        <DetailLine label="Organizations" value={organizationNames} />
        <DetailLine
          label="Per-content publication"
          value={workspace.publication.content_publication_status_supported ? "Supported" : "Inherited from course"}
        />
      </div>
    </section>
  );
}
