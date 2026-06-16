"use client";

import { Users } from "lucide-react";
import { ProductShell } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { AdminDeniedState } from "@/features/admin/shared/route-kit/AdminDeniedState";
import { GatedPanel } from "@/features/admin/shared/route-kit/GatedPanel";
import { LoadingState } from "@/features/admin/shared/route-kit/LoadingState";
import { SessionErrorState } from "@/features/admin/shared/route-kit/SessionErrorState";
import { SignedOutState } from "@/features/admin/shared/route-kit/SignedOutState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { RoleAssignmentPanel } from "../components/RoleAssignmentPanel";
import { UserDetailPanel } from "../components/UserDetailPanel";
import { UserListPanel } from "../components/UserListPanel";
import { UserSearchPanel } from "../components/UserSearchPanel";
import { type AdminUsersRouteController } from "../route/useAdminUsersRoute";

export function AdminUsersView({ route }: { route: AdminUsersRouteController }) {
  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[{ label: "Admin", href: "/admin" }, { label: "Users" }]}
      description="Search platform users, inspect account status, and assign platform roles."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : route.canViewUsers ? "User access" : "User gated"} />
          <StatusPill label={`${route.users.length} search results`} />
          <StatusPill label={route.canAssignRoles ? "Role assignment" : "Assignment gated"} tone={route.canAssignRoles ? "good" : "neutral"} />
        </>
      }
      title="User management"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !route.allowed ? <AdminDeniedState workspace={route.workspace} /> : null}
      {route.session && route.allowed && !route.canViewUsers ? (
        <GatedPanel capability={route.usersCapability} icon={<Users size={20} aria-hidden />} title="User management unavailable" />
      ) : null}
      {route.session && route.allowed && route.canViewUsers ? (
        <div className={styles.stack}>
          <UserSearchPanel
            error={route.searchError}
            onSearch={route.handleSearch}
            onSearchInputChange={route.setSearchInput}
            searchInput={route.searchInput}
            state={route.searchState}
          />
          <div className={styles.twoColumnWide}>
            <UserListPanel
              onSelect={route.selectUser}
              selectedUserId={route.profile?.id ?? null}
              state={route.searchState}
              users={route.users}
            />
            <div className={styles.stack}>
              <UserDetailPanel user={route.profile} />
              <RoleAssignmentPanel
                canAssign={route.canAssignRoles}
                canViewRoleCatalog={route.canViewRoleCatalog}
                error={route.assignError}
                onAssign={route.assignRole}
                onRoleNameChange={route.setRoleName}
                roleName={route.roleName}
                roles={route.roles}
                rolesError={route.rolesError}
                rolesState={route.rolesState}
                state={route.assignState}
                user={route.profile}
              />
            </div>
          </div>
        </div>
      ) : null}
    </ProductShell>
  );
}
