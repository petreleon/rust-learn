import { CheckCircle2, KeyRound, Loader2, MailCheck } from "lucide-react";
import Link from "next/link";
import { type FormEventHandler } from "react";
import { type VerifyEmailResult } from "@/lib/auth";
import { AuthErrorBox } from "../../shared/components/AuthErrorBox";
import { AuthHeader } from "../../shared/components/AuthHeader";
import { AuthHero } from "../../shared/components/AuthHero";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import styles from "../../shared/auth.module.css";
import { ResendVerificationPanel, type ResendVerificationPanelProps } from "../components/ResendVerificationPanel";
import { type VerifyState } from "../model/VerifyState";

type VerifyEmailViewProps = {
  error: AuthFormError | null;
  resendVerification: ResendVerificationPanelProps;
  result: VerifyEmailResult | null;
  setToken: (value: string) => void;
  submitVerification: FormEventHandler<HTMLFormElement>;
  token: string;
  verifyState: VerifyState;
};

export function VerifyEmailView({
  error,
  resendVerification,
  result,
  setToken,
  submitVerification,
  token,
  verifyState,
}: VerifyEmailViewProps) {
  return (
    <main className={styles.page}>
      <AuthHeader
        navLabel="Verification links"
        links={[
          { href: "/login", label: "Login" },
          { href: "/register", label: "Register" },
        ]}
      />
      <section className={styles.layout}>
        <AuthHero
          eyebrow="Verify email"
          title="Confirm your RustLearn account"
          description="Complete account verification before login so the workspace can load your profile and permissions."
          statusItems={[
            { icon: <MailCheck size={16} aria-hidden />, label: "Required before login" },
            { icon: <KeyRound size={16} aria-hidden />, label: "Token expires after 24 hours" },
          ]}
        />
        <form className={styles.panel} onSubmit={submitVerification}>
          <div className={styles.panelHeader}>
            <MailCheck size={22} aria-hidden />
            <h2>Email verification</h2>
          </div>
          {result ? (
            <div className={styles.successBox} role="status">
              <strong>{result.state === "already_verified" ? "Email already verified" : "Email verified"}</strong>
              <span>{result.message}</span>
              <div className={styles.successActions}>
                <Link className={styles.secondaryLink} href="/login">
                  Go to login
                </Link>
              </div>
            </div>
          ) : null}
          {error ? <AuthErrorBox error={error} /> : null}
          <label className={styles.field}>
            <span>Verification token</span>
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
          <button className={styles.primaryButton} type="submit" disabled={verifyState === "loading" || !token.trim()}>
            {verifyState === "loading" ? (
              <Loader2 className={styles.spin} size={18} aria-hidden />
            ) : (
              <CheckCircle2 size={18} aria-hidden />
            )}
            Verify email
          </button>
        </form>
        <ResendVerificationPanel {...resendVerification} />
      </section>
    </main>
  );
}
