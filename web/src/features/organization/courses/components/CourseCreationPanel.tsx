"use client";

import { PlusCircle } from "lucide-react";
import { type FormEvent } from "react";
import { type ActionState } from "@/shared/route-state/ActionState";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function CourseCreationPanel({
  actionMessage,
  actionState,
  canCreate,
  onSubmit,
  onTitleChange,
  organizationName,
  titleDraft,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  canCreate: boolean;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  onTitleChange: (value: string) => void;
  organizationName: string;
  titleDraft: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <PlusCircle size={18} aria-hidden />
        <h2>Create organization course</h2>
      </div>
      {actionMessage ? <p className={styles.muted} role="status">{actionMessage}</p> : null}
      <form className={styles.filterPanel} onSubmit={onSubmit}>
        <label>
          <span>Title</span>
          <input
            disabled={!canCreate || actionState === "saving"}
            onChange={(event) => onTitleChange(event.target.value)}
            placeholder={`${organizationName} course title`}
            value={titleDraft}
          />
        </label>
        <button className={styles.primaryButton} disabled={!canCreate || actionState === "saving"} type="submit">
          <PlusCircle size={17} aria-hidden />
          Create draft course
        </button>
      </form>
    </section>
  );
}
