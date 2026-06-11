"use client";

import { Bell } from "lucide-react";
import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { accessSummary } from "@/lib/access";
import styles from "../product-shell.module.css";
import { AccountMenu } from "./AccountMenu";
import { GlobalNotice } from "./GlobalNotice";
import { MobileMenu } from "./MobileMenu";
import { buildNavItems } from "./buildNavItems";
import { buildWorkspaceOptions } from "./buildWorkspaceOptions";
import { type Breadcrumb } from "./Breadcrumb";
import { type ProductShellProps } from "./ProductShellProps";

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
  const pathname = usePathname();
  const router = useRouter();

  const workspaceOptions = buildWorkspaceOptions(session);
  const accountLabel = session?.user.name || (isSignedIn ? "Resolving" : "Account");
  const navItems = buildNavItems(session);
  const showOperationsConsole = Boolean(session && accessSummary(session).platformAdmin);

  const currentWorkspaceValue = (() => {
    if (!session) return "";
    const orgMatch = pathname.match(/^\/organizations\/(\d+)/);
    if (orgMatch) {
      return `organization:${orgMatch[1]}`;
    }
    const courseMatch = pathname.match(/^\/courses\/(\d+)/);
    if (courseMatch) {
      return `course:${courseMatch[1]}`;
    }
    return `user:${session.user.id}`;
  })();

  const handleWorkspaceChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const value = event.target.value;
    if (!value) return;

    const [type, id] = value.split(":");
    if (type === "organization") {
      router.push(`/organizations/${id}`);
    } else if (type === "course") {
      router.push(`/courses/${id}`);
    } else {
      router.push("/session");
    }
  };

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
            <select
              aria-label="Workspace"
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
          currentWorkspaceValue={currentWorkspaceValue}
          handleWorkspaceChange={handleWorkspaceChange}
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
