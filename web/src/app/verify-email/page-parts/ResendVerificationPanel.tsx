"use client";

import { AlertCircle, CheckCircle2, Loader2, MailCheck } from "lucide-react";
import { type FormEvent, useMemo, useState } from "react";
import { AuthRequestError, requestEmailVerification } from "@/lib/auth";
import styles from "../../auth.module.css";

type SubmitState = "idle" | "loading" | "success" | "error";

export function ResendVerificationPanel() {
  const [email, setEmail] = useState("");
  const [submitState, setSubmitState] = useState<SubmitState>("idle");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<{ code: string; message: string } | null>(null);
  const canSubmit = useMemo(() => email.trim().length > 0 && submitState !== "loading", [email, submitState]);

  async function submitResend(event: FormEvent<HTMLFormElement>) {
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
      setMessage(await requestEmailVerification({ email: normalizedEmail }));
      setSubmitState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Verification email request failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setSubmitState("error");
    }
  }

  return (
    <form className={styles.panel} onSubmit={submitResend}>
      <div className={styles.panelHeader}>
        <MailCheck size={22} aria-hidden />
        <h2>Resend verification</h2>
      </div>
      {message ? (
        <div className={styles.successBox} role="status">
          <strong>Check your email</strong>
          <span>{message}</span>
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
          <MailCheck size={18} aria-hidden />
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
        ) : (
          <CheckCircle2 size={18} aria-hidden />
        )}
        Send verification email
      </button>
    </form>
  );
}
