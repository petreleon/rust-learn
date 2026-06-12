"use client";

import { Loader2, LogIn, ShieldCheck, UserPlus } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
} from "@/lib/session";
import styles from "./page.module.css";

export default function ProductEntryPage() {
  const router = useRouter();

  useEffect(() => {
    const timeout = window.setTimeout(async () => {
      const token = readStoredSessionToken();
      if (!token) {
        router.replace("/login");
        return;
      }

      try {
        await fetchCurrentSession({ token });
        router.replace("/session");
      } catch {
        clearStoredSessionToken();
        router.replace("/login");
      }
    }, 0);

    return () => window.clearTimeout(timeout);
  }, [router]);

  return (
    <main className={styles.page}>
      <header className={styles.topbar}>
        <Link className={styles.brand} href="/">
          <span className={styles.brandMark}>RL</span>
          <span>
            <strong>RustLearn</strong>
            <small>Product</small>
          </span>
        </Link>
        <nav className={styles.topActions} aria-label="Entry links">
          <Link className={styles.navLink} href="/login">
            Login
          </Link>
          <Link className={styles.navLink} href="/register">
            Register
          </Link>
        </nav>
      </header>

      <section className={styles.hero}>
        <p className={styles.eyebrow}>RustLearn</p>
        <h1>Opening your workspace</h1>
        <p>
          The app is resolving the current session before sending you to the right
          product surface.
        </p>
        <div className={styles.statusList}>
          <span>
            <Loader2 className={styles.spin} size={16} aria-hidden />
            Checking session
          </span>
          <span>
            <ShieldCheck size={16} aria-hidden />
            Permission-aware entry
          </span>
        </div>
        <div className={styles.actionRow}>
          <Link className={styles.primaryLink} href="/login">
            <LogIn size={18} aria-hidden />
            Sign in
          </Link>
          <Link className={styles.secondaryLink} href="/register">
            <UserPlus size={18} aria-hidden />
            Register
          </Link>
        </div>
      </section>
    </main>
  );
}
