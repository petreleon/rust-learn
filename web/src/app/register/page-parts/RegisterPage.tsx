"use client";

import { type FormEvent, useMemo, useState } from "react";
import Link from "next/link";
import { Mail, KeyRound, UserPlus } from "lucide-react";
import { AuthRequestError, registerAccount } from "@/lib/auth";
import styles from "../../auth.module.css";
import { policyChecks } from "./policyChecks";
import { RegisterFormBody } from "./RegisterFormBody";
import { RegistrationSuccessPanel } from "./RegistrationSuccessPanel";
import { type SubmitState } from "./SubmitState";

export default function RegisterPage() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [dateOfBirth, setDateOfBirth] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [submitState, setSubmitState] = useState<SubmitState>("idle");
  const [error, setError] = useState<{ code: string; message: string } | null>(null);
  const policyResults = useMemo(
    () => policyChecks.map((check) => ({ ...check, met: check.test(password) })),
    [password],
  );
  const passwordMeetsPolicy = policyResults.every((check) => check.met);
  const canSubmit = name.trim().length > 0 && email.trim().length > 0 && password.length > 0 && submitState !== "loading";

  async function submitRegistration(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const normalizedName = name.trim();
    const normalizedEmail = email.trim();
    if (!normalizedName || !normalizedEmail || !password) {
      setError({ code: "missing_fields", message: "Name, email, and password are required." });
      setSubmitState("error");
      return;
    }
    if (!passwordMeetsPolicy) {
      setError({ code: "password_policy", message: "Password does not meet the policy." });
      setSubmitState("error");
      return;
    }
    setSubmitState("loading");
    setError(null);
    try {
      await registerAccount({ dateOfBirth, email: normalizedEmail, name: normalizedName, password });
      setSubmitState("success");
      setPassword("");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError ? nextError : new AuthRequestError("Registration failed.", 0, "network_error");
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
        <nav className={styles.topActions} aria-label="Registration links">
          <Link className={styles.navLink} href="/login">
            Login
          </Link>
          <Link className={styles.navLink} href="/session">
            Session
          </Link>
        </nav>
      </header>
      <section className={styles.layout}>
        <div className={styles.hero}>
          <p className={styles.eyebrow}>Register</p>
          <h1>Create your RustLearn account</h1>
          <p>Start with learner access, verify your email, then grow into teacher, organization, or platform scopes as permissions are granted.</p>
          <div className={styles.statusList}>
            <span>
              <Mail size={16} aria-hidden /> Verification email required
            </span>
            <span>
              <KeyRound size={16} aria-hidden /> Strong password policy
            </span>
          </div>
        </div>
        <form className={styles.panel} onSubmit={submitRegistration}>
          <div className={styles.panelHeader}>
            <UserPlus size={22} aria-hidden />
            <h2>Register</h2>
          </div>
          {submitState === "success" ? (
            <RegistrationSuccessPanel />
          ) : (
            <RegisterFormBody
              canSubmit={canSubmit}
              dateOfBirth={dateOfBirth}
              email={email}
              error={error}
              name={name}
              password={password}
              policyResults={policyResults}
              setDateOfBirth={setDateOfBirth}
              setEmail={setEmail}
              setName={setName}
              setPassword={setPassword}
              setShowPassword={setShowPassword}
              showPassword={showPassword}
              submitState={submitState}
            />
          )}
        </form>
      </section>
    </main>
  );
}
