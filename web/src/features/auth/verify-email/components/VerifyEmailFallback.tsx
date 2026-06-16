"use client";

import { Loader2 } from "lucide-react";
import styles from "../../shared/auth.module.css";

export function VerifyEmailFallback() {
  return (
    <main className={styles.page}>
      <section className={styles.layout}>
        <div className={styles.hero}>
          <p className={styles.eyebrow}>Verify email</p>
          <h1>Confirm your RustLearn account</h1>
        </div>
        <div className={styles.panel}>
          <div className={styles.panelHeader}>
            <Loader2 className={styles.spin} size={22} aria-hidden />
            <h2>Email verification</h2>
          </div>
        </div>
      </section>
    </main>
  );
}
