"use client";

import { AlertTriangle, Mail, RotateCcw } from "lucide-react";
import Link from "next/link";
import { useState } from "react";
import styles from "../auth.module.css";

export default function ForgotPasswordPage() {
  const [email, setEmail] = useState("");

  return (
    <main className={styles.page}>
      <header className={styles.topbar}>
        <Link className={styles.brand} href="/">
          <span className={styles.brandMark}>RL</span>
          <span>
            <strong>RustLearn</strong>
            <small>Workspace</small>
          </span>
        </Link>
        <nav className={styles.topActions} aria-label="Password recovery links">
          <Link className={styles.navLink} href="/login">
            Login
          </Link>
          <Link className={styles.navLink} href="/register">
            Register
          </Link>
          <Link className={styles.navLink} href="/">
            Operations
          </Link>
        </nav>
      </header>

      <section className={styles.layout}>
        <div className={styles.hero}>
          <p className={styles.eyebrow}>Password recovery</p>
          <h1>Reset access to your workspace</h1>
          <p>
            Password reset email delivery is deferred until the backend reset-token
            contract is added.
          </p>
          <div className={styles.statusList}>
            <span>
              <AlertTriangle size={16} aria-hidden />
              Backend contract pending
            </span>
            <span>
              <RotateCcw size={16} aria-hidden />
              Route is ready for wiring
            </span>
          </div>
        </div>

        <form className={styles.panel}>
          <div className={styles.panelHeader}>
            <RotateCcw size={22} aria-hidden />
            <h2>Forgot password</h2>
          </div>

          <div className={styles.noticeBox} role="status">
            <strong>Password reset is not active yet</strong>
            <span>
              This screen is intentionally present so auth navigation is complete
              while reset-token APIs are still pending.
            </span>
          </div>

          <label className={styles.field}>
            <span>Email</span>
            <span className={styles.inputShell}>
              <Mail size={18} aria-hidden />
              <input
                autoComplete="email"
                inputMode="email"
                name="email"
                type="email"
                value={email}
                onChange={(event) => setEmail(event.target.value)}
              />
            </span>
          </label>

          <button className={styles.primaryButton} type="button" disabled>
            <Mail size={18} aria-hidden />
            Send reset email
          </button>

          <p className={styles.inlinePrompt}>
            Remembered your password? <Link href="/login">Sign in</Link>
          </p>
        </form>
      </section>
    </main>
  );
}
