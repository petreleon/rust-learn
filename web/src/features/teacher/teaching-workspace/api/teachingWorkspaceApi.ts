import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { fetchMyTeacherApplication } from "@/lib/teacher/fetchMyTeacherApplication";
import { fetchTeachingCourses } from "@/lib/teacher/fetchTeachingCourses";
import { type TeacherApplicationSnapshot } from "@/lib/teacher/TeacherApplicationSnapshot";
import { type TeacherCoursesResponse } from "@/lib/teacher/TeacherCoursesResponse";
import { type CourseQuery } from "../model/CourseQuery";

export type TeachingWorkspaceData = {
  applicationSnapshot: TeacherApplicationSnapshot;
  courses: TeacherCoursesResponse;
  session: CurrentSession;
};

export async function loadTeachingWorkspaceData({
  query,
  token,
}: {
  query: CourseQuery;
  token: string;
}): Promise<TeachingWorkspaceData> {
  const [session, courses, applicationSnapshot] = await Promise.all([
    fetchCurrentSession({ token }),
    fetchTeachingCourses({
      lifecycleStatus: query.lifecycleStatus,
      search: query.search,
      token,
    }),
    fetchMyTeacherApplication({ token }),
  ]);

  return { applicationSnapshot, courses, session };
}
