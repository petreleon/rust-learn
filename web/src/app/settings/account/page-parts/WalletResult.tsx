"use client";

import { type WalletSummary } from "@/lib/learner";
import { type RouteError } from "./RouteError";

export type WalletResult = {
  error: RouteError | null;
  wallet: WalletSummary | null;
};
