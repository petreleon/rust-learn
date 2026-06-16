"use client";

import { UserPlus } from "lucide-react";
import { useState } from "react";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { organizationMemberRoleOptions } from "./organizationMemberRoleOptions";

export function AssignRoleControl({
  memberId,
  onAssign,
}: {
  memberId: number;
  onAssign: (memberId: number, roleName: string) => void;
}) {
  const [selected, setSelected] = useState("");
  const roleOptions = organizationMemberRoleOptions.filter((r) => r.value !== "");

  return (
    <div className={styles.actionRow} style={{ gap: "0.5rem", marginTop: "0.5rem" }}>
      <select
        aria-label="Assign organization role"
        onChange={(event) => setSelected(event.target.value)}
        value={selected}
      >
        <option value="">Assign role…</option>
        {roleOptions.map((role) => (
          <option key={role.value} value={role.value}>
            {role.label}
          </option>
        ))}
      </select>
      <button
        className={styles.secondaryButton}
        disabled={!selected}
        onClick={() => {
          if (!selected) return;
          onAssign(memberId, selected);
        }}
        type="button"
      >
        <UserPlus size={17} aria-hidden />
        Assign
      </button>
    </div>
  );
}
