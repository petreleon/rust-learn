import { platformPermissionEnabled } from "@/lib/admin/platformPermissionEnabled";
import { type PlatformAdminWorkspace } from "@/lib/admin/PlatformAdminWorkspace";
import { type PlatformCapability } from "@/lib/admin/PlatformCapability";

export const emptyTeacherApplicationWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

export function canReviewTeacherApplications(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "REVIEW_TEACHER_APPLICATIONS");
}

export function canApproveTeacherApplication(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "APPROVE_TEACHER_APPLICATION");
}

export function canRejectTeacherApplication(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "REJECT_TEACHER_APPLICATION");
}

export function findTeacherApplicationCapability(workspace: PlatformAdminWorkspace): PlatformCapability {
  return (
    workspace.capabilities.find((item) => item.key === "teacher_applications") ?? {
      enabled: false,
      key: "teacher_applications",
      label: "Teacher application review",
      permissions: ["REVIEW_TEACHER_APPLICATIONS"],
    }
  );
}
