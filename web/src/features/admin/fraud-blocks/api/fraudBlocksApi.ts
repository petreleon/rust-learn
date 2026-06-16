import { createFraudBlock } from "@/lib/admin/createFraudBlock";
import { fetchFraudBlockAudit } from "@/lib/admin/fetchFraudBlockAudit";
import { fetchFraudBlocks } from "@/lib/admin/fetchFraudBlocks";
import { revokeFraudBlock } from "@/lib/admin/revokeFraudBlock";
import { type FraudBlockAuditEvent } from "@/lib/admin/FraudBlockAuditEvent";
import { type FraudBlockItem } from "@/lib/admin/FraudBlockItem";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { ADMIN_FRAUD_BLOCK_PAGE_SIZE } from "../model/fraudBlockPagination";

export type FraudBlockListResult = {
  blocks: FraudBlockItem[];
  limit: number;
  offset: number;
  total: number;
};

export function loadAdminFraudBlockSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function loadAdminFraudBlocks({
  active,
  offset,
  scopeType,
  token,
}: {
  active: boolean | null;
  offset: number;
  scopeType: string;
  token: string;
}): Promise<FraudBlockListResult> {
  return fetchFraudBlocks({
    active,
    limit: ADMIN_FRAUD_BLOCK_PAGE_SIZE,
    offset,
    scope_type: scopeType || null,
    token,
  });
}

export function loadAdminFraudBlockAudit({
  blockId,
  token,
}: {
  blockId: number;
  token: string;
}): Promise<FraudBlockAuditEvent[]> {
  return fetchFraudBlockAudit({ blockId, token });
}

export function createAdminFraudBlock({
  evidence,
  reason,
  scopeType,
  targetId,
  token,
}: {
  evidence: string;
  reason: string;
  scopeType: string;
  targetId: string;
  token: string;
}): Promise<FraudBlockItem> {
  const numericTargetId = Number(targetId.trim());
  const hasNumericTarget = targetId.trim().length > 0 && !Number.isNaN(numericTargetId);

  return createFraudBlock({
    course_id: hasNumericTarget && scopeType === "course" ? numericTargetId : null,
    evidence_reference: evidence.trim() || null,
    organization_id: hasNumericTarget && scopeType === "organization" ? numericTargetId : null,
    reason: reason.trim(),
    reward_policy_id: hasNumericTarget && scopeType === "reward_policy" ? numericTargetId : null,
    scope_type: scopeType,
    teacher_user_id: hasNumericTarget && scopeType === "teacher" ? numericTargetId : null,
    token,
  });
}

export function revokeAdminFraudBlock({
  blockId,
  token,
}: {
  blockId: number;
  token: string;
}): Promise<FraudBlockItem> {
  return revokeFraudBlock({ blockId, token });
}
