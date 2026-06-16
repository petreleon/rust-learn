import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { fetchMyTeacherApplication } from "@/lib/teacher/fetchMyTeacherApplication";
import { submitTeacherApplication } from "@/lib/teacher/submitTeacherApplication";
import { type TeacherApplicationSnapshot } from "@/lib/teacher/TeacherApplicationSnapshot";
import { type ApplicationDraft } from "../model/ApplicationDraft";

export type TeacherApplicationData = {
  session: CurrentSession;
  snapshot: TeacherApplicationSnapshot;
};

export async function loadTeacherApplicationData({ token }: { token: string }): Promise<TeacherApplicationData> {
  const session = await fetchCurrentSession({ token });
  const snapshot = await fetchMyTeacherApplication({ token });

  return { session, snapshot };
}

export function loadTeacherApplicationSnapshot({ token }: { token: string }) {
  return fetchMyTeacherApplication({ token });
}

export function submitTeacherApplicationDraft({
  draft,
  portfolioLinks,
  token,
}: {
  draft: ApplicationDraft;
  portfolioLinks: string[];
  token: string;
}) {
  return submitTeacherApplication({
    payload: {
      experience_summary: draft.experienceSummary.trim(),
      idempotency_key: draft.idempotencyKey,
      portfolio_links: portfolioLinks,
      requested_course_id: draft.requestedScope === "course" ? Number(draft.requestedCourseId) : undefined,
      requested_organization_id:
        draft.requestedScope === "organization" ? Number(draft.requestedOrganizationId) : undefined,
      requested_scope: draft.requestedScope,
    },
    token,
  });
}
