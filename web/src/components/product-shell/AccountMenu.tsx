"use client";

import { BriefcaseBusiness, ChevronDown, LogIn, LogOut, Settings, UserCircle } from "lucide-react";
import Link from "next/link";
import { type CurrentSession } from "@/lib/session";
import styles from "../product-shell.module.css";
import { DelegationMenu } from "./DelegationMenu";

export function AccountMenu({
  accountLabel,
  isSignedIn,
  onSignOut,
  session,
  showOperationsConsole,
}: {
  accountLabel: string;
  isSignedIn: boolean;
  onSignOut?: () => void;
  session?: CurrentSession | null;
  showOperationsConsole: boolean;
}) {
  return (
    <details className={styles.accountMenu}>
      <summary className={styles.accountSummary}>
        <UserCircle size={18} aria-hidden />
        <span className={styles.accountLabel}>{accountLabel}</span>
        <ChevronDown size={16} aria-hidden />
      </summary>
      <div className={styles.menuPanel}>
        <div className={styles.menuMeta}>
          <strong>{session?.user.name || (isSignedIn ? "Session resolving" : "Not signed in")}</strong>
          <span>{session?.user.email || "Sign in to load account details."}</span>
        </div>
        <DelegationMenu session={session} />
        <Link className={styles.accountMenuItem} href="/settings/account">
          <Settings size={16} aria-hidden />
          Account settings
        </Link>
        {showOperationsConsole ? (
          <Link className={styles.accountMenuItem} href="/ops">
            <BriefcaseBusiness size={16} aria-hidden />
            Operations console
          </Link>
        ) : null}
        {isSignedIn && onSignOut ? (
          <button className={styles.accountMenuItem} type="button" onClick={onSignOut}>
            <LogOut size={16} aria-hidden />
            Sign out
          </button>
        ) : (
          <Link className={styles.accountMenuItem} href="/login">
            <LogIn size={16} aria-hidden />
            Sign in
          </Link>
        )}
      </div>
    </details>
  );
}
