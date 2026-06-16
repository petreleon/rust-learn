import { CheckCircle2, Loader2, Mail, RotateCcw } from "lucide-react";
import Link from "next/link";
import { type FormEventHandler } from "react";
import { AuthErrorBox } from "../../shared/components/AuthErrorBox";
import { AuthHeader } from "../../shared/components/AuthHeader";
import { AuthHero } from "../../shared/components/AuthHero";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import styles from "../../shared/auth.module.css";
import { type ForgotPasswordSubmitState } from "../model/ForgotPasswordSubmitState";

type ForgotPasswordViewProps = {
  canSubmit: boolean;
  email: string;
  error: AuthFormError | null;
  message: string | null;
  setEmail: (value: string) => void;
  submitRequest: FormEventHandler<HTMLFormElement>;
  submitState: ForgotPasswordSubmitState;
};

export function ForgotPasswordView({
  canSubmit,
  email,
  error,
  message,
  setEmail,
  submitRequest,
  submitState,
}: ForgotPasswordViewProps) {
  return (
    <main className={styles.page}>
      <AuthHeader
        navLabel="Password recovery links"
        links={[
          { href: "/login", label: "Login" },
          { href: "/register", label: "Register" },
        ]}
      />
      <section className={styles.layout}>
        <AuthHero
          eyebrow="Password recovery"
          title="Reset access to your workspace"
          description="Enter your account email and RustLearn will send a one-time reset link if the account exists."
          statusItems={[
            { icon: <Mail size={16} aria-hidden />, label: "Private account lookup" },
            { icon: <RotateCcw size={16} aria-hidden />, label: "One-time reset link" },
          ]}
        />
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
          {error ? <AuthErrorBox error={error} /> : null}
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
