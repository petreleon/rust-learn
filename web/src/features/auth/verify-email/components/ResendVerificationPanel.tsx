"use client";

import { CheckCircle2, Loader2, MailCheck } from "lucide-react";
import { type FormEventHandler } from "react";
import { AuthErrorBox } from "../../shared/components/AuthErrorBox";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import styles from "../../shared/auth.module.css";
import { type ResendVerificationSubmitState } from "../model/ResendVerificationSubmitState";

export type ResendVerificationPanelProps = {
  canSubmit: boolean;
  email: string;
  error: AuthFormError | null;
  message: string | null;
  setEmail: (value: string) => void;
  submitResend: FormEventHandler<HTMLFormElement>;
  submitState: ResendVerificationSubmitState;
};

export function ResendVerificationPanel({
  canSubmit,
  email,
  error,
  message,
  setEmail,
  submitResend,
  submitState,
}: ResendVerificationPanelProps) {
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
      {error ? <AuthErrorBox error={error} /> : null}
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
