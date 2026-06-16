import { fetchKycStatus, submitKyc, type KycStatusResponse, type SubmitKycPayload } from "@/lib/kyc";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { fetchNotificationPreferences } from "@/lib/session/fetchNotificationPreferences";
import { saveNotificationPreferences } from "@/lib/session/saveNotificationPreferences";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type NotificationPreferences } from "@/lib/session/NotificationPreferences";
import { fetchWalletForAccount } from "./fetchWalletForAccount";
import { type WalletResult } from "../model/WalletResult";

export type AccountSettingsData = {
  prefs: NotificationPreferences | null;
  session: CurrentSession;
  walletResult: WalletResult;
};

export async function loadAccountSettingsData({ token }: { token: string }): Promise<AccountSettingsData> {
  const [session, walletResult] = await Promise.all([
    fetchCurrentSession({ token }),
    fetchWalletForAccount(token),
  ]);
  let prefs: NotificationPreferences | null = null;
  try {
    prefs = await fetchNotificationPreferences({ token });
  } catch {
    prefs = null;
  }
  return { prefs, session, walletResult };
}

export function saveAccountNotificationPreferences({
  emailEnabled,
  pushEnabled,
  token,
}: {
  emailEnabled: boolean;
  pushEnabled: boolean;
  token: string;
}) {
  return saveNotificationPreferences({ emailEnabled, pushEnabled, token });
}

export function loadAccountKycStatus({ token }: { token: string }): Promise<KycStatusResponse> {
  return fetchKycStatus({ token });
}

export function submitAccountKyc({ payload, token }: { payload: SubmitKycPayload; token: string }) {
  return submitKyc({ payload, token });
}
