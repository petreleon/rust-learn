"use client";

import {
  Bell,
  BriefcaseBusiness,
  ChevronDown,
  Home,
  LogIn,
  LogOut,
  Menu,
  Settings,
  UserCircle,
  type LucideIcon,
} from "lucide-react";
import Link from "next/link";
import { type ReactNode } from "react";
import { accessSummary } from "@/lib/access";
import { type CurrentSession } from "@/lib/session";
import styles from "./product-shell.module.css";

type ActiveNav = "session" | "learn" | "teach" | "organizations" | "admin" | "account" | "none";

type Breadcrumb = {
  label: string;
  href?: string;
};

export type ShellNotice = {
  actionHref?: string;
  actionLabel?: string;
  message: string;
  title: string;
  tone: "info" | "success" | "warn" | "error";
};

type NavItem = {
  href: string;
  icon: LucideIcon;
  key: ActiveNav;
  label: string;
};

type ProductShellProps = {
  activeNav: ActiveNav;
  breadcrumbs?: Breadcrumb[];
  children: ReactNode;
  description: string;
  eyebrow: string;
  isSignedIn?: boolean;
  notice?: ShellNotice | null;
  onSignOut?: () => void;
  session?: CurrentSession | null;
  statusItems?: ReactNode;
  title: string;
};

const baseNavItems: NavItem[] = [
  { key: "session", href: "/session", label: "Workspace", icon: Home },
  { key: "account", href: "/settings/account", label: "Account", icon: Settings },
];

export function ProductShell({
  activeNav,
  breadcrumbs = [],
  children,
  description,
  eyebrow,
  isSignedIn = false,
  notice,
  onSignOut,
  session,
  statusItems,
  title,
}: ProductShellProps) {
  const workspaceOptions = buildWorkspaceOptions(session);
  const accountLabel = session?.user.name || (isSignedIn ? "Resolving" : "Account");
  const navItems = buildNavItems(session);
  const showOperationsConsole = Boolean(session && accessSummary(session).platformAdmin);

  return (
    <main className={styles.page}>
      <header className={styles.topbar}>
        <Link className={styles.brand} href="/">
          <span className={styles.brandMark}>RL</span>
          <span className={styles.brandText}>
            <strong>RustLearn</strong>
            <small>Product workspace</small>
          </span>
        </Link>

        <nav className={styles.desktopNav} aria-label="Primary navigation">
          {navItems.map((item) => {
            const Icon = item.icon;
            return (
              <Link
                aria-current={activeNav === item.key ? "page" : undefined}
                className={`${styles.navItem} ${activeNav === item.key ? styles.activeNav : ""}`}
                href={item.href}
                key={item.key}
              >
                <Icon size={17} aria-hidden />
                {item.label}
              </Link>
            );
          })}
        </nav>

        <div className={styles.toolbar}>
          <label className={styles.workspaceControl}>
            <span>Workspace</span>
            <select aria-label="Workspace" disabled={workspaceOptions.length === 0}>
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
          <button
            aria-label="Notifications"
            className={styles.iconButton}
            disabled
            title="Notifications are not available yet"
            type="button"
          >
            <Bell size={18} aria-hidden />
          </button>
          <AccountMenu
            accountLabel={accountLabel}
            isSignedIn={isSignedIn}
            onSignOut={onSignOut}
            session={session}
            showOperationsConsole={showOperationsConsole}
          />
        </div>

        <MobileMenu
          activeNav={activeNav}
          accountLabel={accountLabel}
          isSignedIn={isSignedIn}
          navItems={navItems}
          onSignOut={onSignOut}
          session={session}
          showOperationsConsole={showOperationsConsole}
          workspaceOptions={workspaceOptions}
        />
      </header>

      <section className={styles.routeHeader}>
        <div className={styles.routeTitle}>
          <p className={styles.eyebrow}>{eyebrow}</p>
          <h1>{title}</h1>
          <p>{description}</p>
        </div>
        {statusItems ? <div className={styles.statusStrip}>{statusItems}</div> : null}
      </section>

      {breadcrumbs.length ? (
        <nav className={styles.breadcrumbs} aria-label="Breadcrumb">
          {breadcrumbs.map((breadcrumb, index) => (
            <span key={`${breadcrumb.label}-${index}`}>
              {index > 0 ? <span className={styles.breadcrumbDivider}>/</span> : null}
              {breadcrumb.href ? <Link href={breadcrumb.href}>{breadcrumb.label}</Link> : breadcrumb.label}
            </span>
          ))}
        </nav>
      ) : null}

      {notice ? <GlobalNotice notice={notice} /> : null}

      <div className={styles.content}>{children}</div>
    </main>
  );
}

