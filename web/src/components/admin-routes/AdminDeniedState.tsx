"use client";

import { ShieldAlert } from "lucide-react";
import Link from "next/link";
import { type PlatformAdminWorkspace } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";

export function AdminDeniedState({ workspace }: { workspace: PlatformAdminWorkspace }) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <ShieldAlert size={20} aria-hidden />
        <div>
          <h2>Platform admin access is not available</h2>
          <p>This session has {workspace.effectivePermissionCount} resolved platform permissions, but none unlock the platform admin workspace.</p>
        </div>
      </div>
      <Link className={styles.secondaryLink} href="/session">
        Review current session
      </Link>
    </section>
  );
}
