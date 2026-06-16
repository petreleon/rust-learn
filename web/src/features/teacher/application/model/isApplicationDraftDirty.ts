import { type ApplicationDraft } from "./ApplicationDraft";

export function isApplicationDraftDirty(draft: ApplicationDraft) {
  return (
    draft.experienceSummary.trim() !== "" ||
    draft.portfolioLinks.trim() !== "" ||
    draft.requestedCourseId !== "" ||
    draft.requestedOrganizationId !== "" ||
    draft.requestedScope !== "platform"
  );
}
