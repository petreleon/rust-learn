import { type CurrentSession } from "@/lib/session/CurrentSession";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";

export function loadSessionWorkspace({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}
