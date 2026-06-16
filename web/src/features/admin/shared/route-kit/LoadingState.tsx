"use client";

import { Loader2 } from "lucide-react";
import styles from "@/features/admin/shared/admin-routes.module.css";

export function LoadingState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <div>
          <h2>Resolving platform access</h2>
          <p>RustLearn is loading the current user and platform permissions.</p>
        </div>
      </div>
    </section>
  );
}
