"use client";

import { type FormEvent, useCallback, useState } from "react";
import { type OrganizationDetail } from "@/lib/organization/OrganizationDetail";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { type SettingsSaveState } from "../model/SettingsSaveState";
import { deleteOrganizationSettings, saveOrganizationSettings } from "../api/settingsApi";
import { normalizeOrganizationSettingsRouteError } from "./normalizeOrganizationSettingsRouteError";

export function useOrganizationSettingsActions({
  canManageSettings,
  onDetailChange,
  organizationId,
}: {
  canManageSettings: boolean;
  onDetailChange: (detail: OrganizationDetail | null) => void;
  organizationId: number | null;
}) {
  const [deleteConfirm, setDeleteConfirm] = useState(false);
  const [deleteMessage, setDeleteMessage] = useState<string | null>(null);
  const [deleteState, setDeleteState] = useState<SettingsSaveState>("idle");
  const [nameDraft, setNameDraft] = useState("");
  const [profileUrlDraft, setProfileUrlDraft] = useState("");
  const [saveMessage, setSaveMessage] = useState<string | null>(null);
  const [saveState, setSaveState] = useState<SettingsSaveState>("idle");
  const [websiteDraft, setWebsiteDraft] = useState("");

  const setDraftsFromDetail = useCallback((detail: OrganizationDetail) => {
    setNameDraft(detail.name);
    setWebsiteDraft(detail.website_link ?? "");
    setProfileUrlDraft(detail.profile_url ?? "");
  }, []);

  async function handleSave(event: FormEvent) {
    event.preventDefault();
    const token = readBrowserSessionToken();
    if (!token || !organizationId || !canManageSettings) return;
    setSaveState("saving");
    setSaveMessage(null);

    try {
      const updated = await saveOrganizationSettings({
        organizationId,
        payload: {
          name: nameDraft.trim() || undefined,
          profile_url: profileUrlDraft.trim() || null,
          website_link: websiteDraft.trim() || null,
        },
        token,
      });
      onDetailChange(updated);
      setDraftsFromDetail(updated);
      setSaveState("success");
      setSaveMessage("Settings saved.");
    } catch (nextError) {
      setSaveMessage(normalizeOrganizationSettingsRouteError(nextError).message);
      setSaveState("error");
    }
  }

  async function handleDelete() {
    const token = readBrowserSessionToken();
    if (!token || !organizationId || !canManageSettings) return;
    setDeleteState("saving");
    setDeleteMessage(null);

    try {
      await deleteOrganizationSettings({ organizationId, token });
      setDeleteState("success");
      setDeleteMessage("Organization deleted. Return to the organization list.");
      onDetailChange(null);
    } catch (nextError) {
      setDeleteMessage(normalizeOrganizationSettingsRouteError(nextError).message);
      setDeleteState("error");
    }
  }

  function resetDeleteConfirm() {
    setDeleteConfirm(false);
    setDeleteState("idle");
    setDeleteMessage(null);
  }

  return {
    deleteConfirm, deleteMessage, deleteState, handleDelete, handleSave, nameDraft,
    profileUrlDraft, resetDeleteConfirm, saveMessage, saveState, setDeleteConfirm,
    setDraftsFromDetail, setNameDraft, setProfileUrlDraft, setWebsiteDraft, websiteDraft,
  };
}
