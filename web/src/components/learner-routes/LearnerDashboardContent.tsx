"use client";

import { BookOpen, CheckCircle, CreditCard, RefreshCw, Trophy } from "lucide-react";
import Link from "next/link";
import { useMemo } from "react";
import { type LearnerDashboardSnapshot } from "@/lib/learner";
import { type CurrentSession } from "@/lib/session";
import styles from "../learner-routes.module.css";
import { DashboardCourseCard } from "./DashboardCourseCard";
import { DashboardRewardPanel } from "./DashboardRewardPanel";
import { DashboardWalletPanel } from "./DashboardWalletPanel";
import { EmptyState } from "./EmptyState";
import { StatusPill } from "./StatusPill";
import { SummaryCard } from "./SummaryCard";
import { courseContentLabel } from "./courseContentLabel";
import { courseOrganizationLabel } from "./courseOrganizationLabel";
import { summarizeRewards } from "./summarizeRewards";

export function LearnerDashboardContent({
  dashboard,
  onRefresh,
  session,
}: {
  dashboard: LearnerDashboardSnapshot;
  onRefresh: () => void;
  session: CurrentSession;
}) {
  const enrolledCourses = dashboard.enrolled_catalog.courses;
  const recommendedCourses = dashboard.recommended_catalog.courses;
  const continueCourse = enrolledCourses.find((course) => course.access.can_view_content && course.content.has_content) || null;
  const rewardSummary = useMemo(() => summarizeRewards(dashboard.reward_history), [dashboard.reward_history]);
  const progressLabel = continueCourse ? "Not tracked" : "No lesson";

  return (
    <>
      <section className={styles.dashboardGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Enrolled courses" value={dashboard.enrolled_catalog.total} />
        <SummaryCard icon={<CheckCircle size={20} aria-hidden />} label="Progress" value={progressLabel} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Recent rewards" value={dashboard.reward_history.length} />
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet" value={dashboard.wallet ? "Linked" : "Unlinked"} />
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Next step</h2>
          <button className={styles.iconAction} aria-label="Refresh learner dashboard" type="button" onClick={onRefresh}>
            <RefreshCw size={18} aria-hidden />
          </button>
        </div>
        {continueCourse ? (
          <article className={styles.itemCard}>
            <div className={styles.itemHeader}>
              <h3>{continueCourse.title}</h3>
              <StatusPill label="Continue" tone="good" />
            </div>
            <p className={styles.muted}>
              Progress tracking is not available yet. You can still open the next available lesson from the course outline.
            </p>
            <div className={styles.metaRow}>
              <span>{courseOrganizationLabel(continueCourse)}</span>
              <span>{courseContentLabel(continueCourse)}</span>
              <span>{continueCourse.rewards.available ? "Rewards available" : "No active rewards"}</span>
            </div>
            <div className={styles.actionRow}>
              <Link className={styles.primaryLink} href={`/courses/${continueCourse.id}/learn`}>
                <BookOpen size={18} aria-hidden />
                Continue learning
              </Link>
              <Link className={styles.secondaryLink} href={`/courses/${continueCourse.id}`}>
                Course details
              </Link>
            </div>
          </article>
        ) : (
          <EmptyState
            actionHref="/courses"
            actionLabel={enrolledCourses.length ? "Open courses" : "Find courses"}
            detail={
              enrolledCourses.length
                ? "Your current courses do not have viewable content yet."
                : "Choose a course from the catalog to start building your learner workspace."
            }
            title={enrolledCourses.length ? "No lesson ready" : "Start with a course"}
          />
        )}
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Enrolled courses</h2>
          <StatusPill label={`${enrolledCourses.length} shown`} tone="neutral" />
        </div>
        {enrolledCourses.length ? (
          <div className={styles.itemGrid}>
            {enrolledCourses.map((course) => (
              <DashboardCourseCard course={course} key={course.id} />
            ))}
          </div>
        ) : (
          <EmptyState
            actionHref="/courses"
            actionLabel="Browse catalog"
            detail="Enrolled courses appear here after course staff approve access or assign you directly."
            title="No enrolled courses"
          />
        )}
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Rewards and wallet</h2>
          <StatusPill label={rewardSummary.needsHelp ? "Needs attention" : "Ready"} tone={rewardSummary.needsHelp ? "warn" : "neutral"} />
        </div>
        <div className={styles.itemGrid}>
          <DashboardRewardPanel rewardCount={dashboard.reward_history.length} summary={rewardSummary} />
          <DashboardWalletPanel wallet={dashboard.wallet} />
        </div>
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Recommended courses</h2>
          <StatusPill label={`${recommendedCourses.length} available`} tone="neutral" />
        </div>
        {recommendedCourses.length ? (
          <div className={styles.itemGrid}>
            {recommendedCourses.map((course) => (
              <DashboardCourseCard course={course} key={course.id} />
            ))}
          </div>
        ) : (
          <EmptyState
            detail={
              session.courses.length
                ? "No additional available courses are visible right now."
                : "Published courses will appear here when they are visible to learners."
            }
            title="No recommendations yet"
          />
        )}
      </section>
    </>
  );
}
