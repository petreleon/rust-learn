"use client";

import { fetchMyWallet } from "@/lib/learner/fetchMyWallet";
import { type WalletResult } from "../model/WalletResult";
import { normalizeAccountSettingsError } from "../route/normalizeAccountSettingsError";

export async function fetchWalletForAccount(token: string): Promise<WalletResult> {
  try {
    return {
      error: null,
      wallet: await fetchMyWallet({ token }),
    };
  } catch (error) {
    return {
      error: normalizeAccountSettingsError(error),
      wallet: null,
    };
  }
}
