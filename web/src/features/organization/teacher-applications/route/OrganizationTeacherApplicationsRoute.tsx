"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { buildOrganizationWorkspace, fetchOrganizationTeacherApplications, findOrganizationWorkspaceItem, type OrganizationTeacherApplicationList } from "@/lib/organization";
import { clearStoredSessionToken, readStoredSessionToken } from "@/lib/session";
import { ErrorState } from "@/features/organization/shared/route-kit/ErrorState";
import { LoadingState } from "@/features/organization/shared/route-kit/LoadingState";
import { MissingOrganizationState } from "@/features/organization/shared/route-kit/MissingOrganizationState";
import { ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE } from "@/features/organization/shared/route-kit/ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE";
import { OrganizationStatus } from "@/features/organization/shared/route-kit/OrganizationStatus";
import { OrganizationTeacherApplicationsContent } from "@/features/organization/shared/route-kit/OrganizationTeacherApplicationsContent";
import { SignedOutState } from "@/features/organization/shared/route-kit/SignedOutState";
import { TeacherApplicationsDeniedState } from "@/features/organization/shared/route-kit/TeacherApplicationsDeniedState";
import { emptyWorkspace } from "@/features/organization/shared/route-kit/emptyWorkspace";
import { type RouteError } from "@/features/organization/shared/route-kit/RouteError";
import { type TeacherApplicationLoadState } from "@/features/organization/shared/route-kit/TeacherApplicationLoadState";
import { normalizeRouteError } from "@/features/organization/shared/route-kit/normalizeRouteError";
import { organizationNotice } from "@/features/organization/shared/route-kit/organizationNotice";
import { useOrganizationSession } from "@/features/organization/shared/route-kit/useOrganizationSession";

export function OrganizationTeacherApplicationsRoute({ organizationId }: { organizationId: string }) {
  const route = useOrganizationSession();
  const numericOrganizationId = Number.parseInt(organizationId, 10);
  const invalidOrganizationId = !/^\d+$/.test(organizationId) || !Number.isFinite(numericOrganizationId);
  const organization = useMemo(
    () =>
      route.session && !invalidOrganizationId
        ? findOrganizationWorkspaceItem(route.session, numericOrganizationId)
        : null,
    [invalidOrganizationId, numericOrganizationId, route.session],
  );
  const workspace = useMemo(
    () => (route.session ? buildOrganizationWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const teacherApplicationCapability = organization?.capabilities.find(
    (capability) => capability.key === "teacher_applications",
  );
  const canTrackTeacherApplications = Boolean(teacherApplicationCapability?.enabled);
  const [applicationError, setApplicationError] = useState<RouteError | null>(null);
  const [applicationLoadState, setApplicationLoadState] = useState<TeacherApplicationLoadState>("idle");
  const [applications, setApplications] = useState<OrganizationTeacherApplicationList | null>(null);
  const [draftSearch, setDraftSearch] = useState("");
  const [page, setPage] = useState(0);
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const notice = organizationNotice(route.error || applicationError);

  const loadApplications = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canTrackTeacherApplications) {
      return;
    }

    setApplicationError(null);
    setApplicationLoadState("loading");

    try {
      const nextApplications = await fetchOrganizationTeacherApplications({
        limit: ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE,
        offset: page * ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE,
        organizationId: organization.id,
        search,
        status: statusFilter,
        token,
      });
      setApplications(nextApplications);
      setApplicationLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
      }
      setApplications(null);
      setApplicationError(routeError);
      setApplicationLoadState("error");
    }
  }, [canTrackTeacherApplications, invalidOrganizationId, organization, page, search, statusFilter]);

  useEffect(() => {
    if (route.session && organization && canTrackTeacherApplications) {
      const timeout = window.setTimeout(() => {
        setApplicationError(null);
        setApplicationLoadState("idle");
        setApplications(null);
        void loadApplications();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canTrackTeacherApplications, loadApplications, organization, route.session]);

  useEffect(() => {
    function handleVisible() {
      if (document.visibilityState === "visible" && route.session && organization && canTrackTeacherApplications) {
        void loadApplications();
      }
    }
    document.addEventListener("visibilitychange", handleVisible);
    return () => document.removeEventListener("visibilitychange", handleVisible);
  }, [canTrackTeacherApplications, loadApplications, organization, route.session]);

  function applyFilters() {
    setPage(0);
    setSearch(draftSearch);
  }

  function resetFilters() {
    setDraftSearch("");
    setPage(0);
    setSearch("");
    setStatusFilter("");
  }

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: organization ? `/organizations/${organization.id}` : undefined,
          label: organization?.name || "Organization",
        },
        { label: "Teacher nominations" },
      ]}
      description="Organization-sponsored teacher application tracking with applicant context, status filters, and audit hints."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} teacher nominations` : "Teacher nominations"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/teacher-applications`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/teacher-applications`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canTrackTeacherApplications ? (
        <TeacherApplicationsDeniedState
          capability={teacherApplicationCapability}
          organizationName={organization.name}
        />
      ) : null}
      {route.session && organization && canTrackTeacherApplications ? (
        <OrganizationTeacherApplicationsContent
          applications={applications}
          draftSearch={draftSearch}
          loadState={applicationLoadState}
          onApplyFilters={applyFilters}
          onDraftSearchChange={setDraftSearch}
          onPageChange={setPage}
          onRefresh={loadApplications}
          onResetFilters={resetFilters}
          onStatusFilterChange={(nextStatus) => {
            setStatusFilter(nextStatus);
            setPage(0);
          }}
          organization={organization}
          page={page}
          routeError={applicationError}
          statusFilter={statusFilter}
        />
      ) : null}
    </ProductShell>
  );
}
