"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { buildOrganizationWorkspace, fetchOrganizationCourses, findOrganizationWorkspaceItem, type OrganizationCourseList } from "@/lib/organization";
import { clearStoredSessionToken, readStoredSessionToken } from "@/lib/session";
import { CoursesDeniedState } from "./CoursesDeniedState";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";
import { MissingOrganizationState } from "./MissingOrganizationState";
import { ORGANIZATION_COURSE_PAGE_SIZE } from "./ORGANIZATION_COURSE_PAGE_SIZE";
import { OrganizationCoursesContent } from "./OrganizationCoursesContent";
import { OrganizationStatus } from "./OrganizationStatus";
import { SignedOutState } from "./SignedOutState";
import { emptyWorkspace } from "./emptyWorkspace";
import { type CourseLoadState } from "./CourseLoadState";
import { type RouteError } from "./RouteError";
import { normalizeRouteError } from "./normalizeRouteError";
import { organizationNotice } from "./organizationNotice";
import { useOrganizationSession } from "./useOrganizationSession";

export function OrganizationCoursesRoute({ organizationId }: { organizationId: string }) {
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
  const coursesCapability = organization?.capabilities.find((capability) => capability.key === "courses");
  const canViewCourses = Boolean(coursesCapability?.enabled);
  const [courseError, setCourseError] = useState<RouteError | null>(null);
  const [courseLoadState, setCourseLoadState] = useState<CourseLoadState>("idle");
  const [courses, setCourses] = useState<OrganizationCourseList | null>(null);
  const [draftSearch, setDraftSearch] = useState("");
  const [lifecycleStatus, setLifecycleStatus] = useState("");
  const [page, setPage] = useState(0);
  const [rewardFilter, setRewardFilter] = useState<"all" | "rewarded" | "unrewarded">("all");
  const [search, setSearch] = useState("");
  const notice = organizationNotice(route.error || courseError);

  const loadCourses = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token || invalidOrganizationId || !organization || !canViewCourses) {
      return;
    }

    setCourseError(null);
    setCourseLoadState("loading");

    try {
      const nextCourses = await fetchOrganizationCourses({
        lifecycleStatus,
        limit: ORGANIZATION_COURSE_PAGE_SIZE,
        offset: page * ORGANIZATION_COURSE_PAGE_SIZE,
        organizationId: organization.id,
        rewardAvailable:
          rewardFilter === "all" ? null : rewardFilter === "rewarded",
        search,
        token,
      });
      setCourses(nextCourses);
      setCourseLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
      }
      setCourses(null);
      setCourseError(routeError);
      setCourseLoadState("error");
    }
  }, [canViewCourses, invalidOrganizationId, lifecycleStatus, organization, page, rewardFilter, search]);

  useEffect(() => {
    if (route.session && organization && canViewCourses) {
      const timeout = window.setTimeout(() => {
        setCourseError(null);
        setCourseLoadState("idle");
        setCourses(null);
        void loadCourses();
      }, 0);
      return () => window.clearTimeout(timeout);
    }

    return undefined;
  }, [canViewCourses, loadCourses, organization, route.session]);

  function applyFilters() {
    setPage(0);
    setSearch(draftSearch);
  }

  function resetFilters() {
    setDraftSearch("");
    setLifecycleStatus("");
    setPage(0);
    setRewardFilter("all");
    setSearch("");
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
        { label: "Courses" },
      ]}
      description="Organization-sponsored courses, lifecycle state, teacher coverage, enrollment pressure, content readiness, and reward policy status."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title={organization?.name ? `${organization.name} courses` : "Organization courses"}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/courses`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/courses`} /> : null}
      {route.session && (invalidOrganizationId || !organization) ? <MissingOrganizationState /> : null}
      {route.session && organization && !canViewCourses ? (
        <CoursesDeniedState capability={coursesCapability} organizationName={organization.name} />
      ) : null}
      {route.session && organization && canViewCourses ? (
        <OrganizationCoursesContent
          courses={courses}
          draftSearch={draftSearch}
          lifecycleStatus={lifecycleStatus}
          loadState={courseLoadState}
          onApplyFilters={applyFilters}
          onDraftSearchChange={setDraftSearch}
          onLifecycleStatusChange={(nextStatus) => {
            setLifecycleStatus(nextStatus);
            setPage(0);
          }}
          onPageChange={setPage}
          onRefresh={loadCourses}
          onResetFilters={resetFilters}
          onRewardFilterChange={(nextFilter) => {
            setRewardFilter(nextFilter);
            setPage(0);
          }}
          organization={organization}
          page={page}
          routeError={courseError}
          rewardFilter={rewardFilter}
        />
      ) : null}
    </ProductShell>
  );
}
