"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { buildOrganizationWorkspace } from "@/lib/organization/buildOrganizationWorkspace";
import { findOrganizationWorkspaceItem } from "@/lib/organization/findOrganizationWorkspaceItem";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationDetail } from "@/lib/organization/OrganizationDetail";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { loadCurrentOrganizationSession, loadOrganizationSettings } from "../api/settingsApi";
import { emptyOrganizationWorkspace } from "../model/emptyOrganizationWorkspace";
import {
  canManageOrganizationSettings,
  findSettingsCapability,
  parseOrganizationRouteId,
} from "../model/settingsRouteModel";
import { normalizeOrganizationSettingsRouteError } from "./normalizeOrganizationSettingsRouteError";
import { useOrganizationSettingsActions } from "./useOrganizationSettingsActions";

export function useOrganizationSettingsRoute(organizationId: string) {
  const routeId = useMemo(() => parseOrganizationRouteId(organizationId), [organizationId]);
  const [detail, setDetail] = useState<OrganizationDetail | null>(null);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [settingsError, setSettingsError] = useState<RouteError | null>(null);
  const [settingsLoadState, setSettingsLoadState] = useState<LoadState>("idle");

  const organization = useMemo(
    () => (session && !routeId.invalidOrganizationId ? findOrganizationWorkspaceItem(session, routeId.numericOrganizationId) : null),
    [routeId.invalidOrganizationId, routeId.numericOrganizationId, session],
  );
  const workspace = useMemo(() => (session ? buildOrganizationWorkspace(session) : emptyOrganizationWorkspace), [session]);
  const canManageSettings = canManageOrganizationSettings(organization);
  const settingsCapability = findSettingsCapability(organization);

  const actions = useOrganizationSettingsActions({
    canManageSettings,
    onDetailChange: setDetail,
    organizationId: organization?.id ?? null,
  });
  const { setDraftsFromDetail } = actions;

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setError(null);
    setHasToken(false);
    setLoadState(nextLoadState);
    setSession(null);
  }, []);

  const loadSession = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) {
      clearRoute("idle");
      return;
    }

    setError(null);
    setHasToken(true);
    setLoadState("loading");

    try {
      setSession(await loadCurrentOrganizationSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeOrganizationSettingsRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearBrowserSession();
        setHasToken(false);
      }
      setError(routeError);
      setLoadState("error");
      setSession(null);
    }
  }, [clearRoute]);

  const loadSettings = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || routeId.invalidOrganizationId || !organization || !canManageSettings) return;
    setSettingsError(null);
    setSettingsLoadState("loading");

    try {
      const nextDetail = await loadOrganizationSettings({ organizationId: organization.id, token });
      setDetail(nextDetail);
      setDraftsFromDetail(nextDetail);
      setSettingsLoadState("success");
    } catch (nextError) {
      const routeError = normalizeOrganizationSettingsRouteError(nextError);
      if (routeError.status === 401) clearBrowserSession();
      setDetail(null);
      setSettingsError(routeError);
      setSettingsLoadState("error");
    }
  }, [canManageSettings, organization, routeId.invalidOrganizationId, setDraftsFromDetail]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  useEffect(() => {
    if (session && organization && canManageSettings) {
      const timeout = window.setTimeout(() => {
        setDetail(null);
        setSettingsError(null);
        setSettingsLoadState("idle");
        void loadSettings();
      }, 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [canManageSettings, loadSettings, organization, session]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  return {
    canManageSettings, detail, error, hasToken, loadSession, loadSettings, loadState,
    organization, session, settingsCapability, settingsError, settingsLoadState,
    signOut, workspace, ...actions,
  };
}

export type OrganizationSettingsRouteController = ReturnType<typeof useOrganizationSettingsRoute>;
