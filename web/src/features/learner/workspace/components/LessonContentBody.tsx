"use client";

import { AlertTriangle, ExternalLink, FileText, Loader2 } from "lucide-react";
import { useEffect, useState } from "react";
import { fetchContentMediaUrl, type CourseLearningContent } from "@/lib/learner";
import { readStoredSessionToken } from "@/lib/session";
import styles from "../learner-workspace.module.css";
import { isReadableTextContent } from "./isReadableTextContent";
import { lessonStateCopy } from "./lessonStateCopy";

type MediaState = {
  error: boolean;
  key: string;
  url: string | null;
};

export function LessonContentBody({ content, courseId }: { content: CourseLearningContent; courseId: number }) {
  const [mediaState, setMediaState] = useState<MediaState | null>(null);

  const isVideo = content.content_type.trim().toLowerCase().startsWith("video/");
  const isDocument = content.content_type.trim().toLowerCase() === "application/pdf"
    || content.content_type.trim().toLowerCase() === "application/msword"
    || content.content_type.trim().toLowerCase() === "application/vnd.openxmlformats-officedocument.wordprocessingml.document";

  const shouldFetchMedia = content.display_state === "ready" && content.data && !isReadableTextContent(content.content_type) && (isVideo || isDocument);
  const mediaKey = shouldFetchMedia ? `${courseId}:${content.chapter_id}:${content.id}:${content.data}` : null;
  const activeMediaState = mediaState?.key === mediaKey ? mediaState : null;
  const mediaUrl = activeMediaState?.url ?? null;
  const mediaError = activeMediaState?.error ?? false;

  useEffect(() => {
    if (!shouldFetchMedia || !mediaKey) return;
    const token = readStoredSessionToken();
    if (!token) return;
    let cancelled = false;

    fetchContentMediaUrl({
      chapterId: content.chapter_id,
      contentId: content.id,
      courseId,
      token,
    })
      .then((result) => {
        if (!cancelled) setMediaState({ error: false, key: mediaKey, url: result.url });
      })
      .catch(() => {
        if (!cancelled) setMediaState({ error: true, key: mediaKey, url: null });
      });
    return () => {
      cancelled = true;
    };
  }, [content.chapter_id, content.id, courseId, mediaKey, shouldFetchMedia]);

  if (content.display_state === "ready" && isReadableTextContent(content.content_type) && content.data) {
    return <article className={styles.lessonText}>{content.data}</article>;
  }

  if (shouldFetchMedia && mediaUrl) {
    if (isVideo) {
      return (
        <article className={styles.mediaPlayer}>
          <video controls playsInline preload="metadata" style={{ maxWidth: "100%" }}>
            <source src={mediaUrl} type={content.content_type} />
            Your browser does not support inline video playback.
          </video>
        </article>
      );
    }
    if (isDocument) {
      return (
        <article className={styles.statePanel}>
          <div className={styles.panelHeader}>
            <FileText size={20} aria-hidden />
            <h3>Document ready</h3>
          </div>
          <p className={styles.muted}>Open the document in a new tab to view or download.</p>
          <a className={styles.primaryLink} href={mediaUrl} rel="noopener noreferrer" target="_blank" style={{ marginTop: "0.5rem" }}>
            <ExternalLink size={17} aria-hidden />
            Open document
          </a>
        </article>
      );
    }
  }

  if (shouldFetchMedia && !mediaUrl && !mediaError) {
    return (
      <article className={styles.statePanel}>
        <div className={styles.panelHeader}>
          <Loader2 className={styles.spin} size={20} aria-hidden />
          <h3>Loading media…</h3>
        </div>
        <p className={styles.muted}>Requesting a secure streaming link.</p>
      </article>
    );
  }

  if (shouldFetchMedia && mediaError) {
    return (
      <article className={styles.statePanel}>
        <div className={styles.panelHeader}>
          <AlertTriangle size={20} aria-hidden />
          <h3>Media unavailable</h3>
        </div>
        <p className={styles.muted}>Could not load media. The content may have been removed or the presigned link may have expired. Refresh to try again.</p>
      </article>
    );
  }

  const stateCopy = lessonStateCopy(content);
  return (
    <article className={styles.statePanel}>
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h3>{stateCopy.title}</h3>
      </div>
      <p className={styles.muted}>{stateCopy.detail}</p>
      {content.processing_error ? <p className={styles.muted}>Last error: {content.processing_error}</p> : null}
    </article>
  );
}
