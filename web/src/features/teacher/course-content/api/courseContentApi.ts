import { fetchCurrentSession, type CurrentSession } from "@/lib/session";
import {
  createTeacherChapter,
  createTeacherContent,
  deleteTeacherContent,
  fetchTeacherContentProcessingHistory,
  fetchTeacherAssessments,
  fetchTeachingCourseWorkspace,
  fetchUploadUrl,
  processContent,
  updateTeacherContent,
  type CreateTeacherContentPayload,
  type TeacherAssessment,
  type TeacherContentProcessingHistory,
  type TeacherContent,
  type TeacherCourseWorkspaceResponse,
} from "@/lib/teacher";
export { uploadTeacherContentFile } from "./uploadTeacherContentFile";

export type TeacherCourseContentLoadResult = {
  assessments: TeacherAssessment[];
  session: CurrentSession;
  workspace: TeacherCourseWorkspaceResponse;
};

export function loadTeacherCourseContentWorkspace({
  courseId,
  token,
}: {
  courseId: string;
  token: string;
}): Promise<TeacherCourseContentLoadResult> {
  return Promise.all([
    fetchCurrentSession({ token }),
    fetchTeacherAssessments({ courseId, token }),
    fetchTeachingCourseWorkspace({ courseId, token }),
  ]).then(([session, assessments, workspace]) => ({ assessments, session, workspace }));
}

export function createTeacherCourseContentChapter({
  courseId,
  order,
  title,
  token,
}: {
  courseId: string;
  order: number;
  title: string;
  token: string;
}) {
  return createTeacherChapter({ courseId, payload: { order, title }, token });
}

export function createTeacherCourseContentItem({
  chapterId,
  courseId,
  payload,
  token,
}: {
  chapterId: string;
  courseId: string;
  payload: CreateTeacherContentPayload;
  token: string;
}): Promise<TeacherContent> {
  return createTeacherContent({ chapterId, courseId, payload, token });
}

export function updateTeacherCourseContentItem({
  chapterId,
  contentId,
  courseId,
  payload,
  token,
}: {
  chapterId: string;
  contentId: number;
  courseId: string;
  payload: CreateTeacherContentPayload;
  token: string;
}): Promise<TeacherContent> {
  return updateTeacherContent({ chapterId, contentId, courseId, payload, token });
}

export function deleteTeacherCourseContentItem({
  chapterId,
  contentId,
  courseId,
  token,
}: {
  chapterId: number;
  contentId: number;
  courseId: string;
  token: string;
}) {
  return deleteTeacherContent({ chapterId, contentId, courseId, token });
}

export function processTeacherCourseContentItem({
  chapterId,
  contentId,
  courseId,
  token,
}: {
  chapterId: number;
  contentId: number;
  courseId: string;
  token: string;
}) {
  return processContent({ chapterId, contentId, courseId: Number(courseId), token });
}
export function loadTeacherContentProcessingHistory({
  chapterId,
  contentId,
  courseId,
  token,
}: {
  chapterId: number;
  contentId: number;
  courseId: string;
  token: string;
}): Promise<TeacherContentProcessingHistory> {
  return fetchTeacherContentProcessingHistory({
    chapterId,
    contentId,
    courseId: Number(courseId),
    token,
  });
}

export function requestTeacherContentUploadUrl({
  chapterId,
  contentType,
  courseId,
  filename,
  token,
}: {
  chapterId: string;
  contentType: string;
  courseId: string;
  filename: string;
  token: string;
}) {
  return fetchUploadUrl({
    chapterId: Number(chapterId),
    contentType,
    courseId: Number(courseId),
    filename,
    token,
  });
}
