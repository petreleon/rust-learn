"use client";

import { Ban, ClipboardList, KeyRound } from "lucide-react";
import styles from "../ops-console.module.css";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { optionLabel } from "../model/optionLabel";
import { positiveIntegerInputProps } from "../model/positiveIntegerInputProps";
import { OpsServerDeniedNotice } from "./OpsServerDeniedNotice";
import { PermissionNotice } from "./PermissionNotice";
import { RequirementNotice } from "./RequirementNotice";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function DelegationWorkflowPanel({ controller }: { controller: OpsConsoleController }) {
  const { auth, delegation } = controller;

  return (
    <section id="delegation-workflow" className={styles.panel} aria-labelledby="delegation-title">
      <div className={styles.panelHeader}>
        <div>
          <p className={styles.eyebrow}>Delegation</p>
          <h2 id="delegation-title">Reward permissions</h2>
        </div>
        <KeyRound size={22} aria-hidden />
      </div>
      {!delegation.canDelegate && (
        <PermissionNotice
          title="Delegation permission disabled"
          detail="Enable delegated reward approval permission to grant or revoke delegations."
        />
      )}
      {delegation.canDelegate && (
        <>
          {auth.hasSessionToken && delegation.hasDelegationGrantDraft && (
            <RequirementNotice action="Grant delegation" fields={delegation.delegationGrantMissingFields} />
          )}
          <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.grantDelegatedPermission} action="Grant" />
          <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.delegatedPermissions} action="Load" />
          <DelegationGrantControls controller={controller} />
          {auth.hasSessionToken && delegation.hasDelegationRevokeDraft && (
            <RequirementNotice action="Revoke delegation" fields={delegation.delegationRevokeMissingFields} />
          )}
          <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.revokeDelegatedPermission} action="Revoke" />
          <DelegationRevokeControls controller={controller} />
        </>
      )}
    </section>
  );
}

function DelegationGrantControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, delegation } = controller;

  return (
    <fieldset className={styles.formGrid}>
      <input
        aria-label="Delegation grantee user id"
        {...positiveIntegerInputProps}
        placeholder="Grantee user id"
        value={delegation.delegation.grantee_user_id}
        onChange={(event) => delegation.setDelegation((current) => ({ ...current, grantee_user_id: event.target.value }))}
      />
      <select
        aria-label="Delegated permission"
        value={delegation.activeDelegatedPermission}
        onChange={(event) => delegation.setDelegation((current) => ({ ...current, permission: event.target.value }))}
      >
        {delegation.scopedDelegatedPermissions.map((permission) => (
          <option key={permission} value={permission}>
            {optionLabel(permission)}
          </option>
        ))}
      </select>
      <select
        aria-label="Delegation scope type"
        value={delegation.delegation.scope_type}
        onChange={(event) => delegation.setDelegation((current) => ({ ...current, scope_type: event.target.value }))}
      >
        <option value="platform">{optionLabel("platform")}</option>
        <option value="organization">{optionLabel("organization")}</option>
        <option value="course">{optionLabel("course")}</option>
      </select>
      {delegation.delegation.scope_type === "organization" && (
        <input
          aria-label="Delegation organization id"
          {...positiveIntegerInputProps}
          placeholder="Organization id"
          value={delegation.delegation.organization_id}
          onChange={(event) => delegation.setDelegation((current) => ({ ...current, organization_id: event.target.value }))}
        />
      )}
      {delegation.delegation.scope_type === "course" && (
        <input
          aria-label="Delegation course id"
          {...positiveIntegerInputProps}
          placeholder="Course id"
          value={delegation.delegation.course_id}
          onChange={(event) => delegation.setDelegation((current) => ({ ...current, course_id: event.target.value }))}
        />
      )}
      <label className={styles.fieldLabel}>
        Expires at
        <input
          aria-label="Delegation expiration"
          type="datetime-local"
          value={delegation.delegation.expires_at}
          onChange={(event) => delegation.setDelegation((current) => ({ ...current, expires_at: event.target.value }))}
        />
      </label>
      <input
        aria-label="Delegation reason"
        className={styles.fullWidth}
        placeholder="Reason"
        value={delegation.delegation.reason}
        onChange={(event) => delegation.setDelegation((current) => ({ ...current, reason: event.target.value }))}
      />
      <button
        type="button"
        className={styles.primaryButton}
        onClick={delegation.grantDelegation}
        {...auth.actionState(
          delegation.canGrantDelegationForm,
          true,
          "Required permission",
          PROTECTED_ACTIONS.grantDelegatedPermission,
        )}
      >
        <KeyRound size={17} aria-hidden />
        <span>Grant</span>
      </button>
      <button
        type="button"
        className={styles.secondaryButton}
        onClick={delegation.listDelegations}
        {...auth.actionState(true, true, "Required permission", PROTECTED_ACTIONS.delegatedPermissions)}
      >
        <ClipboardList size={17} aria-hidden />
        <span>Load</span>
      </button>
    </fieldset>
  );
}

function DelegationRevokeControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, delegation } = controller;

  return (
    <div className={styles.actionStrip}>
      <input
        aria-label="Delegation id"
        {...positiveIntegerInputProps}
        placeholder="Delegation id"
        value={delegation.delegationId}
        onChange={(event) => delegation.setDelegationId(event.target.value)}
      />
      <input
        aria-label="Delegation revoke reason"
        placeholder="Revoke reason"
        value={delegation.revokeReason}
        onChange={(event) => delegation.setRevokeReason(event.target.value)}
      />
      <button
        type="button"
        className={styles.secondaryButton}
        onClick={delegation.revokeDelegation}
        {...auth.actionState(
          delegation.canRevokeDelegationForm,
          true,
          "Required permission",
          PROTECTED_ACTIONS.revokeDelegatedPermission,
        )}
      >
        <Ban size={17} aria-hidden />
        <span>Revoke</span>
      </button>
    </div>
  );
}
