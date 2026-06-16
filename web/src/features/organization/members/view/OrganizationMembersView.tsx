"use client";

import { ProductShell } from "@/components/product-shell";
import { ErrorState } from "@/components/organization-routes/ErrorState";
import { InviteMemberForm } from "@/components/organization-routes/InviteMemberForm";
import { LoadingState } from "@/components/organization-routes/LoadingState";
import { MembersDeniedState } from "@/components/organization-routes/MembersDeniedState";
import { MissingOrganizationState } from "@/components/organization-routes/MissingOrganizationState";
import { OrganizationMembersContent } from "@/components/organization-routes/OrganizationMembersContent";
import { OrganizationStatus } from "@/components/organization-routes/OrganizationStatus";
import { SignedOutState } from "@/components/organization-routes/SignedOutState";
import { organizationNotice } from "@/components/organization-routes/organizationNotice";
import { organizationMembersTitle } from "../model/organizationMembersRouteModel";
import { type OrganizationMembersRouteController } from "../route/useOrganizationMembersRoute";

export function OrganizationMembersView({
  organizationId,
  route,
}: {
  organizationId: string;
  route: OrganizationMembersRouteController;
}) {
  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        {
          href: route.organization ? `/organizations/${route.organization.id}` : undefined,
          label: route.organization?.name || "Organization",
        },
        { label: "Members" },
      ]}
      description="Organization members, role labels, direct and delegated scoped permissions, and operator action readiness."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={organizationNotice(route.error || route.memberError)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={route.workspace} />}
      title={organizationMembersTitle(route.organization)}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/members`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/members`} /> : null}
      {route.session && !route.organization ? <MissingOrganizationState /> : null}
      {route.session && route.organization && !route.canViewMembers ? (
        <MembersDeniedState capability={route.membersCapability} organizationName={route.organization.name} />
      ) : null}
      {route.session && route.organization && route.canViewMembers ? (
        <>
          <InviteMemberForm
            email={route.inviteEmail}
            inviteMessage={route.inviteMessage}
            inviteState={route.inviteState}
            onEmailChange={route.setInviteEmail}
            onRoleChange={route.setInviteRole}
            onSubmit={route.handleInviteMember}
            roleName={route.inviteRole}
          />
          <OrganizationMembersContent
            assignRoleMessage={route.assignRoleMessage}
            assignRoleState={route.assignRoleState}
            canAssignRoles={route.members?.operator_permissions.can_assign_roles ?? false}
            canManageMembers={route.members?.operator_permissions.can_manage_members ?? false}
            draftSearch={route.draftSearch}
            loadState={route.memberLoadState}
            members={route.members}
            onApplyFilters={route.applyFilters}
            onAssignRole={route.handleAssignRole}
            onDraftSearchChange={route.setDraftSearch}
            onPageChange={route.setPage}
            onPermissionFilterChange={(nextPermission) => {
              route.setPermissionFilter(nextPermission);
              route.setPage(0);
            }}
            onRefresh={route.loadMembers}
            onRemoveMember={route.handleRemoveMember}
            onResetFilters={route.resetFilters}
            onRoleFilterChange={(nextRole) => {
              route.setRoleFilter(nextRole);
              route.setPage(0);
            }}
            organization={route.organization}
            page={route.page}
            permissionFilter={route.permissionFilter}
            removeMemberMessage={route.removeMemberMessage}
            roleFilter={route.roleFilter}
            routeError={route.memberError}
          />
        </>
      ) : null}
    </ProductShell>
  );
}
