import { type Dispatch, type FormEvent, type SetStateAction } from "react";
import { readStoredSessionToken } from "@/lib/session";
import { createTeacherChapter } from "@/lib/teacher";
import { type ActionState } from "./ActionState";
import { type ChapterDraft } from "./ChapterDraft";
import { defaultChapterDraft } from "./defaultChapterDraft";
import { normalizeRouteError } from "./normalizeRouteError";

type Args = {
  chapterDraft: ChapterDraft;
  courseId: string;
  loadContentRoute: () => Promise<void>;
  setActionMessage: Dispatch<SetStateAction<string | null>>;
  setActionState: Dispatch<SetStateAction<ActionState>>;
  setChapterDraft: Dispatch<SetStateAction<ChapterDraft>>;
};

export function createChapterSubmitAction(args: Args) {
  return async function submitChapter(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const title = args.chapterDraft.title.trim();
    const orderText = args.chapterDraft.order.trim();
    const order = Number(orderText);
    if (!token || !title || !orderText || !Number.isFinite(order) || order < 0) {
      args.setActionMessage("Chapter title and a non-negative order are required.");
      return;
    }

    args.setActionState("saving");
    args.setActionMessage(null);
    try {
      await createTeacherChapter({ courseId: args.courseId, payload: { order, title }, token });
      args.setChapterDraft(defaultChapterDraft);
      args.setActionMessage("Chapter created.");
      await args.loadContentRoute();
    } catch (nextError) {
      args.setActionMessage(normalizeRouteError(nextError).message);
    } finally {
      args.setActionState("idle");
    }
  };
}
