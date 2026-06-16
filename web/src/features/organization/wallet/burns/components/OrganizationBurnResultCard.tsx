"use client";

import { Flame } from "lucide-react";
import { StatusPill } from "@/features/organization/shared/route-kit/StatusPill";
import { formatUnderscoreLabel } from "@/features/organization/shared/route-kit/formatUnderscoreLabel";
import { type OrganizationTokenBurn } from "../model/organizationBurnTypes";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function OrganizationBurnResultCard({ burn }: { burn: OrganizationTokenBurn }) {
  return (
    <article className={styles.compactRow}>
      <div>
        <strong>{burn.amount} LearnToken</strong>
        <p>
          {formatUnderscoreLabel(burn.source)} · {formatUnderscoreLabel(burn.fee_path)}
        </p>
        <p>
          Actor {burn.actor_user_id} · {formatUnderscoreLabel(burn.wallet_action)}
        </p>
      </div>
      <StatusPill icon={<Flame size={16} aria-hidden />} label={formatUnderscoreLabel(burn.status)} tone="neutral" />
    </article>
  );
}
