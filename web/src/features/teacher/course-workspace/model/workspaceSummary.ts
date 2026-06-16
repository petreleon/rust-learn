import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";

export function workspaceSummary(workspace: TeacherCourseWorkspaceResponse) {
  return workspace.chapters.reduce(
    (totals, chapter) => {
      totals.contentCount += chapter.contents.length;
      for (const content of chapter.contents) {
        if (content.display_state === "failed_processing") {
          totals.failedProcessingCount += 1;
        }
      }
      return totals;
    },
    {
      contentCount: 0,
      failedProcessingCount: 0,
    },
  );
}

export function hasProcessingContent(workspace: TeacherCourseWorkspaceResponse) {
  return workspace.chapters.some((chapter) =>
    chapter.contents.some((content) => content.display_state === "processing"),
  );
}
