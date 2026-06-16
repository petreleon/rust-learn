"use client";

import { type FormEvent } from "react";
import { type OrganizationDetail } from "@/lib/organization/OrganizationDetail";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { DeleteOrganizationPanel } from "./DeleteOrganizationPanel";
import { OrganizationIdentityForm } from "./OrganizationIdentityForm";
import { SettingsHero } from "./SettingsHero";
import { SettingsLoadStatePanel } from "./SettingsLoadStatePanel";
import { SettingsStatusMessage } from "./SettingsStatusMessage";
import { type SettingsSaveState } from "../model/SettingsSaveState";

type Props = {
  deleteConfirm: boolean;
  deleteMessage: string | null;
  deleteState: SettingsSaveState;
  detail: OrganizationDetail | null;
  nameDraft: string;
  onDelete: () => void;
  onDeleteConfirmChange: (value: boolean) => void;
  onDeleteConfirmReset: () => void;
  onNameDraftChange: (value: string) => void;
  onProfileUrlDraftChange: (value: string) => void;
  onRefresh: () => void;
  onSave: (event: FormEvent) => void;
  onWebsiteDraftChange: (value: string) => void;
  organization: OrganizationWorkspaceItem;
  profileUrlDraft: string;
  saveMessage: string | null;
  saveState: SettingsSaveState;
  settingsError: RouteError | null;
  settingsLoadState: LoadState;
  websiteDraft: string;
};

export function OrganizationSettingsPanel(props: Props) {
  return (
    <>
      <SettingsHero organization={props.organization} />
      <SettingsStatusMessage errorTitle="Update error" message={props.saveMessage} state={props.saveState} successTitle="Saved" />
      <SettingsStatusMessage errorTitle="Deletion error" message={props.deleteMessage} state={props.deleteState} successTitle="Deleted" />
      <SettingsLoadStatePanel onRefresh={props.onRefresh} settingsError={props.settingsError} settingsLoadState={props.settingsLoadState} />
      {props.settingsLoadState === "success" && props.detail ? (
        <>
          <OrganizationIdentityForm
            nameDraft={props.nameDraft}
            onNameDraftChange={props.onNameDraftChange}
            onProfileUrlDraftChange={props.onProfileUrlDraftChange}
            onRefresh={props.onRefresh}
            onSave={props.onSave}
            onWebsiteDraftChange={props.onWebsiteDraftChange}
            profileUrlDraft={props.profileUrlDraft}
            saveState={props.saveState}
            websiteDraft={props.websiteDraft}
          />
          <DeleteOrganizationPanel
            deleteConfirm={props.deleteConfirm}
            deleteState={props.deleteState}
            onDelete={props.onDelete}
            onDeleteConfirmChange={props.onDeleteConfirmChange}
            onDeleteConfirmReset={props.onDeleteConfirmReset}
            organization={props.organization}
          />
        </>
      ) : null}
    </>
  );
}
