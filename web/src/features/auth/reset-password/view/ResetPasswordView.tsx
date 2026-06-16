import { CheckCircle2, KeyRound, Loader2, RotateCcw } from "lucide-react";
import Link from "next/link";
import { type FormEventHandler } from "react";
import { AuthErrorBox } from "../../shared/components/AuthErrorBox";
import { AuthHeader } from "../../shared/components/AuthHeader";
import { AuthHero } from "../../shared/components/AuthHero";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import { type PasswordPolicyResult } from "../../shared/model/passwordPolicy";
import styles from "../../shared/auth.module.css";
import { type ResetPasswordSubmitState } from "../model/ResetPasswordSubmitState";

type ResetPasswordViewProps = {
  canSubmit: boolean;
  error: AuthFormError | null;
  message: string | null;
  password: string;
  policyResults: PasswordPolicyResult[];
  setPassword: (value: string) => void;
  setToken: (value: string) => void;
  submitReset: FormEventHandler<HTMLFormElement>;
  submitState: ResetPasswordSubmitState;
  token: string;
};

export function ResetPasswordView({
  canSubmit,
  error,
  message,
  password,
  policyResults,
  setPassword,
  setToken,
  submitReset,
  submitState,
  token,
}: ResetPasswordViewProps) {
  return (
    <main className={styles.page}>
      <AuthHeader
        navLabel="Password reset links"
        links={[
          { href: "/login", label: "Login" },
          { href: "/forgot-password", label: "Forgot password" },
        ]}
      />
      <section className={styles.layout}>
        <AuthHero
          eyebrow="Password reset"
          title="Choose a new password"
          description="Use the token from your reset email and choose a new password for your RustLearn account."
          statusItems={[
            { icon: <KeyRound size={16} aria-hidden />, label: "One-time token" },
            { icon: <CheckCircle2 size={16} aria-hidden />, label: "Strong password required" },
          ]}
        />
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
          {error ? <AuthErrorBox error={error} /> : null}
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
