import { type CurrentSession } from "@/lib/session/CurrentSession";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { fetchTeachingCourseWorkspace } from "@/lib/teacher/fetchTeachingCourseWorkspace";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";

export type TeacherCourseWorkspaceData = {
  session: CurrentSession;
  workspace: TeacherCourseWorkspaceResponse;
};

export async function loadTeacherCourseWorkspace({
  courseId,
  token,
}: {
  courseId: string;
  token: string;
}): Promise<TeacherCourseWorkspaceData> {
  const [session, workspace] = await Promise.all([
    fetchCurrentSession({ token }),
    fetchTeachingCourseWorkspace({ courseId, token }),
  ]);

  return { session, workspace };
}
