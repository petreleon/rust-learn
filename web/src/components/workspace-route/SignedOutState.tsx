"use client";

import { LogIn } from "lucide-react";
import Link from "next/link";
import styles from "../workspace-route.module.css";

export function SignedOutState({ redirect }: { redirect: string }) {
  return (
    <section className={styles.statePanel}>
      <div className={styles.panelHeader}>
        <LogIn size={20} aria-hidden />
        <h2>Sign in required</h2>
      </div>
      <p className={styles.muted}>
        This workspace is shown after RustLearn resolves your current account and scoped
        permissions.
      </p>
      <Link className={styles.primaryLink} href={`/login?redirect=${redirect}`}>
        <LogIn size={18} aria-hidden />
        Sign in
      </Link>
    </section>
  );
}
