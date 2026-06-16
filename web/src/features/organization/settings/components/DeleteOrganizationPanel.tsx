"use client";

import { AlertTriangle, Send } from "lucide-react";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { type SettingsSaveState } from "../model/SettingsSaveState";

export function DeleteOrganizationPanel({
  deleteConfirm,
  deleteState,
  onDelete,
  onDeleteConfirmChange,
  onDeleteConfirmReset,
  organization,
}: {
  deleteConfirm: boolean;
  deleteState: SettingsSaveState;
  onDelete: () => void;
  onDeleteConfirmChange: (value: boolean) => void;
  onDeleteConfirmReset: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  const isDeleting = deleteState === "saving";

  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Delete organization</h2>
      </div>
      <p className={styles.muted}>
        Permanently remove {organization.name} and its data. Inactive courses and existing wallet state may prevent deletion.
      </p>
      {deleteConfirm ? (
        <div className={styles.actionRow} style={{ marginTop: "0.75rem" }}>
          <button
            className={styles.primaryButton}
            disabled={isDeleting}
            onClick={onDelete}
            style={{ background: "var(--color-warn, #dc2626)", borderColor: "var(--color-warn, #dc2626)" }}
            type="button"
          >
            <Send size={17} aria-hidden />
            {isDeleting ? "Deleting..." : "Confirm delete"}
          </button>
          <button className={styles.secondaryButton} disabled={isDeleting} onClick={onDeleteConfirmReset} type="button">
            Cancel
          </button>
        </div>
      ) : (
        <button
          className={styles.secondaryButton}
          onClick={() => onDeleteConfirmChange(true)}
          style={{ color: "var(--color-warn, #dc2626)", marginTop: "0.75rem" }}
          type="button"
        >
          <AlertTriangle size={17} aria-hidden />
          Delete {organization.name}
        </button>
      )}
    </section>
  );
}
