"use client";

import { Ban, CheckCircle2, ClipboardList, History } from "lucide-react";
import styles from "../ops-console.module.css";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { fraudBlockScopes } from "../model/fraudBlockScopes";
import { optionLabel } from "../model/optionLabel";
import { positiveIntegerInputProps } from "../model/positiveIntegerInputProps";
import { OpsServerDeniedNotice } from "./OpsServerDeniedNotice";
import { PermissionNotice } from "./PermissionNotice";
import { RequirementNotice } from "./RequirementNotice";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function FraudCreateControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, fraud } = controller;

  return (
    <>
      {auth.hasSessionToken && fraud.hasFraudBlockCreateDraft && (
        <RequirementNotice action="Create block" fields={fraud.fraudBlockCreateMissingFields} />
      )}
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.createRewardFraudBlock} action="Create block" />
      {auth.hasSessionToken && !fraud.canCreateSelectedFraudScope && (
        <PermissionNotice
          title="Selected block scope permission disabled"
          detail={fraud.selectedFraudBlockScopePermission.detail}
        />
      )}
      <div className={styles.formGrid}>
        <label className={styles.fieldLabel}>
          Scope
          <select
            aria-label="Fraud block scope type"
            value={fraud.activeFraudBlockScope}
            onChange={(event) => fraud.setFraudBlock((current) => ({ ...current, scope_type: event.target.value }))}
          >
            {fraudBlockScopes.map((scope) => (
              <option key={scope} value={scope} disabled={!fraud.fraudBlockScopePermissions[scope]?.allowed}>
                {optionLabel(scope)}
              </option>
            ))}
          </select>
        </label>
        <FraudScopeTargetInputs controller={controller} />
        <input
          aria-label="Fraud block evidence reference"
          placeholder="Evidence reference"
          value={fraud.fraudBlock.evidence_reference}
          onChange={(event) => fraud.setFraudBlock((current) => ({ ...current, evidence_reference: event.target.value }))}
        />
        <label className={`${styles.fieldLabel} ${styles.fullWidth}`}>
          Reason
          <textarea
            rows={2}
            value={fraud.fraudBlock.reason}
            onChange={(event) => fraud.setFraudBlock((current) => ({ ...current, reason: event.target.value }))}
          />
        </label>
        <button
          type="button"
          className={styles.primaryButton}
          onClick={fraud.createFraudBlock}
          {...auth.actionState(
            fraud.canCreateFraudBlockFields,
            fraud.canCreateSelectedFraudScope,
            fraud.selectedFraudBlockScopePermission.permissionTitle,
            PROTECTED_ACTIONS.createRewardFraudBlock,
          )}
        >
          <Ban size={17} aria-hidden />
          <span>Create block</span>
        </button>
      </div>
    </>
  );
}

function FraudScopeTargetInputs({ controller }: { controller: OpsConsoleController }) {
  const { fraud } = controller;
  const targetFields = [
    ["teacher", "Fraud block teacher user id", "Teacher user id", "teacher_user_id"],
    ["organization", "Fraud block organization id", "Organization id", "organization_id"],
    ["course", "Fraud block course id", "Course id", "course_id"],
    ["reward_policy", "Fraud block policy id", "Policy id", "reward_policy_id"],
  ] as const;

  return (
    <>
      {targetFields.map(([scope, label, placeholder, key]) =>
        fraud.activeFraudBlockScope === scope ? (
          <input
            key={scope}
            aria-label={label}
            {...positiveIntegerInputProps}
            placeholder={placeholder}
            value={fraud.fraudBlock[key]}
            onChange={(event) => fraud.setFraudBlock((current) => ({ ...current, [key]: event.target.value }))}
          />
        ) : null,
      )}
    </>
  );
}

export function FraudUseControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, fraud } = controller;

  return (
    <>
      {auth.hasSessionToken && fraud.hasFraudBlockUseDraft && (
        <RequirementNotice action={fraud.fraudBlockActionLabel} fields={fraud.fraudBlockUseMissingFields} />
      )}
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.rewardFraudBlocks} action="Load active" />
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.rewardFraudAudit} action="Audit" />
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.revokeRewardFraudBlock} action="Revoke" />
      <div className={styles.actionStrip}>
        {fraud.canViewFraud && (
          <button
            type="button"
            className={styles.secondaryButton}
            onClick={fraud.listFraudBlocks}
            {...auth.actionState(true, true, "Required permission", PROTECTED_ACTIONS.rewardFraudBlocks)}
          >
            <ClipboardList size={17} aria-hidden />
            <span>Load active</span>
          </button>
        )}
        <input
          aria-label="Fraud block id"
          {...positiveIntegerInputProps}
          placeholder="Block id"
          value={fraud.fraudBlockId}
          onChange={(event) => fraud.setFraudBlockId(event.target.value)}
        />
        {fraud.canViewFraud && (
          <button
            type="button"
            className={styles.secondaryButton}
            onClick={fraud.loadFraudAudit}
            {...auth.actionState(fraud.canUseFraudBlockForm, true, "Required permission", PROTECTED_ACTIONS.rewardFraudAudit)}
          >
            <History size={17} aria-hidden />
            <span>Audit</span>
          </button>
        )}
        {fraud.canManageFraud && (
          <button
            type="button"
            className={styles.secondaryButton}
            onClick={fraud.revokeFraudBlock}
            {...auth.actionState(
              fraud.canUseFraudBlockForm,
              true,
              "Required permission",
              PROTECTED_ACTIONS.revokeRewardFraudBlock,
            )}
          >
            <CheckCircle2 size={17} aria-hidden />
            <span>Revoke</span>
          </button>
        )}
      </div>
    </>
  );
}
