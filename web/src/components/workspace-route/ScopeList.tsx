"use client";

import { type PlatformSessionScope } from "@/lib/session";
import styles from "../workspace-route.module.css";
import { PermissionPreview } from "./PermissionPreview";
import { StatusPill } from "./StatusPill";

export function ScopeList({
  empty,
  scopes,
  title,
}: {
  empty: string;
  scopes: Array<PlatformSessionScope & { id?: number; name?: string; title?: string; lifecycle_status?: string }>;
  title: string;
}) {
  return (
    <section className={styles.scopeList}>
      <div className={styles.scopeHeader}>
        <h2>{title}</h2>
        <StatusPill label={`${scopes.length} visible`} tone="neutral" />
      </div>
      {scopes.length ? (
        scopes.map((scope, index) => (
          <article className={styles.scopeCard} key={scope.id || title + index}>
            <h3>{scope.name || scope.title || "Platform"}</h3>
            {scope.lifecycle_status ? <p className={styles.muted}>{scope.lifecycle_status}</p> : null}
            <PermissionPreview scope={scope} />
          </article>
        ))
      ) : (
        <section className={styles.statePanel}>
          <p className={styles.muted}>{empty}</p>
        </section>
      )}
    </section>
  );
}
