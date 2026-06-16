"use client";

import { useCallback, useMemo, useState } from "react";
import { type PlatformTeacherApplicationItem } from "@/lib/admin/PlatformTeacherApplicationItem";
import { type PlatformTeacherApplicationsResponse } from "@/lib/admin/PlatformTeacherApplicationsResponse";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminTeacherApplications } from "../api/teacherApplicationsApi";
import {
  defaultTeacherApplicationFilters,
  type TeacherApplicationFilters,
} from "../model/TeacherApplicationFilters";
import { normalizeAdminTeacherApplicationRouteError } from "./normalizeAdminTeacherApplicationRouteError";

export function useTeacherApplicationList({
  allowed,
  canReview,
  session,
}: {
  allowed: boolean;
  canReview: boolean;
  session: CurrentSession | null;
}) {
  const [applicationError, setApplicationError] = useState<RouteError | null>(null);
  const [applicationState, setApplicationState] = useState<LoadState>("idle");
  const [applications, setApplications] = useState<PlatformTeacherApplicationsResponse | null>(null);
  const [filters, setFilters] = useState<TeacherApplicationFilters>(defaultTeacherApplicationFilters);
  const [selectedApplicationId, setSelectedApplicationId] = useState<number | null>(null);

  const selectedApplication = useMemo(
    () => applications?.applications.find((item) => item.id === selectedApplicationId) || applications?.applications[0] || null,
    [applications, selectedApplicationId],
  );

  const loadApplications = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!session || !token || !allowed || !canReview) return;

    setApplicationState("loading");
    setApplicationError(null);

    try {
      const response = await loadAdminTeacherApplications({
        offset: filters.offset,
        search: filters.appliedSearch,
        status: filters.status,
        token,
      });
      setApplications(response);
      setApplicationState("success");
      setSelectedApplicationId((current) => nextSelectedApplicationId(current, response.applications));
    } catch (nextError) {
      setApplications(null);
      setApplicationError(
        normalizeAdminTeacherApplicationRouteError(
          nextError,
          "Teacher application review queue could not be loaded.",
        ),
      );
      setApplicationState("error");
    }
  }, [allowed, canReview, filters.appliedSearch, filters.offset, filters.status, session]);

  const applyFilters = useCallback(() => {
    setFilters((current) => ({
      ...current,
      appliedSearch: current.searchInput.trim(),
      offset: 0,
    }));
  }, []);

  const resetFilters = useCallback(() => {
    setFilters(defaultTeacherApplicationFilters);
  }, []);

  const updateFilters = useCallback((patch: Partial<TeacherApplicationFilters>) => {
    setFilters((current) => ({ ...current, ...patch }));
  }, []);

  const setPageOffset = useCallback((offset: number) => {
    setFilters((current) => ({ ...current, offset: Math.max(0, offset) }));
  }, []);

  return {
    applicationError,
    applications,
    applicationState,
    applyFilters,
    filters,
    loadApplications,
    resetFilters,
    selectedApplication,
    selectedApplicationId,
    setPageOffset,
    setSelectedApplicationId,
    updateFilters,
  };
}

function nextSelectedApplicationId(current: number | null, applications: PlatformTeacherApplicationItem[]) {
  if (current && applications.some((application) => application.id === current)) return current;
  return applications[0]?.id || null;
}
