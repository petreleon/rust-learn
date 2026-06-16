import { Eye, EyeOff, KeyRound, Loader2, LogIn, Mail } from "lucide-react";
import Link from "next/link";
import { type Dispatch, type FormEventHandler, type SetStateAction } from "react";
import { AuthErrorBox } from "../../shared/components/AuthErrorBox";
import { AuthHeader } from "../../shared/components/AuthHeader";
import { AuthHero } from "../../shared/components/AuthHero";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import styles from "../../shared/auth.module.css";
import { type LoginSubmitState } from "../model/LoginSubmitState";

type LoginViewProps = {
  canSubmit: boolean;
  email: string;
  error: AuthFormError | null;
  password: string;
  setEmail: (value: string) => void;
  setPassword: (value: string) => void;
  setShowPassword: Dispatch<SetStateAction<boolean>>;
  showPassword: boolean;
  submitLogin: FormEventHandler<HTMLFormElement>;
  submitState: LoginSubmitState;
};

export function LoginView({
  canSubmit,
  email,
  error,
  password,
  setEmail,
  setPassword,
  setShowPassword,
  showPassword,
  submitLogin,
  submitState,
}: LoginViewProps) {
  return (
    <main className={styles.page}>
      <AuthHeader
        navLabel="Login links"
        links={[
          { href: "/session", label: "Session" },
          { href: "/register", label: "Register" },
        ]}
      />
      <section className={styles.layout}>
        <AuthHero
          eyebrow="Sign in"
          title="Open your RustLearn workspace"
          description="Use the account attached to your learner, teacher, organization, or platform access."
          statusItems={[
            { icon: <KeyRound size={16} aria-hidden />, label: "Email verification required" },
            { icon: <LogIn size={16} aria-hidden />, label: "Shared across browser tabs" },
          ]}
        />
        <form className={styles.panel} onSubmit={submitLogin}>
          <div className={styles.panelHeader}>
            <LogIn size={22} aria-hidden />
            <h2>Login</h2>
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
          <label className={styles.field}>
            <span>Password</span>
            <span className={styles.inputShell}>
              <KeyRound size={18} aria-hidden />
              <input
                autoComplete="current-password"
                name="password"
                type={showPassword ? "text" : "password"}
                value={password}
                onChange={(event) => setPassword(event.target.value)}
              />
              <button
                aria-label={showPassword ? "Hide password" : "Show password"}
                className={styles.iconButton}
                type="button"
                onClick={() => setShowPassword((current) => !current)}
              >
                {showPassword ? <EyeOff size={18} aria-hidden /> : <Eye size={18} aria-hidden />}
              </button>
            </span>
          </label>
          {error ? <AuthErrorBox error={error} /> : null}
          <button className={styles.primaryButton} type="submit" disabled={!canSubmit}>
            {submitState === "loading" ? (
              <Loader2 className={styles.spin} size={18} aria-hidden />
            ) : (
              <LogIn size={18} aria-hidden />
            )}
            Sign in
          </button>
          <p className={styles.inlinePrompt}>
            New to RustLearn? <Link href="/register">Create an account</Link>
          </p>
          <p className={styles.inlinePrompt}>
            Forgot your password? <Link href="/forgot-password">Reset access</Link>
          </p>
        </form>
      </section>
    </main>
  );
}
