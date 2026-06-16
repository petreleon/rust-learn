import { decideTeacherApplication } from "@/lib/admin/decideTeacherApplication";
import { fetchPlatformTeacherApplications } from "@/lib/admin/fetchPlatformTeacherApplications";
import { fetchTeacherApplicationAudit } from "@/lib/admin/fetchTeacherApplicationAudit";
import { type PlatformTeacherApplicationsResponse } from "@/lib/admin/PlatformTeacherApplicationsResponse";
import { type TeacherApplication } from "@/lib/admin/TeacherApplication";
import { type TeacherApplicationAuditEvent } from "@/lib/admin/TeacherApplicationAuditEvent";
import { type TeacherApplicationStatus } from "@/lib/admin/TeacherApplicationStatus";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherApplicationDecisionStatus } from "../model/TeacherApplicationDecisionDraft";
import { ADMIN_TEACHER_APPLICATION_PAGE_SIZE } from "../model/teacherApplicationPagination";

export function loadAdminTeacherApplicationSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function loadAdminTeacherApplications({
  offset,
  search,
  status,
  token,
}: {
  offset: number;
  search: string;
  status: TeacherApplicationStatus | "";
  token: string;
}): Promise<PlatformTeacherApplicationsResponse> {
  return fetchPlatformTeacherApplications({
    limit: ADMIN_TEACHER_APPLICATION_PAGE_SIZE,
    offset,
    search,
    status,
    token,
  });
}

export function loadAdminTeacherApplicationAudit({
  applicationId,
  token,
}: {
  applicationId: number;
  token: string;
}): Promise<TeacherApplicationAuditEvent[]> {
  return fetchTeacherApplicationAudit({ applicationId, token });
}

export function decideAdminTeacherApplication({
  applicationId,
  decisionReason,
  status,
  token,
}: {
  applicationId: number;
  decisionReason: string;
  status: TeacherApplicationDecisionStatus;
  token: string;
}): Promise<TeacherApplication> {
  return decideTeacherApplication({
    applicationId,
    decisionReason,
    status,
    token,
  });
}
