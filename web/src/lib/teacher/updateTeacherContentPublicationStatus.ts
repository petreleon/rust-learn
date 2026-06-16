import { updateTeacherContent } from "./updateTeacherContent";
import { type TeacherContent } from "./TeacherContent";
import { type UpdateTeacherContentPublicationStatusOptions } from "./UpdateTeacherContentPublicationStatusOptions";

export function updateTeacherContentPublicationStatus({
  apiRoot,
  chapterId,
  contentId,
  courseId,
  publicationStatus,
  timeoutMs,
  token,
}: UpdateTeacherContentPublicationStatusOptions): Promise<TeacherContent> {
  return updateTeacherContent({
    apiRoot,
    chapterId,
    contentId,
    courseId,
    payload: { publication_status: publicationStatus },
    timeoutMs,
    token,
  });
}
