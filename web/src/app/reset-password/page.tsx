"use client";

import { AlertCircle, CheckCircle2, KeyRound, Loader2, RotateCcw } from "lucide-react";
import Link from "next/link";
import { type FormEvent, useMemo, useState } from "react";
import { AuthRequestError, resetPassword } from "@/lib/auth";
import { policyChecks } from "../register/page-parts/policyChecks";
import styles from "../auth.module.css";

type SubmitState = "idle" | "loading" | "success" | "error";

export default function ResetPasswordPage() {
  const [token, setToken] = useState(() => {
    if (typeof window === "undefined") {
      return "";
    }
    return new URLSearchParams(window.location.search).get("token") || "";
  });
  const [password, setPassword] = useState("");
  const [submitState, setSubmitState] = useState<SubmitState>("idle");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<{ code: string; message: string } | null>(null);

  const policyResults = useMemo(() => {
    return policyChecks.map((check) => ({ ...check, met: check.test(password) }));
  }, [password]);
  const canSubmit = token.trim().length > 0 && password.length > 0 && submitState !== "loading";

  async function submitReset(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!token.trim() || !password) {
      setError({ code: "missing_fields", message: "Reset token and new password are required." });
      setSubmitState("error");
      return;
    }

    setSubmitState("loading");
    setError(null);
    setMessage(null);
    try {
      const nextMessage = await resetPassword({ password, token: token.trim() });
      setMessage(nextMessage);
      setPassword("");
      setSubmitState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Password reset failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setSubmitState("error");
    }
  }

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
        </nav>
      </header>

      <section className={styles.layout}>
        <div className={styles.hero}>
          <p className={styles.eyebrow}>Password reset</p>
          <h1>Choose a new password</h1>
          <p>Use the token from your reset email and choose a new password for your RustLearn account.</p>
          <div className={styles.statusList}>
            <span>
              <KeyRound size={16} aria-hidden />
              One-time token
            </span>
            <span>
              <CheckCircle2 size={16} aria-hidden />
              Strong password required
            </span>
          </div>
        </div>

        <form className={styles.panel} onSubmit={submitReset}>
          <div className={styles.panelHeader}>
            <RotateCcw size={22} aria-hidden />
            <h2>Reset password</h2>
          </div>

          {message ? (
            <div className={styles.successBox} role="status">
              <strong>Password updated</strong>
              <span>{message}</span>
              <div className={styles.successActions}>
                <Link className={styles.secondaryLink} href="/login">
                  Sign in
                </Link>
              </div>
            </div>
          ) : null}

          {error ? (
            <div className={styles.errorBox} role="status">
              <AlertCircle size={18} aria-hidden />
              <span>
                <strong>{error.code}</strong>
                {error.message}
              </span>
            </div>
          ) : null}

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

          <div className={styles.policyList} aria-label="Password policy">
            {policyResults.map((check) => (
              <span className={check.met ? styles.met : styles.unmet} key={check.key}>
                <CheckCircle2 size={14} aria-hidden />
                {check.label}
              </span>
            ))}
          </div>

          <button className={styles.primaryButton} type="submit" disabled={!canSubmit}>
            {submitState === "loading" ? (
              <Loader2 className={styles.spin} size={18} aria-hidden />
            ) : submitState === "success" ? (
              <CheckCircle2 size={18} aria-hidden />
            ) : (
              <KeyRound size={18} aria-hidden />
            )}
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
