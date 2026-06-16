import { type TeacherCourseWorkspaceContent } from "@/lib/teacher";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { findContentChapter } from "@/features/teacher/shared/route-kit/contentAuthoringHelpers";
import { setTeacherCourseContentPublicationStatus } from "../api/courseContentApi";
import { type CourseContentActionArgs, runContentAction } from "./courseContentActionHelpers";

export async function updateContentPublicationStatus(
  args: CourseContentActionArgs,
  content: TeacherCourseWorkspaceContent,
  publicationStatus: "published" | "unpublished",
) {
  const chapter = findContentChapter(args.workspace, content);
  const token = readBrowserSessionToken();
  if (!token || !chapter) {
    args.setActionMessage("Chapter not found for this content item.");
    return;
  }

  await runContentAction(args, `Content item ${publicationStatus}.`, async () => {
    const response = await setTeacherCourseContentPublicationStatus({
      chapterId: chapter.id,
      contentId: content.id,
      courseId: args.courseId,
      publicationStatus,
      token,
    });
    return `Content item ${response.publication_status}.`;
  });
}
