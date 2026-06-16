"use client";

import { useMemo } from "react";
import { accessSummary } from "@/lib/access";
import {
  countSessionCapabilityPermissions,
  sessionCapabilityPermissionGroups,
  sessionPermissionEnabled,
} from "@/lib/session";
import { flowStatus } from "../model/flowStatus";
import { useDelegationOpsWorkflow } from "./useDelegationOpsWorkflow";
import { useFraudOpsWorkflow } from "./useFraudOpsWorkflow";
import { useOpsSessionController } from "./useOpsSessionController";
import { useReportOpsWorkflow } from "./useReportOpsWorkflow";
import { useRewardOpsWorkflow } from "./useRewardOpsWorkflow";
import { useTeacherOpsWorkflow } from "./useTeacherOpsWorkflow";

export function useOpsConsoleController() {
  const auth = useOpsSessionController();
  const hasPermission = (permission: string) => sessionPermissionEnabled(auth.session, permission);
  const teacher = useTeacherOpsWorkflow({ hasPermission, sendApi: auth.sendApi });
  const reward = useRewardOpsWorkflow({ hasPermission, sendApi: auth.sendApi });
  const report = useReportOpsWorkflow({ hasPermission, sendApi: auth.sendApi });
  const fraud = useFraudOpsWorkflow({ hasPermission, sendApi: auth.sendApi });
  const delegation = useDelegationOpsWorkflow({ hasPermission, sendApi: auth.sendApi });
  const permissionGroups = useMemo(() => sessionCapabilityPermissionGroups(auth.session), [auth.session]);
  const enabledPermissionCount = useMemo(() => countSessionCapabilityPermissions(auth.session), [auth.session]);
  const isPlatformAdmin = auth.session ? accessSummary(auth.session).platformAdmin : false;
  const teacherWorkflowServerDenied = teacher.visibleActions.some(auth.isServerDenied);
  const rewardWorkflowServerDenied = reward.visibleActions.some(auth.isServerDenied);
  const auditWorkflowServerDenied = [
    ...report.visibleActions,
    ...fraud.visibleActions,
    ...delegation.visibleActions,
  ].some(auth.isServerDenied);
  const canUseAuditWorkflow = report.canUseReportWorkflow || fraud.canUseFraudWorkflow || delegation.canDelegate;

  return {
    auditFlowStatus: flowStatus(canUseAuditWorkflow, auth.hasSessionToken, auditWorkflowServerDenied),
    auth,
    delegation,
    enabledPermissionCount,
    fraud,
    isPlatformAdmin,
    permissionGroups,
    report,
    reward,
    rewardFlowStatus: flowStatus(reward.canUseRewardWorkflow, auth.hasSessionToken, rewardWorkflowServerDenied),
    teacher,
    teacherFlowStatus: flowStatus(teacher.canUseTeacherWorkflow, auth.hasSessionToken, teacherWorkflowServerDenied),
  };
}

export type OpsConsoleController = ReturnType<typeof useOpsConsoleController>;
