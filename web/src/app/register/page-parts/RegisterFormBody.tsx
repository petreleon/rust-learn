"use client";

import Link from "next/link";
import { AlertCircle, CalendarDays, CheckCircle2, Eye, EyeOff, KeyRound, Loader2, Mail, UserPlus, UserRound } from "lucide-react";
import styles from "../../auth.module.css";
import { type policyChecks } from "./policyChecks";
import { type SubmitState } from "./SubmitState";

type PolicyResult = (typeof policyChecks)[number] & { met: boolean };

type RegisterFormBodyProps = {
  canSubmit: boolean;
  dateOfBirth: string;
  email: string;
  error: { code: string; message: string } | null;
  name: string;
  password: string;
  policyResults: PolicyResult[];
  setDateOfBirth: (value: string) => void;
  setEmail: (value: string) => void;
  setName: (value: string) => void;
  setPassword: (value: string) => void;
  setShowPassword: (update: (current: boolean) => boolean) => void;
  showPassword: boolean;
  submitState: SubmitState;
};

export function RegisterFormBody({
  canSubmit,
  dateOfBirth,
  email,
  error,
  name,
  password,
  policyResults,
  setDateOfBirth,
  setEmail,
  setName,
  setPassword,
  setShowPassword,
  showPassword,
  submitState,
}: RegisterFormBodyProps) {
  return (
    <>
      <label className={styles.field}>
        <span>Name</span>
        <span className={styles.inputShell}>
          <UserRound size={18} aria-hidden />
          <input autoComplete="name" name="name" type="text" value={name} onChange={(event) => setName(event.target.value)} />
        </span>
      </label>
      <label className={styles.field}>
        <span>Email</span>
        <span className={styles.inputShell}>
          <Mail size={18} aria-hidden />
          <input autoComplete="email" inputMode="email" name="email" type="email" value={email} onChange={(event) => setEmail(event.target.value)} />
        </span>
      </label>
      <label className={styles.field}>
        <span>Date of birth</span>
        <span className={styles.inputShell}>
          <CalendarDays size={18} aria-hidden />
          <input name="dateOfBirth" type="date" value={dateOfBirth} onChange={(event) => setDateOfBirth(event.target.value)} />
        </span>
      </label>
      <label className={styles.field}>
        <span>Password</span>
        <span className={styles.inputShell}>
          <KeyRound size={18} aria-hidden />
          <input
            autoComplete="new-password"
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
      <div className={styles.policyList} aria-label="Password policy">
        {policyResults.map((check) => (
          <span className={check.met ? styles.met : styles.unmet} key={check.key}>
            <CheckCircle2 size={14} aria-hidden /> {check.label}
          </span>
        ))}
      </div>
      {error ? (
        <div className={styles.errorBox} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong> {error.message}
          </span>
        </div>
      ) : null}
      <button className={styles.primaryButton} type="submit" disabled={!canSubmit}>
        {submitState === "loading" ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <UserPlus size={18} aria-hidden />}
        Create account
      </button>
      <p className={styles.inlinePrompt}>
        Already have an account? <Link href="/login">Sign in</Link>
      </p>
    </>
  );
}
