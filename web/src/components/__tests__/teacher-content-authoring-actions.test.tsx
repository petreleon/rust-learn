import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ContentAuthoringForm } from "@/features/teacher/shared/route-kit/ContentAuthoringForm";
import { ContentRow } from "@/features/teacher/shared/route-kit/ContentRow";
import { type ContentDraft } from "@/features/teacher/shared/route-kit/ContentDraft";
import {
  type TeacherCourseWorkspaceContent,
  type TeacherCourseWorkspaceResponse,
} from "@/lib/teacher";

const content: TeacherCourseWorkspaceContent = {
  content_type: "article",
  data: "Original lesson body",
  data_present: true,
  display_state: "ready",
  id: 11,
  order: 0,
  processing_error: null,
  processing_status: null,
  publication_status: "published",
};

const draft: ContentDraft = {
  chapterId: "3",
  contentType: "article",
  data: "Original lesson body",
  file: null,
  filename: "",
  order: "0",
  uploadKind: "text",
};

function workspace(): TeacherCourseWorkspaceResponse {
  return {
    chapters: [{ contents: [content], id: 3, order: 0, title: "Intro" }],
    course: {
      content: { chapter_count: 1, content_count: 1, content_types: ["article"], has_content: true },
      id: 9,
      lifecycle_status: "published",
      organizations: [],
      permissions: {
        can_approve_reward_candidates: false,
        can_manage_content: true,
        can_manage_enrollments: true,
        can_manage_reward_rules: false,
        can_manage_settings: true,
        can_view_reward_candidates: false,
      },
      reward_queue: { failed_count: 0, pending_teacher_count: 0, teacher_approved_count: 0 },
      rewards: { active_policy_count: 0, available: false, event_types: [], payment_strategies: [], token_amounts: [] },
      roster: { enrolled_student_count: 0, pending_join_request_count: 0, waitlisted_join_request_count: 0 },
      title: "Rust Safety",
    },
    publication: {
      content_publication_status_supported: true,
      course_lifecycle_status: "published",
    },
    teacher_roles: ["TEACHER"],
  };
}

describe("teacher content authoring actions", () => {
  it("shows edit and two-step delete controls for manageable text content", () => {
    const onDelete = vi.fn();
    const onEdit = vi.fn();
    render(
      <ContentRow
        canManageContent
        content={content}
        deleteConfirmContentId={content.id}
        onDeleteContent={onDelete}
        onEditContent={onEdit}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: "Edit" }));
    fireEvent.click(screen.getByRole("button", { name: "Confirm delete" }));

    expect(onEdit).toHaveBeenCalledWith(content);
    expect(onDelete).toHaveBeenCalledWith(content);
  });

  it("shows publication toggle for manageable content", () => {
    const onSetContentPublicationStatus = vi.fn();
    render(
      <ContentRow
        canManageContent
        content={content}
        onSetContentPublicationStatus={onSetContentPublicationStatus}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: "Unpublish" }));
    expect(onSetContentPublicationStatus).toHaveBeenCalledWith(content, "unpublished");
  });

  it("locks chapter and upload kind while editing text content", () => {
    const onCancel = vi.fn();
    render(
      <ContentAuthoringForm
        actionState="idle"
        canManageContent
        contentDraft={draft}
        editingContentId={content.id}
        onCancelEdit={onCancel}
        onContentDraftChange={vi.fn()}
        onSubmitContent={vi.fn()}
        uploadProgress={null}
        workspace={workspace()}
      />,
    );

    expect(screen.getByRole("heading", { name: "Edit text content" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Update content" })).toBeInTheDocument();
    expect(screen.getByLabelText("Chapter")).toBeDisabled();
    expect(screen.getByLabelText("Kind")).toBeDisabled();

    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(onCancel).toHaveBeenCalled();
  });
});
