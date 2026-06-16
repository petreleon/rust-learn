"use client";

import { AlertCircle, Clock3, Loader2, LogIn, RefreshCw, Trophy } from "lucide-react";
import Link from "next/link";
import { ProductShell } from "@/components/product-shell";
import { StatePanel } from "@/components/teacher-routes/StatePanel";
import { StatusLine } from "@/components/teacher-routes/StatusLine";
import { routeNotice } from "@/components/teacher-routes/routeNotice";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { RewardReviewContent } from "../components/RewardReviewContent";
import { type TeacherCourseRewardsRouteController } from "../route/useTeacherCourseRewardsRoute";

export function TeacherCourseRewardsView({
  courseId,
  route,
}: {
  courseId: string;
  route: TeacherCourseRewardsRouteController;
}) {
  const courseTitle = route.students?.course.title || "Reward review";

  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={courseRewardsBreadcrumbs(courseId, courseTitle)}
      description="Review course-scoped reward evidence and apply teacher decisions without platform payout controls."
      eyebrow="Teacher rewards"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={routeNotice(route.error)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<CourseRewardStatusItems route={route} />}
      title={courseTitle}
    >
      <CourseRewardsBody courseId={courseId} route={route} />
    </ProductShell>
  );
}

function CourseRewardsBody({
  courseId,
  route,
}: {
  courseId: string;
  route: TeacherCourseRewardsRouteController;
}) {
  if (route.loadState === "loading") return <LoadingRewardReview />;
  if (route.loadState === "idle") return <SignedOutRewardReview courseId={courseId} />;
  if (route.loadState === "error" && route.error) return <RewardRouteError courseId={courseId} route={route} />;
  if (route.loadState === "success" && route.students) {
    return (
      <RewardReviewContent
        actionMessage={route.decision.actionMessage}
        actionState={route.decision.actionState}
        candidates={route.candidates}
        decisionDrafts={route.decision.decisionDrafts}
        onDecisionDraftChange={route.decision.updateDecisionDraft}
        onRefresh={() => void route.loadRewardsRoute()}
        onStatusFilterChange={route.setStatusFilter}
        onSubmitDecision={route.decision.submitRewardDecision}
        statusFilter={route.statusFilter}
        students={route.students}
      />
    );
  }
  return null;
}

function CourseRewardStatusItems({ route }: { route: TeacherCourseRewardsRouteController }) {
  if (!route.students) return null;

  const pendingCount = route.students.course.reward_queue.pending_teacher_count;

  return (
    <>
      <StatusLine
        icon={<Clock3 size={16} aria-hidden />}
        label={`${pendingCount} pending`}
        tone={pendingCount ? "warn" : "neutral"}
      />
      <StatusLine
        icon={<Trophy size={16} aria-hidden />}
        label={`${route.candidates.length} shown`}
        tone={route.candidates.length ? "good" : "neutral"}
      />
    </>
  );
}

function LoadingRewardReview() {
  return (
    <StatePanel
      detail="Loading reward candidates, learner context, and course-scoped permissions."
      icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
      title="Loading reward review"
    />
  );
}

function SignedOutRewardReview({ courseId }: { courseId: string }) {
  return (
    <StatePanel
      action={
        <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/rewards`}>
          <LogIn size={18} aria-hidden />
          Sign in
        </Link>
      }
      detail="Reward review loads from your signed-in teaching session."
      icon={<LogIn size={22} aria-hidden />}
      title="Sign in required"
    />
  );
}

function RewardRouteError({
  courseId,
  route,
}: {
  courseId: string;
  route: TeacherCourseRewardsRouteController;
}) {
  return (
    <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
      <AlertCircle size={18} aria-hidden />
      <span>
        <strong>{route.error?.code}</strong>
        {route.error?.message}
      </span>
      {route.students ? (
        <Link className={styles.secondaryLink} href={`/teach/courses/${courseId}`}>
          Course workspace
        </Link>
      ) : null}
      <button className={styles.secondaryButton} type="button" onClick={() => route.loadRewardsRoute()}>
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}

function courseRewardsBreadcrumbs(courseId: string, courseTitle: string) {
  return [
    { href: "/session", label: "Workspace" },
    { href: "/teach", label: "Teach" },
    { href: "/teach/courses", label: "Courses" },
    { href: `/teach/courses/${courseId}`, label: courseTitle },
    { label: "Rewards" },
  ];
}
