import { type CurrentSession } from "@/lib/session/CurrentSession";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { fetchTeachingCourseWorkspace } from "@/lib/teacher/fetchTeachingCourseWorkspace";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { updateTeacherCourse } from "@/lib/teacher/updateTeacherCourse";
import { updateTeacherCourseLifecycle } from "@/lib/teacher/updateTeacherCourseLifecycle";
import { courseSettingsPayload, type CourseSettingsDraft } from "../model/courseSettingsModel";

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

export async function saveTeacherCourseSettings({
  courseId,
  draft,
  token,
}: {
  courseId: string;
  draft: CourseSettingsDraft;
  token: string;
}) {
  return updateTeacherCourse({
    courseId,
    payload: courseSettingsPayload(draft),
    token,
  });
}

export async function saveTeacherCourseLifecycle({
  courseId,
  status,
  token,
}: {
  courseId: string;
  status: string;
  token: string;
}) {
  return updateTeacherCourseLifecycle({ courseId, status, token });
}
