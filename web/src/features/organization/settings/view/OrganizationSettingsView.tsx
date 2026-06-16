"use client";

import { ProductShell } from "@/components/product-shell";
import { ErrorState } from "@/features/organization/shared/route-kit/ErrorState";
import { LoadingState } from "@/features/organization/shared/route-kit/LoadingState";
import { MissingOrganizationState } from "@/features/organization/shared/route-kit/MissingOrganizationState";
import { OrganizationStatus } from "@/features/organization/shared/route-kit/OrganizationStatus";
import { SettingsDeniedState } from "@/features/organization/shared/route-kit/SettingsDeniedState";
import { SignedOutState } from "@/features/organization/shared/route-kit/SignedOutState";
import { organizationNotice } from "@/features/organization/shared/route-kit/organizationNotice";
import { OrganizationSettingsPanel } from "../components/OrganizationSettingsPanel";
import { organizationSettingsTitle } from "../model/settingsRouteModel";
import { type OrganizationSettingsRouteController } from "../route/useOrganizationSettingsRoute";

export function OrganizationSettingsView({
  organizationId,
  route,
}: {
  organizationId: string;
  route: OrganizationSettingsRouteController;
}) {
  const saveError = route.saveState === "error" ? { code: "error", message: route.saveMessage ?? "", status: 0 } : null;
  const deleteError = route.deleteState === "error" ? { code: "error", message: route.deleteMessage ?? "", status: 0 } : null;

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/organizations", label: "Organizations" },
        { href: route.organization ? `/organizations/${route.organization.id}` : undefined, label: route.organization?.name || "Organization" },
        { label: "Settings" },
      ]}
      description="Organization name, profile links, and destructive actions scoped to operator permissions."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={organizationNotice(route.error || route.settingsError || saveError || deleteError)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={route.workspace} />}
      title={organizationSettingsTitle(route.organization)}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={`/organizations/${organizationId}/settings`} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect={`/organizations/${organizationId}/settings`} /> : null}
      {route.session && !route.organization ? <MissingOrganizationState /> : null}
      {route.session && route.organization && !route.canManageSettings ? (
        <SettingsDeniedState capability={route.settingsCapability} organizationName={route.organization.name} />
      ) : null}
      {route.session && route.organization && route.canManageSettings ? (
        <OrganizationSettingsPanel
          deleteConfirm={route.deleteConfirm}
          deleteMessage={route.deleteMessage}
          deleteState={route.deleteState}
          detail={route.detail}
          nameDraft={route.nameDraft}
          onDelete={route.handleDelete}
          onDeleteConfirmChange={route.setDeleteConfirm}
          onDeleteConfirmReset={route.resetDeleteConfirm}
          onNameDraftChange={route.setNameDraft}
          onProfileUrlDraftChange={route.setProfileUrlDraft}
          onRefresh={route.loadSettings}
          onSave={route.handleSave}
          onWebsiteDraftChange={route.setWebsiteDraft}
          organization={route.organization}
          profileUrlDraft={route.profileUrlDraft}
          saveMessage={route.saveMessage}
          saveState={route.saveState}
          settingsError={route.settingsError}
          settingsLoadState={route.settingsLoadState}
          websiteDraft={route.websiteDraft}
        />
      ) : null}
    </ProductShell>
  );
}
