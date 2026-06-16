import { type PlatformTeacherApplicationItem } from "@/lib/admin/PlatformTeacherApplicationItem";
import { type TeacherApplicationStatus } from "@/lib/admin/TeacherApplicationStatus";
import { isFinalTeacherApplicationStatus } from "./teacherApplicationDisplay";

export type TeacherApplicationDecisionStatus = Extract<
  TeacherApplicationStatus,
  "approved" | "needs_changes" | "rejected"
>;

export type TeacherApplicationDecisionDraft = {
  reason: string;
  status: TeacherApplicationDecisionStatus;
};

export const defaultTeacherApplicationDecisionDraft: TeacherApplicationDecisionDraft = {
  reason: "",
  status: "needs_changes",
};

export function canSubmitTeacherApplicationDecision({
  application,
  canApprove,
  canReject,
  draft,
  state,
}: {
  application: PlatformTeacherApplicationItem;
  canApprove: boolean;
  canReject: boolean;
  draft: TeacherApplicationDecisionDraft;
  state: string;
}) {
  if (state === "submitting" || isFinalTeacherApplicationStatus(application.status)) return false;
  if (!draft.reason.trim()) return false;
  if (draft.status === "needs_changes") return true;
  if (draft.status === "approved") return canApprove;
  return canReject;
}
