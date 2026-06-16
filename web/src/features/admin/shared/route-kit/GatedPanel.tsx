"use client";

import { type ReactNode } from "react";
import { missingPlatformPermissions, type PlatformCapability } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";

export function GatedPanel({
  capability,
  icon,
  title,
}: {
  capability: PlatformCapability;
  icon: ReactNode;
  title: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.gatedPanel}`} role="status">
      <div className={styles.panelHeader}>
        {icon}
        <div>
          <h2>{title}</h2>
          <p>Missing platform permission: {missingPlatformPermissions(capability).join(" or ")}</p>
        </div>
      </div>
    </section>
  );
}
