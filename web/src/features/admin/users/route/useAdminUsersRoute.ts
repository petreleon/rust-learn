"use client";

import { type FormEvent, useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace, type AdminRole, type AdminUserProfile } from "@/lib/admin";
import { type CurrentSession } from "@/lib/session";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  assignRoleToAdminUser,
  loadAdminPlatformRoles,
  loadAdminUserProfile,
  loadAdminUsersSession,
  searchAdminUsers,
} from "../api/adminUsersApi";
import {
  canAssignAdminUserRoles,
  canViewAdminUsers,
  canViewPlatformRoleCatalog,
  emptyAdminUsersWorkspace,
  findUsersCapability,
} from "../model/adminUserAccessModel";
import { useAdminUserRoleAudit } from "./useAdminUserRoleAudit";
import { normalizeAdminUsersRouteError } from "./normalizeAdminUsersRouteError";

export function useAdminUsersRoute() {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [profile, setProfile] = useState<AdminUserProfile | null>(null);
  const [roles, setRoles] = useState<AdminRole[]>([]);
  const [roleName, setRoleName] = useState("");
  const [rolesError, setRolesError] = useState<RouteError | null>(null);
  const [rolesState, setRolesState] = useState<LoadState>("idle");
  const [searchError, setSearchError] = useState<RouteError | null>(null);
  const [searchInput, setSearchInput] = useState("");
  const [searchState, setSearchState] = useState<LoadState>("idle");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [users, setUsers] = useState<AdminUserProfile[]>([]);
  const [assignError, setAssignError] = useState<RouteError | null>(null);
  const [assignState, setAssignState] = useState<LoadState>("idle");

  const workspace = useMemo(() => (session ? buildPlatformAdminWorkspace(session) : emptyAdminUsersWorkspace), [session]);
  const allowed = session ? hasPlatformAdminAccess(session) : false;
  const canAssignRoles = canAssignAdminUserRoles(workspace);
  const canViewRoleCatalog = canViewPlatformRoleCatalog(workspace);
  const canViewUsers = canViewAdminUsers(workspace);
  const usersCapability = findUsersCapability(workspace);
  const audit = useAdminUserRoleAudit(canViewRoleCatalog);

  const clearRoute = useCallback((nextState: LoadState) => {
    setError(null);
    setHasToken(false);
    setLoadState(nextState);
    setSession(null);
  }, []);

  const loadRoles = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) return;
    setRolesError(null);
    setRolesState("loading");
    try {
      const nextRoles = await loadAdminPlatformRoles({ token });
      setRoles(nextRoles);
      setRoleName((current) => current || nextRoles[0]?.name || "");
      setRolesState("success");
    } catch (nextError) {
      setRolesError(normalizeAdminUsersRouteError(nextError, "Platform roles could not be loaded."));
      setRolesState("error");
    }
  }, []);

  const loadSession = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) return clearRoute("idle");
    setError(null);
    setHasToken(true);
    setLoadState("loading");
    try {
      setSession(await loadAdminUsersSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeAdminUsersRouteError(nextError, "Platform admin session could not be loaded.");
      if (routeError.status === 401 || routeError.status === 404) clearBrowserSession();
      setError(routeError);
      setHasToken(routeError.status !== 401 && routeError.status !== 404);
      setLoadState("error");
      setSession(null);
    }
  }, [clearRoute]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  useEffect(() => {
    if (session && allowed && canViewRoleCatalog) {
      const timeout = window.setTimeout(() => void loadRoles(), 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [allowed, canViewRoleCatalog, loadRoles, session]);

  async function handleSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readBrowserSessionToken();
    const trimmedSearch = searchInput.trim();
    if (!token || !canViewUsers || !trimmedSearch) return;
    setSearchError(null);
    setSearchState("loading");
    try {
      const result = await searchAdminUsers({ search: trimmedSearch, token });
      setUsers(result.users);
      setProfile(null);
      audit.resetAudit();
      setSearchState("success");
    } catch (nextError) {
      setSearchError(normalizeAdminUsersRouteError(nextError, "Users could not be loaded."));
      setSearchState("error");
    }
  }

  async function selectUser(userId: number) {
    const token = readBrowserSessionToken();
    if (!token) return;
    setSearchError(null);
    audit.resetAudit();
    try {
      const nextProfile = await loadAdminUserProfile({ token, userId });
      setProfile(nextProfile);
      void audit.loadAudit(nextProfile.id, token);
    } catch (nextError) {
      setSearchError(normalizeAdminUsersRouteError(nextError, "User profile could not be loaded."));
    }
  }

  async function assignRole() {
    const token = readBrowserSessionToken();
    if (!token || !profile || !roleName.trim()) return;
    setAssignError(null);
    setAssignState("loading");
    try {
      await assignRoleToAdminUser({ roleName: roleName.trim(), token, userId: profile.id });
      const nextProfile = await loadAdminUserProfile({ token, userId: profile.id });
      setProfile(nextProfile);
      void audit.loadAudit(nextProfile.id, token);
      setAssignState("success");
    } catch (nextError) {
      setAssignError(normalizeAdminUsersRouteError(nextError, "Role assignment failed."));
      setAssignState("error");
    }
  }

  function signOut() {
    clearBrowserSession();
    audit.resetAudit();
    clearRoute("idle");
  }

  function refreshAudit() {
    if (profile) void audit.loadAudit(profile.id);
  }

  return {
    allowed, assignError, assignRole, assignState, auditError: audit.error,
    auditEvents: audit.events, auditState: audit.state, canAssignRoles, canViewRoleCatalog,
    canViewUsers, error, hasToken, loadRoles, loadSession, loadState, profile, refreshAudit,
    roleName, roles, rolesError, rolesState, searchError, searchInput, searchState,
    selectUser, session, setRoleName, setSearchInput, signOut, users, usersCapability,
    workspace, handleSearch,
  };
}

export type AdminUsersRouteController = ReturnType<typeof useAdminUsersRoute>;
