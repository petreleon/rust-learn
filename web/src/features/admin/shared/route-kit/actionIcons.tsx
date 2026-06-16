"use client";

import { Database, FileSpreadsheet, Gauge, Landmark, ListChecks, ShieldAlert, ShieldCheck, UserCheck, Users, WalletCards, type LucideIcon } from "lucide-react";
import { type PlatformCapabilityKey } from "@/lib/admin";

export const actionIcons: Record<PlatformCapabilityKey, LucideIcon> = {
  delegations: ShieldCheck,
  exports: FileSpreadsheet,
  fraud_blocks: ShieldAlert,
  kyc_reviews: ShieldCheck,
  reward_amount_review: Landmark,
  reward_policies: ListChecks,
  summary: Gauge,
  system: Database,
  teacher_applications: UserCheck,
  users: Users,
  wallets: WalletCards,
};
