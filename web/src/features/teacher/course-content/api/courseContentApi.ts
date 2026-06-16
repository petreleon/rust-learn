import { fetchCurrentSession, type CurrentSession } from "@/lib/session";
import {
  createTeacherChapter,
  createTeacherContent,
  deleteTeacherContent,
  fetchTeacherAssessments,
  fetchTeachingCourseWorkspace,
  fetchUploadUrl,
  processContent,
  updateTeacherContent,
  type CreateTeacherContentPayload,
  type TeacherAssessment,
  type TeacherContent,
  type TeacherCourseWorkspaceResponse,
} from "@/lib/teacher";

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

export function uploadTeacherContentFile({
  file,
  onProgress,
  uploadUrl,
}: {
  file: File;
  onProgress: (progress: number) => void;
  uploadUrl: string;
}): Promise<void> {
  return new Promise((resolve, reject) => {
    const request = new XMLHttpRequest();
    request.open("PUT", uploadUrl);
    request.upload.onprogress = (event) => {
      if (event.lengthComputable && event.total > 0) {
        onProgress(Math.round((event.loaded / event.total) * 100));
      }
    };
    request.onload = () => {
      if (request.status >= 200 && request.status < 300) {
        onProgress(100);
        resolve();
        return;
      }
      reject(new Error(`Upload failed: ${request.status} ${request.statusText}`.trim()));
    };
    request.onerror = () => reject(new Error("Upload failed: network error"));
    onProgress(0);
    request.send(file);
  });
}
