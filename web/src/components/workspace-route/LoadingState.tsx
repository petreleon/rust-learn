"use client";

import { Loader2 } from "lucide-react";
import styles from "../workspace-route.module.css";

export function LoadingState() {
  return (
    <section className={styles.statePanel} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Resolving workspace</h2>
      </div>
      <div className={styles.skeletonGrid} aria-hidden>
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
      </div>
    </section>
  );
}
