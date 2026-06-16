import { type CurrentSession } from "@/lib/session/CurrentSession";

export function accountWorkspaceSummary(session: CurrentSession | null) {
  if (!session) return "No workspace loaded";
  const count = session.organizations.length + session.courses.length;
  return `${count} workspace${count === 1 ? "" : "s"}`;
}
