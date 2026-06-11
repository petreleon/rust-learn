"use client";

import { Bell, BriefcaseBusiness, LogIn, LogOut, Menu } from "lucide-react";
import Link from "next/link";
import { type CurrentSession } from "@/lib/session";
import styles from "../product-shell.module.css";
import { DelegationMenu } from "./DelegationMenu";
import { type ActiveNav } from "./ActiveNav";
import { type NavItem } from "./NavItem";

export function MobileMenu({
  accountLabel,
  activeNav,
  currentWorkspaceValue,
  handleWorkspaceChange,
  isSignedIn,
  navItems,
  onSignOut,
  session,
  showOperationsConsole,
  workspaceOptions,
}: {
  accountLabel: string;
  activeNav: ActiveNav;
  currentWorkspaceValue: string;
  handleWorkspaceChange: (event: React.ChangeEvent<HTMLSelectElement>) => void;
  isSignedIn: boolean;
  navItems: NavItem[];
  onSignOut?: () => void;
  session?: CurrentSession | null;
  showOperationsConsole: boolean;
  workspaceOptions: Array<{ label: string; value: string }>;
}) {
  return (
    <details className={styles.mobileMenu}>
      <summary className={styles.mobileSummary}>
        <Menu size={18} aria-hidden />
        Menu
      </summary>
      <div className={styles.mobilePanel}>
        {navItems.map((item) => {
          const Icon = item.icon;
          return (
            <Link
              aria-current={activeNav === item.key ? "page" : undefined}
              className={`${styles.mobileNavItem} ${activeNav === item.key ? styles.activeNav : ""}`}
              href={item.href}
              key={item.key}
            >
              <Icon size={17} aria-hidden />
              {item.label}
            </Link>
          );
        })}
        <label className={styles.mobileWorkspace}>
          <span>Workspace</span>
          <select
            aria-label="Mobile workspace"
            disabled={workspaceOptions.length === 0}
            value={currentWorkspaceValue}
            onChange={handleWorkspaceChange}
          >
            {workspaceOptions.length ? (
              workspaceOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))
            ) : (
              <option>No workspace</option>
            )}
          </select>
        </label>
        <button className={styles.accountMenuItem} disabled type="button">
          <Bell size={16} aria-hidden />
          Notifications
        </button>
        {showOperationsConsole ? (
          <Link className={styles.accountMenuItem} href="/ops">
            <BriefcaseBusiness size={16} aria-hidden />
            Operations console
          </Link>
        ) : null}
        <div className={styles.menuMeta}>
          <strong>{accountLabel}</strong>
          <span>{session?.user.email || "Sign in to load account details."}</span>
        </div>
        <DelegationMenu session={session} />
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
