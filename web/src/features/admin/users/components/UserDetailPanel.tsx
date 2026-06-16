"use client";

import { UserRound } from "lucide-react";
import { type AdminUserProfile } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { formatDate } from "@/features/admin/shared/route-kit/formatDate";
import { formatUnderscoreLabel } from "@/features/admin/shared/route-kit/formatUnderscoreLabel";
import { userVerificationTone } from "../model/userVerificationTone";

export function UserDetailPanel({ user }: { user: AdminUserProfile | null }) {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <UserRound size={20} aria-hidden />
        <div>
          <h2>User profile</h2>
          <p>Profile facts come from the platform user profile endpoint.</p>
        </div>
      </div>
      {!user ? <EmptyState text="Select a search result to inspect a user." /> : null}
      {user ? (
        <div className={styles.stack}>
          <div className={styles.statusList}>
            <div className={styles.statusRow}><span>Name</span><strong>{user.name}</strong></div>
            <div className={styles.statusRow}><span>Email</span><strong>{user.email}</strong></div>
            <div className={styles.statusRow}><span>User id</span><strong>{user.id}</strong></div>
            <div className={styles.statusRow}><span>Created</span><strong>{formatDate(user.created_at)}</strong></div>
            <div className={styles.statusRow}>
              <span>Email status</span>
              <StatusPill label={user.email_verified ? "Verified" : "Pending"} tone={userVerificationTone(user.email_verified)} />
            </div>
            <div className={styles.statusRow}>
              <span>KYC status</span>
              <StatusPill label={user.kyc_verified ? "Verified" : "Pending"} tone={userVerificationTone(user.kyc_verified)} />
            </div>
          </div>
          <div>
            <div className={styles.subsectionHeader}>
              <h3>Platform roles</h3>
            </div>
            <div className={styles.rowMeta}>
              {user.platform_roles.length ? (
                user.platform_roles.map((role) => <StatusPill key={role} label={formatUnderscoreLabel(role)} />)
              ) : (
                <span>No platform roles</span>
              )}
            </div>
          </div>
          <div>
            <div className={styles.subsectionHeader}>
              <h3>Platform permissions</h3>
            </div>
            <div className={styles.rowMeta}>
              {user.platform_permissions.length ? (
                user.platform_permissions
                  .slice(0, 8)
                  .map((permission) => <StatusPill key={permission} label={formatUnderscoreLabel(permission)} />)
              ) : (
                <span>No platform permissions</span>
              )}
            </div>
          </div>
        </div>
      ) : null}
    </section>
  );
}
