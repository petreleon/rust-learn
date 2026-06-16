"use client";

import { type AssessmentAttempt } from "@/lib/learner";
import { completedAttempts } from "../model/assessmentState";
import { StatusPill } from "../../components/StatusPill";
import styles from "../../learner-workspace.module.css";

export function AttemptHistory({ attempts }: { attempts: AssessmentAttempt[] }) {
  const completed = completedAttempts(attempts);

  if (!completed.length) {
    return <p className={styles.muted}>No attempts yet.</p>;
  }

  return (
    <div className={styles.attemptList}>
      {completed.slice(0, 3).map((attempt) => (
        <div className={styles.attemptRow} key={attempt.id}>
          <span>Score {attempt.score ?? 0}</span>
          <StatusPill
            label={attempt.passed ? "Passed" : "Failed"}
            tone={attempt.passed ? "good" : "warn"}
          />
        </div>
      ))}
    </div>
  );
}
