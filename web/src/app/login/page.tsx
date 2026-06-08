"use client";

import { AlertCircle, Eye, EyeOff, KeyRound, Loader2, LogIn, Mail } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { type FormEvent, useMemo, useState } from "react";
import { AuthRequestError, loginWithPassword } from "@/lib/auth";
import { storeSessionToken } from "@/lib/session";
import styles from "./page.module.css";

type SubmitState = "idle" | "loading" | "error";

export default function LoginPage() {
  const router = useRouter();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [submitState, setSubmitState] = useState<SubmitState>("idle");
  const [error, setError] = useState<{ code: string; message: string } | null>(null);

  const canSubmit = useMemo(() => {
    return email.trim().length > 0 && password.length > 0 && submitState !== "loading";
  }, [email, password, submitState]);

  async function submitLogin(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const normalizedEmail = email.trim();

    if (!normalizedEmail || !password) {
      setError({ code: "missing_fields", message: "Email and password are required." });
      setSubmitState("error");
      return;
    }

    setSubmitState("loading");
    setError(null);

    try {
      const token = await loginWithPassword({ email: normalizedEmail, password });
      storeSessionToken(token);
      router.replace(intendedRoute());
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Login failed.", 0, "network_error");
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
        <nav className={styles.topActions} aria-label="Login links">
          <Link className={styles.navLink} href="/session">
            Session
          </Link>
          <Link className={styles.navLink} href="/">
            Operations
          </Link>
        </nav>
      </header>

      <section className={styles.layout}>
        <div className={styles.hero}>
          <p className={styles.eyebrow}>Sign in</p>
          <h1>Open your RustLearn workspace</h1>
          <p>
            Use the account attached to your learner, teacher, organization, or platform access.
          </p>
          <div className={styles.statusList}>
            <span>
              <KeyRound size={16} aria-hidden />
              Email verification required
            </span>
            <span>
              <LogIn size={16} aria-hidden />
              Redirects to requested route
            </span>
          </div>
        </div>

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

          {error ? (
            <div className={styles.errorBox} role="status">
              <AlertCircle size={18} aria-hidden />
              <span>
                <strong>{error.code}</strong>
                {error.message}
              </span>
            </div>
          ) : null}

          <button className={styles.primaryButton} type="submit" disabled={!canSubmit}>
            {submitState === "loading" ? (
              <Loader2 className={styles.spin} size={18} aria-hidden />
            ) : (
              <LogIn size={18} aria-hidden />
            )}
            Sign in
          </button>
        </form>
      </section>
    </main>
  );
}

function intendedRoute() {
  if (typeof window === "undefined") {
    return "/session";
  }

  const redirect = new URLSearchParams(window.location.search).get("redirect");
  if (redirect?.startsWith("/") && !redirect.startsWith("//")) {
    return redirect;
  }

  return "/session";
}
