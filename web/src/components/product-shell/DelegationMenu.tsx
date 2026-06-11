"use client";

import { type CurrentSession } from "@/lib/session";
import styles from "../product-shell.module.css";
import { delegationExpiryLabel } from "./delegationExpiryLabel";
import { delegationScopeLabel } from "./delegationScopeLabel";

export function DelegationMenu({ session }: { session?: CurrentSession | null }) {
  const delegations = session?.delegated_permissions || [];
  if (!delegations.length) {
    return null;
  }

  return (
    <div className={styles.delegationMenu} aria-label="Delegated access">
      <strong>Delegated access</strong>
      {delegations.slice(0, 3).map((delegation) => (
        <div className={styles.delegationItem} key={delegation.id}>
          <span>{delegation.permission}</span>
          <small>
            {delegationScopeLabel(delegation)} - {delegationExpiryLabel(delegation.expires_at)}
          </small>
        </div>
      ))}
      {delegations.length > 3 ? <small>+{delegations.length - 3} more delegations</small> : null}
    </div>
  );
}
