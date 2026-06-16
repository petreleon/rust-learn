import { type CurrentSession } from "@/lib/session/CurrentSession";

export function sessionWorkspaceCount(session: CurrentSession | null) {
  if (!session) return 0;
  return session.organizations.length + session.courses.length;
}
