"use client";

import { AlertTriangle, KeyRound, RotateCcw } from "lucide-react";
import Link from "next/link";
import { useState } from "react";
import styles from "../auth.module.css";

export default function ResetPasswordPage() {
  const [token, setToken] = useState("");
  const [password, setPassword] = useState("");

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
        <nav className={styles.topActions} aria-label="Password reset links">
          <Link className={styles.navLink} href="/login">
            Login
          </Link>
          <Link className={styles.navLink} href="/forgot-password">
            Forgot password
          </Link>
          <Link className={styles.navLink} href="/ops">
            Operations
          </Link>
        </nav>
      </header>

      <section className={styles.layout}>
        <div className={styles.hero}>
          <p className={styles.eyebrow}>Password reset</p>
          <h1>Choose a new password</h1>
          <p>
            This route is reserved for reset-token completion once the backend
            issues and validates password reset tokens.
          </p>
          <div className={styles.statusList}>
            <span>
              <AlertTriangle size={16} aria-hidden />
              Token API pending
            </span>
            <span>
              <KeyRound size={16} aria-hidden />
              Strong policy will apply
            </span>
          </div>
        </div>

        <form className={styles.panel}>
          <div className={styles.panelHeader}>
            <RotateCcw size={22} aria-hidden />
            <h2>Reset password</h2>
          </div>

          <div className={styles.noticeBox} role="status">
            <strong>Password reset is deferred</strong>
            <span>
              The route is available now, but submitting a new password waits for
              reset-token backend support.
            </span>
          </div>

          <label className={styles.field}>
            <span>Reset token</span>
            <span className={styles.inputShell}>
              <KeyRound size={18} aria-hidden />
              <input
                autoComplete="one-time-code"
                name="token"
                type="text"
                value={token}
                onChange={(event) => setToken(event.target.value)}
              />
            </span>
          </label>

          <label className={styles.field}>
            <span>New password</span>
            <span className={styles.inputShell}>
              <KeyRound size={18} aria-hidden />
              <input
                autoComplete="new-password"
                name="password"
                type="password"
                value={password}
                onChange={(event) => setPassword(event.target.value)}
              />
            </span>
          </label>

          <button className={styles.primaryButton} type="button" disabled>
            <KeyRound size={18} aria-hidden />
            Update password
          </button>

          <p className={styles.inlinePrompt}>
            Need a reset token? <Link href="/forgot-password">Request reset email</Link>
          </p>
        </form>
      </section>
    </main>
  );
}
