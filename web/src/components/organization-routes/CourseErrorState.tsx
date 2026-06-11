"use client";

import { AlertTriangle, RefreshCw } from "lucide-react";
import styles from "../organization-routes.module.css";
import { type RouteError } from "./RouteError";

export function CourseErrorState({
  error,
  onRetry,
}: {
  error: RouteError | null;
  onRetry: () => void;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error?.code || "course_error"}</strong>
        <span>{error?.message || "Organization courses could not be loaded."}</span>
      </span>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}
