"use client";

import { type FormEvent, useCallback, useEffect, useState } from "react";
import { AlertCircle, CheckCircle2, KeyRound, Loader2, MailCheck } from "lucide-react";
import Link from "next/link";
import { useSearchParams } from "next/navigation";
import { AuthRequestError, verifyEmailToken, type VerifyEmailResult } from "@/lib/auth";
import styles from "../../auth.module.css";
import { ResendVerificationPanel } from "./ResendVerificationPanel";
import { type VerifyState } from "./VerifyState";

export function VerifyEmailContent() {
  const searchParams = useSearchParams();
  const queryToken = searchParams.get("token") || "";
  const [token, setToken] = useState(queryToken);
  const [verifiedToken, setVerifiedToken] = useState("");
  const [verifyState, setVerifyState] = useState<VerifyState>("idle");
  const [result, setResult] = useState<VerifyEmailResult | null>(null);
  const [error, setError] = useState<{ code: string; message: string } | null>(null);

  const runVerification = useCallback(async (nextToken: string) => {
    setVerifyState("loading");
    setError(null);
    setResult(null);

    try {
      const nextResult = await verifyEmailToken({ token: nextToken });
      setResult(nextResult);
      setVerifiedToken(nextToken);
      setVerifyState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Email verification failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setVerifyState("error");
    }
  }, []);

  useEffect(() => {
    if (!queryToken || queryToken === verifiedToken) {
      return undefined;
    }

    const timeout = window.setTimeout(() => {
      setToken(queryToken);
      void runVerification(queryToken);
    }, 0);

    return () => window.clearTimeout(timeout);
  }, [queryToken, runVerification, verifiedToken]);

  function submitVerification(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void runVerification(token);
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
        <nav className={styles.topActions} aria-label="Verification links">
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
          <p className={styles.eyebrow}>Verify email</p>
          <h1>Confirm your RustLearn account</h1>
          <p>Complete account verification before login so the workspace can load your profile and permissions.</p>
          <div className={styles.statusList}>
            <span>
              <MailCheck size={16} aria-hidden />
              Required before login
            </span>
            <span>
              <KeyRound size={16} aria-hidden />
              Token expires after 24 hours
            </span>
          </div>
        </div>

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
        <ResendVerificationPanel />
      </section>
    </main>
  );
}
