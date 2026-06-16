"use client";

import { type FormEvent, useState } from "react";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { type MemberActionState } from "../model/MemberActionState";
import { addMemberByEmail, assignMemberRole, removeMemberFromOrganization } from "../api/memberApi";
import { normalizeOrganizationMembersRouteError } from "./normalizeOrganizationMembersRouteError";

export function useOrganizationMemberActions({
  loadMembers,
  organizationId,
}: {
  loadMembers: () => void;
  organizationId: number | null;
}) {
  const [assignRoleMessage, setAssignRoleMessage] = useState<string | null>(null);
  const [assignRoleState, setAssignRoleState] = useState<MemberActionState>("idle");
  const [inviteEmail, setInviteEmail] = useState("");
  const [inviteMessage, setInviteMessage] = useState<string | null>(null);
  const [inviteRole, setInviteRole] = useState("");
  const [inviteState, setInviteState] = useState<MemberActionState>("idle");
  const [removeMemberMessage, setRemoveMemberMessage] = useState<string | null>(null);

  async function handleAssignRole(memberId: number, roleName: string) {
    const token = readBrowserSessionToken();
    if (!token || !organizationId) return;
    setAssignRoleState("saving");
    setAssignRoleMessage(null);
    try {
      await assignMemberRole({ memberId, organizationId, roleName, token });
      setAssignRoleState("success");
      setAssignRoleMessage("Role assigned.");
      void loadMembers();
    } catch (nextError) {
      setAssignRoleMessage(normalizeOrganizationMembersRouteError(nextError).message);
      setAssignRoleState("error");
    }
  }

  async function handleRemoveMember(memberId: number) {
    const token = readBrowserSessionToken();
    if (!token || !organizationId) return;
    setRemoveMemberMessage(null);
    try {
      await removeMemberFromOrganization({ memberId, organizationId, token });
      setRemoveMemberMessage("Member removed.");
      void loadMembers();
    } catch (nextError) {
      setRemoveMemberMessage(normalizeOrganizationMembersRouteError(nextError).message);
    }
  }

  async function handleInviteMember(event: FormEvent) {
    event.preventDefault();
    const token = readBrowserSessionToken();
    if (!token || !organizationId || !inviteEmail.trim()) {
      setInviteMessage("Enter an email address.");
      setInviteState("error");
      return;
    }

    setInviteState("saving");
    setInviteMessage(null);
    try {
      await addMemberByEmail({
        email: inviteEmail.trim(),
        organizationId,
        roleName: inviteRole || undefined,
        token,
      });
      setInviteEmail("");
      setInviteRole("");
      setInviteState("success");
      setInviteMessage("Member added. Existing-user access is active immediately; pending learner joins stay on Courses.");
      void loadMembers();
    } catch (nextError) {
      setInviteMessage(normalizeOrganizationMembersRouteError(nextError).message);
      setInviteState("error");
    }
  }

  return {
    assignRoleMessage,
    assignRoleState,
    handleAssignRole,
    handleInviteMember,
    handleRemoveMember,
    inviteEmail,
    inviteMessage,
    inviteRole,
    inviteState,
    removeMemberMessage,
    setInviteEmail,
    setInviteRole,
  };
}
