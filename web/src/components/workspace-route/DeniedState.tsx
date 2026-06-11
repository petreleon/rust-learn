"use client";

import { AlertTriangle } from "lucide-react";
import Link from "next/link";
import styles from "../workspace-route.module.css";
import { type WorkspaceConfig } from "./WorkspaceConfig";

export function DeniedState({ config }: { config: WorkspaceConfig }) {
  return (
    <section className={styles.deniedPanel} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Access not available</h2>
      </div>
      <p>
        This workspace is hidden from primary navigation until the current session includes one
        of these resolved access signals.
      </p>
      <ul className={styles.deniedList}>
        {config.deniedSignals.map((signal) => (
          <li key={signal}>{signal}</li>
        ))}
      </ul>
      <Link className={styles.secondaryLink} href="/session">
        Review current session
      </Link>
    </section>
  );
}
