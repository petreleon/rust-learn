"use client";

import { Loader2 } from "lucide-react";
import styles from "@/features/admin/shared/admin-routes.module.css";

export function PanelLoading({ title }: { title: string }) {
  return (
    <section className={styles.panel} role="status">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <div>
          <h2>{title}</h2>
          <p>RustLearn is resolving the latest platform data.</p>
        </div>
      </div>
      <div className={styles.skeletonGrid}>
        <span />
        <span />
        <span />
      </div>
    </section>
  );
}
