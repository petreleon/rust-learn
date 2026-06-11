"use client";

import { Video } from "lucide-react";
import { type TeacherCourseWorkspaceContent } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { statusLabel } from "./statusLabel";

export function ContentRow({
  content,
  onTriggerProcessing,
}: {
  content: TeacherCourseWorkspaceContent;
  onTriggerProcessing?: (content: TeacherCourseWorkspaceContent) => void;
}) {
  const tone = content.display_state === "failed_processing" ? "warn" : content.display_state === "ready" ? "good" : "neutral";
  const isVideoLike = content.content_type.startsWith("video/");
  const canProcess = isVideoLike && content.data_present && (content.display_state === "uploaded" || content.display_state === "failed_processing");
  return (
    <article className={styles.contentRow}>
      <div>
        <strong>{statusLabel(content.content_type)}</strong>
        <p>
          Order {content.order} - {content.data_present ? "Data recorded" : "No stored data"} - {statusLabel(content.publication_status)}
        </p>
      </div>
      <div style={{ display: "flex", gap: "0.5rem", alignItems: "center" }}>
        <span className={`${styles.statusPill} ${styles[tone]}`}>{statusLabel(content.display_state)}</span>
        {canProcess && onTriggerProcessing ? (
          <button
            className={styles.secondaryButton}
            onClick={() => onTriggerProcessing(content)}
            title="Queue video processing"
            type="button"
          >
            <Video size={16} aria-hidden />
            Process
          </button>
        ) : null}
      </div>
      {content.processing_status ? <small>{statusLabel(content.processing_status)}</small> : null}
      {content.processing_error ? <p className={styles.reviewNote}>{content.processing_error}</p> : null}
    </article>
  );
}
