import {
  type TeacherAssessment,
  type TeacherAssessmentPayload,
  type TeacherAssessmentQuestion,
} from "@/lib/teacher";

export type AssessmentDraft = {
  title: string;
  description: string;
  passingScore: string;
  maxAttempts: string;
  published: boolean;
  questions: AssessmentQuestionDraft[];
};

export type AssessmentQuestionDraft = {
  text: string;
  optionsText: string;
  correctAnswer: string;
  points: string;
};

export const defaultAssessmentDraft: AssessmentDraft = {
  title: "",
  description: "",
  passingScore: "70",
  maxAttempts: "3",
  published: false,
  questions: [defaultAssessmentQuestionDraft()],
};

export function defaultAssessmentQuestionDraft(): AssessmentQuestionDraft {
  return {
    text: "",
    optionsText: "",
    correctAnswer: "",
    points: "1",
  };
}

export function draftFromAssessment(assessment: TeacherAssessment): AssessmentDraft {
  return {
    title: assessment.title,
    description: assessment.description ?? "",
    passingScore: String(assessment.passing_score),
    maxAttempts: String(assessment.max_attempts),
    published: assessment.published,
    questions: assessment.questions.length
      ? assessment.questions.map(questionDraftFromAssessment)
      : [defaultAssessmentQuestionDraft()],
  };
}

function questionDraftFromAssessment(question: TeacherAssessmentQuestion): AssessmentQuestionDraft {
  return {
    text: question.text,
    optionsText: optionsText(question),
    correctAnswer: question.correct_answer ?? "",
    points: String(question.points),
  };
}

export function assessmentPayloadFromDraft(draft: AssessmentDraft): TeacherAssessmentPayload {
  const questions = draft.questions
    .filter((question) => question.text.trim())
    .map((question, index) => ({
      text: question.text.trim(),
      question_type: "multiple_choice",
      options: parseOptions(question.optionsText),
      correct_answer: question.correctAnswer.trim() || null,
      points: Number.parseInt(question.points, 10),
      order: index,
    }));
  return {
    title: draft.title.trim(),
    description: draft.description.trim() || null,
    passing_score: Number.parseInt(draft.passingScore, 10),
    max_attempts: Number.parseInt(draft.maxAttempts, 10),
    published: draft.published,
    questions,
  };
}

export function isAssessmentDraftDirty(draft: AssessmentDraft, editingId: number | null) {
  return (
    editingId !== null ||
    draft.title.trim() !== "" ||
    draft.description.trim() !== "" ||
    draft.questions.some(isQuestionDraftDirty)
  );
}

function isQuestionDraftDirty(question: AssessmentQuestionDraft) {
  return (
    question.text.trim() !== "" ||
    question.optionsText.trim() !== "" ||
    question.correctAnswer.trim() !== ""
  );
}

function parseOptions(options: string) {
  const parsed = options
    .split(",")
    .map((option) => option.trim())
    .filter(Boolean);
  return parsed.length ? parsed : null;
}

function optionsText(question: TeacherAssessmentQuestion) {
  return Array.isArray(question.options) ? question.options.join(", ") : "";
}
