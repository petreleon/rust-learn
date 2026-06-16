"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { type WalletSummary } from "@/lib/learner/WalletSummary";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type NotificationPreferences } from "@/lib/session/NotificationPreferences";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAccountSettingsData, saveAccountNotificationPreferences } from "../api/accountSettingsApi";
import { accountWorkspaceSummary } from "../model/accountWorkspaceSummary";
import { type LoadState } from "../model/LoadState";
import { type PrefsSaveState } from "../model/PrefsSaveState";
import { type RouteError } from "../model/RouteError";
import { type WalletLoadState } from "../model/WalletLoadState";
import { walletStatusLabel } from "../model/walletStatusLabel";
import { normalizeAccountSettingsError } from "./normalizeAccountSettingsError";

export function useAccountSettingsRoute() {
  const [activeToken, setActiveToken] = useState<string | null>(null);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [prefs, setPrefs] = useState<NotificationPreferences | null>(null);
  const [prefsEmail, setPrefsEmail] = useState(true);
  const [prefsMessage, setPrefsMessage] = useState<string | null>(null);
  const [prefsPush, setPrefsPush] = useState(false);
  const [prefsSaveState, setPrefsSaveState] = useState<PrefsSaveState>("idle");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [wallet, setWallet] = useState<WalletSummary | null>(null);
  const [walletError, setWalletError] = useState<RouteError | null>(null);
  const [walletState, setWalletState] = useState<WalletLoadState>("idle");

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setActiveToken(null);
    setError(null);
    setHasToken(false);
    setLoadState(nextLoadState);
    setSession(null);
    setWallet(null);
    setWalletError(null);
    setWalletState("idle");
  }, []);

  const loadAccount = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) {
      clearRoute("idle");
      return;
    }
    setActiveToken(token);
    setHasToken(true);
    setLoadState("loading");
    setWalletState("loading");
    setError(null);
    setWalletError(null);

    try {
      const next = await loadAccountSettingsData({ token });
      setSession(next.session);
      setWallet(next.walletResult.wallet);
      setWalletError(next.walletResult.error);
      setWalletState(next.walletResult.error ? "error" : "success");
      if (next.prefs) {
        setPrefs(next.prefs);
        setPrefsEmail(next.prefs.email_enabled);
        setPrefsPush(next.prefs.push_enabled);
      }
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeAccountSettingsError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearBrowserSession();
        setActiveToken(null);
        setHasToken(false);
      } else {
        setActiveToken(token);
        setHasToken(true);
      }
      setSession(null);
      setWallet(null);
      setWalletError(null);
      setWalletState("idle");
      setLoadState("error");
      setError(routeError);
    }
  }, [clearRoute]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadAccount(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadAccount]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  async function savePreferences() {
    if (!activeToken) return;
    setPrefsSaveState("saving");
    setPrefsMessage(null);
    try {
      const updated = await saveAccountNotificationPreferences({
        emailEnabled: prefsEmail,
        pushEnabled: prefsPush,
        token: activeToken,
      });
      setPrefs(updated);
      setPrefsSaveState("success");
      setPrefsMessage("Preferences saved.");
    } catch (nextError) {
      setPrefsMessage(nextError instanceof Error ? nextError.message : "Failed to save.");
      setPrefsSaveState("error");
    }
  }

  return {
    activeToken,
    error,
    hasToken,
    loadAccount,
    loadState,
    prefs,
    prefsEmail,
    prefsMessage,
    prefsPush,
    prefsSaveState,
    savePreferences,
    session,
    setPrefsEmail,
    setPrefsPush,
    signOut,
    wallet,
    walletError,
    walletState,
    walletStatus: walletStatusLabel(wallet, walletState, walletError),
    workspaceSummary: useMemo(() => accountWorkspaceSummary(session), [session]),
  };
}

export type AccountSettingsRouteController = ReturnType<typeof useAccountSettingsRoute>;
