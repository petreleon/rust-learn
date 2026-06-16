"use client";

import { PlusCircle } from "lucide-react";
import { type FormEvent, useState } from "react";
import { type ActionState } from "@/shared/route-state/ActionState";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import {
  defaultCourseCreationDraft,
  type CourseCreationDraft,
  type CourseCreationTarget,
} from "../model/courseCreationModel";

export function CourseCreationPanel({
  actionMessage,
  actionState,
  onSubmit,
  targets,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  onSubmit: (draft: CourseCreationDraft) => void;
  targets: CourseCreationTarget[];
}) {
  const [draft, setDraft] = useState(() => defaultCourseCreationDraft(targets));
  const canCreate = targets.length > 0;

  function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    onSubmit(draft);
  }

  return (
    <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <PlusCircle size={18} aria-hidden />
        <h2>Create course</h2>
      </div>
      {actionMessage ? <p className={styles.muted} role="status">{actionMessage}</p> : null}
      <form className={styles.decisionForm} onSubmit={submit}>
        <label>
          <span>Title</span>
          <input
            disabled={!canCreate || actionState === "saving"}
            onChange={(event) => setDraft({ ...draft, title: event.target.value })}
            placeholder="New course title"
            value={draft.title}
          />
        </label>
        <label>
          <span>Owner</span>
          <select
            disabled={!canCreate || actionState === "saving"}
            onChange={(event) => setDraft({ ...draft, targetValue: event.target.value })}
            value={draft.targetValue}
          >
            {targets.length ? null : <option value="">No create target available</option>}
            {targets.map((target) => (
              <option key={target.value} value={target.value}>{target.label}</option>
            ))}
          </select>
        </label>
        <button className={styles.primaryButton} disabled={!canCreate || actionState === "saving"} type="submit">
          <PlusCircle size={17} aria-hidden />
          Create draft course
        </button>
      </form>
    </section>
  );
}
