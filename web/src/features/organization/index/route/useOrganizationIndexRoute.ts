"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { buildOrganizationWorkspace, filterOrganizationWorkspace, type OrganizationDetail } from "@/lib/organization";
import { clearStoredSessionToken, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { emptyWorkspace } from "@/features/organization/shared/route-kit/emptyWorkspace";
import { type CapabilityFilter } from "@/features/organization/shared/route-kit/CapabilityFilter";
import { type LoadState } from "@/features/organization/shared/route-kit/LoadState";
import { normalizeRouteError } from "@/features/organization/shared/route-kit/normalizeRouteError";
import { type RouteError } from "@/features/organization/shared/route-kit/RouteError";
import { loadOrganizationIndexSession, loadPlatformOrganizations } from "../api/organizationIndexApi";
import { canBrowsePlatformOrganizations, filterPlatformOrganizations } from "../model/platformOrganizationDirectory";

export function useOrganizationIndexRoute() {
  const [capability, setCapability] = useState<CapabilityFilter>("all");
  const [directoryError, setDirectoryError] = useState<RouteError | null>(null);
  const [directorySearch, setDirectorySearch] = useState("");
  const [directoryState, setDirectoryState] = useState<LoadState>("idle");
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [platformOrganizations, setPlatformOrganizations] = useState<OrganizationDetail[]>([]);
  const [search, setSearch] = useState("");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const workspace = useMemo(() => (session ? buildOrganizationWorkspace(session) : emptyWorkspace), [session]);
  const visibleOrganizations = useMemo(
    () => filterOrganizationWorkspace(workspace.organizations, { capability, search }),
    [capability, search, workspace.organizations],
  );
  const canBrowseDirectory = canBrowsePlatformOrganizations(session);
  const visiblePlatformOrganizations = useMemo(
    () => filterPlatformOrganizations(platformOrganizations, directorySearch),
    [directorySearch, platformOrganizations],
  );

  const loadPlatformDirectory = useCallback(async (token = readStoredSessionToken()) => {
    if (!token) return;
    setDirectoryError(null);
    setDirectoryState("loading");

    try {
      setPlatformOrganizations(await loadPlatformOrganizations({ token }));
      setDirectoryState("success");
    } catch (nextError) {
      setPlatformOrganizations([]);
      setDirectoryError(normalizeRouteError(nextError));
      setDirectoryState("error");
    }
  }, []);

  const loadSession = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setError(null);
      setHasToken(false);
      setLoadState("idle");
      setPlatformOrganizations([]);
      setSession(null);
      return;
    }

    setError(null);
    setHasToken(true);
    setLoadState("loading");

    try {
      const nextSession = await loadOrganizationIndexSession({ token });
      setSession(nextSession);
      setLoadState("success");
      if (canBrowsePlatformOrganizations(nextSession)) {
        void loadPlatformDirectory(token);
      } else {
        setDirectoryState("idle");
        setPlatformOrganizations([]);
      }
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setError(routeError);
      setLoadState("error");
      setPlatformOrganizations([]);
      setSession(null);
    }
  }, [loadPlatformDirectory]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  function clearWorkspaceFilters() {
    setCapability("all");
    setSearch("");
  }

  function signOut() {
    clearStoredSessionToken();
    setDirectoryError(null);
    setDirectorySearch("");
    setDirectoryState("idle");
    setError(null);
    setHasToken(false);
    setLoadState("idle");
    setPlatformOrganizations([]);
    setSession(null);
  }

  return {
    canBrowseDirectory,
    capability,
    clearWorkspaceFilters,
    directoryError,
    directorySearch,
    directoryState,
    error,
    hasToken,
    loadPlatformDirectory,
    loadSession,
    loadState,
    platformOrganizations,
    search,
    session,
    setCapability,
    setDirectorySearch,
    setSearch,
    signOut,
    visibleOrganizations,
    visiblePlatformOrganizations,
    workspace,
  };
}
