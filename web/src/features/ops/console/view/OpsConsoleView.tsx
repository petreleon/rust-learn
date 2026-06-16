"use client";

import { ShieldAlert } from "lucide-react";
import Link from "next/link";
import styles from "../ops-console.module.css";
import { OpsSidebar } from "../components/OpsSidebar";
import { OpsWorkspace } from "../components/OpsWorkspace";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function OpsConsoleView({ controller }: { controller: OpsConsoleController }) {
  const { auth } = controller;

  return (
    <main className={styles.shell}>
      <OpsSidebar controller={controller} />
      <section className={styles.workspace}>
        {auth.sessionLoading ? (
          <div className={styles.deniedPanel} role="status">
            <h2>Resolving operations session...</h2>
          </div>
        ) : auth.hasSessionToken && auth.session && !controller.isPlatformAdmin ? (
          <section className={styles.deniedPanel} role="status">
            <div className={styles.panelHeader}>
              <ShieldAlert size={24} className={styles.deniedIcon} />
              <h2>Access Gated: Platform Admin required</h2>
            </div>
            <p>
              This operations console is reserved for internal platform operators. Your current session does not include
              the platform admin role or permissions.
            </p>
            <div className={styles.actionRow}>
              <Link className={styles.primaryLink} href="/session">
                Go to product workspace
              </Link>
              <button type="button" className={styles.secondaryButton} onClick={auth.clearSession}>
                Sign out or clear token
              </button>
            </div>
          </section>
        ) : (
          <OpsWorkspace controller={controller} />
        )}
      </section>
    </main>
  );
}
