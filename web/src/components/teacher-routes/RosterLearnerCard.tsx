"use client";

import { UserMinus } from "lucide-react";
import { type TeacherCourseRosterLearner } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { DetailLine } from "./DetailLine";
import { statusLabel } from "./statusLabel";
import { type ActionState } from "./ActionState";

export function RosterLearnerCard({
  actionState,
  confirmRemoval,
  learner,
  onRemove,
}: {
  actionState: ActionState;
  confirmRemoval: boolean;
  learner: TeacherCourseRosterLearner;
  onRemove: () => void;
}) {
  return (
    <article className={styles.enrollmentCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{learner.user.email}</p>
          <h3>{learner.user.name}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles.good}`}>{statusLabel(learner.access_state)}</span>
      </div>
      <div className={styles.detailList}>
        <DetailLine label="Roles" value={learner.roles.join(", ") || "Learner"} />
        <DetailLine label="Latest request" value={learner.latest_join_request_status ? statusLabel(learner.latest_join_request_status) : "No request history"} />
        <DetailLine label="Progress" value={learner.progress_supported ? "Tracked" : "Not tracked yet"} />
        <DetailLine label="Reward eligibility" value={learner.reward_eligibility_supported ? "Tracked" : "Not tracked yet"} />
      </div>
      {learner.can_remove ? (
        <div className={styles.rosterAction}>
          {confirmRemoval ? (
            <p className={styles.muted}>Confirm removal to revoke course access for this learner.</p>
          ) : null}
          <button className={confirmRemoval ? styles.primaryButton : styles.secondaryButton} disabled={actionState === "saving"} type="button" onClick={onRemove}>
            <UserMinus size={17} aria-hidden />
            {confirmRemoval ? "Confirm removal" : "Remove"}
          </button>
        </div>
      ) : null}
    </article>
  );
}
