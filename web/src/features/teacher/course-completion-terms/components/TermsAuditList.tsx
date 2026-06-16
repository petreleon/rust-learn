import { History } from "lucide-react";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type CourseCompletionTermsHistory } from "../model/CourseCompletionTerms";
import { formatTermsDate, statusLabel } from "../model/termsDisplay";

export function TermsAuditList({ history }: { history: CourseCompletionTermsHistory }) {
  return (
    <div className={styles.priorityList}>
      {history.audit_events.slice(0, 4).map((event) => (
        <article className={styles.priorityItem} key={event.id}>
          <span className={styles.smallIcon}>
            <History size={16} aria-hidden />
          </span>
          <div>
            <strong>{statusLabel(event.event_type)}</strong>
            <p>
              {event.completion_reward_amount} tokens, {event.max_enrolled_students} students
            </p>
            <p>{formatTermsDate(event.created_at)}</p>
          </div>
        </article>
      ))}
    </div>
  );
}
