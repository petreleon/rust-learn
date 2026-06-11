"use client";

import { LogIn } from "lucide-react";
import Link from "next/link";
import styles from "../learner-routes.module.css";
import { loginHref } from "./loginHref";

export function SignedOutState({ redirect }: { redirect: string }) {
  return (
    <section className={styles.statePanel}>
      <div className={styles.panelHeader}>
        <LogIn size={20} aria-hidden />
        <h2>Sign in required</h2>
      </div>
      <p className={styles.muted}>Learner routes load after RustLearn resolves your current session.</p>
      <Link className={styles.primaryLink} href={loginHref(redirect)}>
        <LogIn size={18} aria-hidden />
        Sign in
      </Link>
    </section>
  );
}
