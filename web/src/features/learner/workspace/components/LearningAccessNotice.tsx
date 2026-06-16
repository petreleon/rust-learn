"use client";

import { Eye } from "lucide-react";
import { type CourseLearningResponse } from "@/lib/learner";
import styles from "../learner-workspace.module.css";
import { enrollmentTone } from "./enrollmentTone";
import { humanize } from "./humanize";
import { StatusPill } from "./StatusPill";

export function LearningAccessNotice({ learning }: { learning: CourseLearningResponse }) {
  const state = learning.course.enrollment.state;
  if (state === "enrolled") return null;

  const pending = state === "pending" || state === "waitlisted";
  const detail = pending
    ? "Your join request is still waiting for review, so this course is read-only until enrollment is approved."
    : "Your account can inspect this content, but learner progress is saved only after course enrollment is approved.";

  return (
    <section className={styles.statePanel}>
      <div className={styles.panelHeader}>
        <Eye size={20} aria-hidden />
        <h2>Preview mode</h2>
        <StatusPill label={humanize(state)} tone={enrollmentTone(state)} />
      </div>
      <p className={styles.muted}>{detail}</p>
      {learning.course.enrollment.reason ? (
        <p className={styles.muted}>{learning.course.enrollment.reason}</p>
      ) : null}
    </section>
  );
}
