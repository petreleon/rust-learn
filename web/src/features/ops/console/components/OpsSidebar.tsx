"use client";

import { Ban, Eye, EyeOff, KeyRound } from "lucide-react";
import styles from "../ops-console.module.css";
import { optionLabel } from "../model/optionLabel";
import { statusLabel } from "../model/statusLabel";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function OpsSidebar({ controller }: { controller: OpsConsoleController }) {
  const { auth } = controller;

  return (
    <aside className={styles.sidebar} aria-label="Workspace controls">
      <div className={styles.brand}>
        <span className={styles.brandMark}>RL</span>
        <div>
          <p className={styles.brandName}>RustLearn</p>
          <p className={styles.brandMeta}>Reward operations</p>
        </div>
      </div>
      <section className={`${styles.sidebarSection} ${styles.sessionPanel}`} aria-labelledby="api-session-title">
        <div className={styles.sectionHeaderCompact}>
          <h2 id="api-session-title">Session</h2>
          <span className={`${styles.statusPill} ${styles[auth.apiState]}`}>{statusLabel(auth.apiState)}</span>
        </div>
        {process.env.NODE_ENV === "development" && (
          <label className={styles.fieldLabel}>
            API root
            <input value={auth.apiRoot} onChange={(event) => auth.updateApiRoot(event.target.value)} />
          </label>
        )}
        <form className={styles.sessionForm} onSubmit={auth.signIn}>
          <label className={styles.fieldLabel}>
            Email
            <input
              type="email"
              autoComplete="email"
              value={auth.credentials.email}
              onChange={(event) => auth.setCredentials((current) => ({ ...current, email: event.target.value }))}
            />
          </label>
          <label className={styles.fieldLabel}>
            Password
            <input
              type="password"
              autoComplete="current-password"
              value={auth.credentials.password}
              onChange={(event) => auth.setCredentials((current) => ({ ...current, password: event.target.value }))}
            />
          </label>
          <div className={styles.sessionActions}>
            <button type="submit" className={styles.primaryButton} disabled={!auth.canSignIn} title={auth.pendingActionTitle}>
              <KeyRound size={17} aria-hidden />
              <span>{auth.pendingAction === "Sign in" ? "Signing in" : "Sign in"}</span>
            </button>
            <button
              type="button"
              className={styles.secondaryButton}
              onClick={auth.clearSession}
              disabled={!auth.hasSessionDraft || auth.hasPendingAction}
              title={auth.pendingActionTitle ?? (auth.hasSessionDraft ? "Clear local session fields" : "No session fields to clear")}
              aria-label="Clear session fields"
            >
              <Ban size={17} aria-hidden />
              <span>Clear</span>
            </button>
          </div>
        </form>
        <div className={styles.fieldLabel}>
          <span>JWT</span>
          <span className={styles.secretField}>
            <input
              aria-label="JWT"
              type="text"
              className={auth.showToken ? undefined : styles.maskedSecret}
              autoComplete="off"
              value={auth.token}
              onChange={(event) => auth.updateToken(event.target.value)}
              spellCheck={false}
            />
            <button
              type="button"
              className={styles.tokenVisibilityButton}
              onClick={() => auth.setShowToken((current) => !current)}
              disabled={!auth.hasSessionToken}
              aria-label={auth.showToken ? "Hide JWT" : "Show JWT"}
              aria-pressed={auth.showToken}
              title={auth.showToken ? "Hide JWT" : "Show JWT"}
            >
              {auth.showToken ? <EyeOff size={17} aria-hidden /> : <Eye size={17} aria-hidden />}
            </button>
          </span>
        </div>
        <p className={styles.statusMessage} aria-live="polite">
          {auth.sessionMessage}
        </p>
        <p className={styles.statusMessage} aria-live="polite">
          {auth.apiMessage}
        </p>
        <div className={styles.permissionSummary}>
          <span>Session permissions</span>
          <strong>{controller.enabledPermissionCount}</strong>
          <a href="#permissions-panel">View</a>
        </div>
      </section>
      <section id="permissions-panel" className={`${styles.sidebarSection} ${styles.permissionsPanel}`} aria-labelledby="permissions-title">
        <div className={styles.sectionHeaderCompact}>
          <h2 id="permissions-title">Session permissions</h2>
          <span className={styles.countPill}>{controller.enabledPermissionCount}</span>
        </div>
        {controller.permissionGroups.map((group) => (
          <div key={group.scope} className={styles.permissionGroup}>
            <p>{group.label}</p>
            {group.permissions.length ? (
              group.permissions.map((permission) => (
                <div key={permission} className={styles.checkboxRow}>
                  <span>{optionLabel(permission)}</span>
                </div>
              ))
            ) : (
              <span className={styles.muted}>None enabled</span>
            )}
          </div>
        ))}
      </section>
    </aside>
  );
}