function AccountMenu({
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

function MobileMenu({
  accountLabel,
  activeNav,
  isSignedIn,
  navItems,
  onSignOut,
  session,
  showOperationsConsole,
  workspaceOptions,
}: {
  accountLabel: string;
  activeNav: ActiveNav;
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
          <select aria-label="Mobile workspace" disabled={workspaceOptions.length === 0}>
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

function GlobalNotice({ notice }: { notice: ShellNotice }) {
  return (
    <section
      aria-live={notice.tone === "error" ? "assertive" : "polite"}
      className={`${styles.globalNotice} ${styles[notice.tone]}`}
      role="status"
    >
      <div>
        <strong>{notice.title}</strong>
        <span>{notice.message}</span>
      </div>
      {notice.actionHref && notice.actionLabel ? (
        <Link className={styles.noticeAction} href={notice.actionHref}>
          {notice.actionLabel}
        </Link>
      ) : null}
    </section>
  );
}

function DelegationMenu({ session }: { session?: CurrentSession | null }) {
  const delegations = session?.delegated_permissions || [];
  if (!delegations.length) {
    return null;
  }

  return (
    <div className={styles.delegationMenu} aria-label="Delegated access">
      <strong>Delegated access</strong>
      {delegations.slice(0, 3).map((delegation) => (
        <div className={styles.delegationItem} key={delegation.id}>
          <span>{delegation.permission}</span>
          <small>
            {delegationScopeLabel(delegation)} - {delegationExpiryLabel(delegation.expires_at)}
          </small>
        </div>
      ))}
      {delegations.length > 3 ? <small>+{delegations.length - 3} more delegations</small> : null}
    </div>
  );
}

function buildWorkspaceOptions(session?: CurrentSession | null) {
  if (!session) {
    return [];
  }

  const options = [{ label: "Personal workspace", value: `user:${session.user.id}` }];
  options.push(
    ...session.organizations.map((organization) => ({
      label: organization.name,
      value: `organization:${organization.id}`,
    })),
  );
  options.push(
    ...session.courses.map((course) => ({
      label: course.title,
      value: `course:${course.id}`,
    })),
  );
  return options;
}

function delegationScopeLabel(delegation: CurrentSession["delegated_permissions"][number]) {
  return delegation.organization_name || delegation.course_title || delegation.scope_type;
}

function delegationExpiryLabel(expiresAt: string | null) {
  if (!expiresAt) {
    return "No expiry";
  }

  return `Expires ${expiresAt.replace("T", " ").slice(0, 16)}`;
}

function buildNavItems(session?: CurrentSession | null) {
  if (!session) {
    return baseNavItems;
  }

  const access = accessSummary(session);
  return [
    baseNavItems[0],
    access.learner
      ? { key: "learn", href: "/learn", label: "Learn", icon: Home }
      : null,
    access.teacher
      ? { key: "teach", href: "/teach", label: "Teach", icon: BriefcaseBusiness }
      : null,
    access.organization
      ? { key: "organizations", href: "/organizations", label: "Organizations", icon: BriefcaseBusiness }
      : null,
    access.platformAdmin
      ? { key: "admin", href: "/admin", label: "Admin", icon: BriefcaseBusiness }
      : null,
    baseNavItems[1],
  ].filter((item): item is NavItem => Boolean(item));
}
