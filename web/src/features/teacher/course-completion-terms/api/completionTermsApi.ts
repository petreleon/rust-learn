import { DEFAULT_TIMEOUT_MS } from "@/lib/teacher/DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "@/lib/teacher/teacherJsonRequest";
import { type CourseCompletionTermsHistory, type CourseCompletionTerms } from "../model/CourseCompletionTerms";

export function loadCourseCompletionTerms({
  courseId,
  token,
}: {
  courseId: number | string;
  token: string;
}): Promise<CourseCompletionTermsHistory> {
  return teacherJsonRequest<CourseCompletionTermsHistory>({
    method: "GET",
    timeoutMs: DEFAULT_TIMEOUT_MS,
    token,
    url: `/api/courses/teaching/${courseId}/completion-terms`,
  });
}

export function submitCourseCompletionTerms({
  courseId,
  payload,
  token,
}: {
  courseId: number | string;
  payload: CourseCompletionTermsPayload;
  token: string;
}): Promise<CourseCompletionTerms> {
  return writeTerms({ courseId, payload, token, urlSuffix: "proposals", method: "POST" });
}

export function counterCourseCompletionTerms({
  courseId,
  payload,
  termsId,
  token,
}: CourseCompletionTermsActionOptions<CourseCompletionTermsPayload>): Promise<CourseCompletionTerms> {
  return writeTerms({
    courseId,
    payload,
    token,
    urlSuffix: `${termsId}/counter`,
    method: "PUT",
  });
}

export function acceptCourseCompletionTerms({
  courseId,
  payload,
  termsId,
  token,
}: CourseCompletionTermsActionOptions<CourseCompletionTermsDecisionPayload>): Promise<CourseCompletionTerms> {
  return writeTerms({
    courseId,
    payload,
    token,
    urlSuffix: `${termsId}/accept`,
    method: "PUT",
  });
}

export function rejectCourseCompletionTerms({
  courseId,
  payload,
  termsId,
  token,
}: CourseCompletionTermsActionOptions<CourseCompletionTermsDecisionPayload>): Promise<CourseCompletionTerms> {
  return writeTerms({
    courseId,
    payload,
    token,
    urlSuffix: `${termsId}/reject`,
    method: "PUT",
  });
}

export function withdrawCourseCompletionTerms({
  courseId,
  payload,
  termsId,
  token,
}: CourseCompletionTermsActionOptions<CourseCompletionTermsDecisionPayload>): Promise<CourseCompletionTerms> {
  return writeTerms({
    courseId,
    payload,
    token,
    urlSuffix: `${termsId}/withdraw`,
    method: "PUT",
  });
}

type CourseCompletionTermsPayload = {
  completion_reward_amount: string;
  max_enrolled_students: number;
  note: string | null;
};

type CourseCompletionTermsDecisionPayload = {
  note: string | null;
};

type CourseCompletionTermsActionOptions<TPayload> = {
  courseId: number | string;
  payload: TPayload;
  termsId: number;
  token: string;
};

function writeTerms<TPayload>({
  courseId,
  method,
  payload,
  token,
  urlSuffix,
}: {
  courseId: number | string;
  method: string;
  payload: TPayload;
  token: string;
  urlSuffix: string;
}) {
  return teacherJsonRequest<CourseCompletionTerms>({
    body: JSON.stringify(payload),
    method,
    timeoutMs: DEFAULT_TIMEOUT_MS,
    token,
    url: `/api/courses/teaching/${courseId}/completion-terms/${urlSuffix}`,
  });
}
