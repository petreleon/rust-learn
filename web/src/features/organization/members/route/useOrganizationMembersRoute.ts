"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { buildOrganizationWorkspace } from "@/lib/organization/buildOrganizationWorkspace";
import { findOrganizationWorkspaceItem } from "@/lib/organization/findOrganizationWorkspaceItem";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationMemberList } from "@/lib/organization/OrganizationMemberList";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { loadCurrentOrganizationSession, loadOrganizationMembers } from "../api/memberApi";
import { emptyOrganizationWorkspace } from "../model/emptyOrganizationWorkspace";
import { ORGANIZATION_MEMBER_PAGE_SIZE } from "../model/organizationMemberConstants";
import {
  findMembersCapability,
  membersCapabilityEnabled,
  parseOrganizationRouteId,
} from "../model/organizationMembersRouteModel";
import { normalizeOrganizationMembersRouteError } from "./normalizeOrganizationMembersRouteError";
import { useOrganizationMemberActions } from "./useOrganizationMemberActions";

export function useOrganizationMembersRoute(organizationId: string) {
  const routeId = useMemo(() => parseOrganizationRouteId(organizationId), [organizationId]);
  const [draftSearch, setDraftSearch] = useState("");
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [memberError, setMemberError] = useState<RouteError | null>(null);
  const [memberLoadState, setMemberLoadState] = useState<LoadState>("idle");
  const [members, setMembers] = useState<OrganizationMemberList | null>(null);
  const [page, setPage] = useState(0);
  const [permissionFilter, setPermissionFilter] = useState("");
  const [roleFilter, setRoleFilter] = useState("");
  const [search, setSearch] = useState("");
  const [session, setSession] = useState<CurrentSession | null>(null);

  const organization = useMemo(
    () => (session && !routeId.invalidOrganizationId ? findOrganizationWorkspaceItem(session, routeId.numericOrganizationId) : null),
    [routeId.invalidOrganizationId, routeId.numericOrganizationId, session],
  );
  const workspace = useMemo(() => (session ? buildOrganizationWorkspace(session) : emptyOrganizationWorkspace), [session]);
  const membersCapability = useMemo(() => findMembersCapability(organization), [organization]);
  const canViewMembers = membersCapabilityEnabled(membersCapability);

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setError(null);
    setHasToken(false);
    setLoadState(nextLoadState);
    setSession(null);
  }, []);

  const loadSession = useCallback(async ({ silent = false }: { silent?: boolean } = {}) => {
    const token = readBrowserSessionToken();
    if (!token) {
      clearRoute("idle");
      return;
    }

    setError(null);
    setHasToken(true);
    if (!silent) setLoadState("loading");

    try {
      setSession(await loadCurrentOrganizationSession({ token }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeOrganizationMembersRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearBrowserSession();
        setHasToken(false);
      }
      setError(routeError);
      setLoadState("error");
      setSession(null);
    }
  }, [clearRoute]);

  const loadMembers = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || routeId.invalidOrganizationId || !organization || !canViewMembers) {
      return;
    }

    setMemberError(null);
    setMemberLoadState("loading");

    try {
      setMembers(await loadOrganizationMembers({
        limit: ORGANIZATION_MEMBER_PAGE_SIZE,
        offset: page * ORGANIZATION_MEMBER_PAGE_SIZE,
        organizationId: organization.id,
        permission: permissionFilter,
        role: roleFilter,
        search,
        token,
      }));
      setMemberLoadState("success");
    } catch (nextError) {
      const routeError = normalizeOrganizationMembersRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearBrowserSession();
      }
      setMembers(null);
      setMemberError(routeError);
      setMemberLoadState("error");
    }
  }, [canViewMembers, organization, page, permissionFilter, roleFilter, routeId.invalidOrganizationId, search]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadSession(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadSession]);

  useEffect(() => {
    function handleVisible() {
      if (document.visibilityState === "visible") {
        void loadSession({ silent: true });
      }
    }

    document.addEventListener("visibilitychange", handleVisible);
    return () => document.removeEventListener("visibilitychange", handleVisible);
  }, [loadSession]);

  useEffect(() => {
    if (session && organization && canViewMembers) {
      const timeout = window.setTimeout(() => {
        setMemberError(null);
        setMemberLoadState("idle");
        setMembers(null);
        void loadMembers();
      }, 0);
      return () => window.clearTimeout(timeout);
    }
    return undefined;
  }, [canViewMembers, loadMembers, organization, session]);

  useEffect(() => {
    function handleVisible() {
      if (document.visibilityState === "visible" && session && organization && canViewMembers) {
        void loadMembers();
      }
    }

    document.addEventListener("visibilitychange", handleVisible);
    return () => document.removeEventListener("visibilitychange", handleVisible);
  }, [canViewMembers, loadMembers, organization, session]);

  function signOut() {
    clearBrowserSession();
    clearRoute("idle");
  }

  function applyFilters() {
    setPage(0);
    setSearch(draftSearch);
  }

  function resetFilters() {
    setDraftSearch("");
    setPage(0);
    setPermissionFilter("");
    setRoleFilter("");
    setSearch("");
  }

  const memberActions = useOrganizationMemberActions({
    loadMembers,
    organizationId: organization?.id ?? null,
  });

  return {
    applyFilters, canViewMembers, draftSearch, error, hasToken, loadMembers, loadSession, loadState,
    memberError, memberLoadState, members, membersCapability, organization, page, permissionFilter,
    resetFilters, roleFilter, session, setDraftSearch, setPage, setPermissionFilter, setRoleFilter,
    signOut, workspace,
    ...memberActions,
  };
}
export type OrganizationMembersRouteController = ReturnType<typeof useOrganizationMembersRoute>;
