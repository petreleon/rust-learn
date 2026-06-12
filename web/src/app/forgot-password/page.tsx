"use client";

import { AlertCircle, CheckCircle2, Loader2, Mail, RotateCcw } from "lucide-react";
import Link from "next/link";
import { type FormEvent, useMemo, useState } from "react";
import { AuthRequestError, requestPasswordReset } from "@/lib/auth";
import styles from "../auth.module.css";

type SubmitState = "idle" | "loading" | "success" | "error";

export default function ForgotPasswordPage() {
  const [email, setEmail] = useState("");
  const [submitState, setSubmitState] = useState<SubmitState>("idle");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<{ code: string; message: string } | null>(null);

  const canSubmit = useMemo(() => email.trim().length > 0 && submitState !== "loading", [email, submitState]);

  async function submitRequest(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const normalizedEmail = email.trim();
    if (!normalizedEmail) {
      setError({ code: "missing_email", message: "Email is required." });
      setSubmitState("error");
      return;
    }

    setSubmitState("loading");
    setError(null);
    setMessage(null);
    try {
      const nextMessage = await requestPasswordReset({ email: normalizedEmail });
      setMessage(nextMessage);
      setSubmitState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Password reset request failed.", 0, "network_error");
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
        <nav className={styles.topActions} aria-label="Password recovery links">
          <Link className={styles.navLink} href="/login">
            Login
          </Link>
          <Link className={styles.navLink} href="/register">
            Register
          </Link>
        </nav>
      </header>

      <section className={styles.layout}>
        <div className={styles.hero}>
          <p className={styles.eyebrow}>Password recovery</p>
          <h1>Reset access to your workspace</h1>
          <p>Enter your account email and RustLearn will send a one-time reset link if the account exists.</p>
          <div className={styles.statusList}>
            <span>
              <Mail size={16} aria-hidden />
              Private account lookup
            </span>
            <span>
              <RotateCcw size={16} aria-hidden />
              One-time reset link
            </span>
          </div>
        </div>

        <form className={styles.panel} onSubmit={submitRequest}>
          <div className={styles.panelHeader}>
            <RotateCcw size={22} aria-hidden />
            <h2>Forgot password</h2>
          </div>

          {message ? (
            <div className={styles.successBox} role="status">
              <strong>Check your email</strong>
              <span>{message}</span>
              <div className={styles.successActions}>
                <Link className={styles.secondaryLink} href="/login">
                  Return to login
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

          <button className={styles.primaryButton} type="submit" disabled={!canSubmit}>
            {submitState === "loading" ? (
              <Loader2 className={styles.spin} size={18} aria-hidden />
            ) : submitState === "success" ? (
              <CheckCircle2 size={18} aria-hidden />
            ) : (
              <Mail size={18} aria-hidden />
            )}
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
