"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { buildOrganizationWorkspace, fetchOrganizationTeacherApplications, findOrganizationWorkspaceItem, type OrganizationTeacherApplicationList } from "@/lib/organization";
import { clearStoredSessionToken, readStoredSessionToken } from "@/lib/session";
import { ErrorState } from "@/components/organization-routes/ErrorState";
import { LoadingState } from "@/components/organization-routes/LoadingState";
import { MissingOrganizationState } from "@/components/organization-routes/MissingOrganizationState";
import { ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE } from "@/components/organization-routes/ORGANIZATION_TEACHER_APPLICATION_PAGE_SIZE";
import { OrganizationStatus } from "@/components/organization-routes/OrganizationStatus";
import { OrganizationTeacherApplicationsContent } from "@/components/organization-routes/OrganizationTeacherApplicationsContent";
import { SignedOutState } from "@/components/organization-routes/SignedOutState";
import { TeacherApplicationsDeniedState } from "@/components/organization-routes/TeacherApplicationsDeniedState";
import { emptyWorkspace } from "@/components/organization-routes/emptyWorkspace";
import { type RouteError } from "@/components/organization-routes/RouteError";
import { type TeacherApplicationLoadState } from "@/components/organization-routes/TeacherApplicationLoadState";
import { normalizeRouteError } from "@/components/organization-routes/normalizeRouteError";
import { organizationNotice } from "@/components/organization-routes/organizationNotice";
import { useOrganizationSession } from "@/components/organization-routes/useOrganizationSession";

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
