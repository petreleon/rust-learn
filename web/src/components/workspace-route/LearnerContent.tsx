"use client";

import { BookOpen, BriefcaseBusiness, CreditCard, ShieldCheck, Trophy } from "lucide-react";
import Link from "next/link";
import { sessionScopePermissionEnabled, type CurrentSession } from "@/lib/session";
import styles from "../workspace-route.module.css";
import { ScopeList } from "./ScopeList";
import { SummaryCard } from "./SummaryCard";

export function LearnerContent({ session }: { session: CurrentSession }) {
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Courses" value={session.courses.length} />
        <SummaryCard
          icon={<ShieldCheck size={20} aria-hidden />}
          label="Reward scopes"
          value={session.courses.filter((course) => sessionScopePermissionEnabled(course, "VIEW_COURSE_REWARD_STATUS")).length}
        />
        <SummaryCard icon={<BriefcaseBusiness size={20} aria-hidden />} label="Organizations" value={session.organizations.length} />
      </section>
      <section className={styles.statePanel}>
        <div className={styles.panelHeader}>
          <BookOpen size={20} aria-hidden />
          <h2>Continue as a learner</h2>
        </div>
        <p className={styles.muted}>
          Open course access, reward history, or wallet readiness without leaving the product
          workspace.
        </p>
        <div className={styles.actionRow}>
          <Link className={styles.primaryLink} href="/courses">
            <BookOpen size={18} aria-hidden />
            Courses
          </Link>
          <Link className={styles.secondaryLink} href="/rewards">
            <Trophy size={18} aria-hidden />
            Rewards
          </Link>
          <Link className={styles.secondaryLink} href="/wallet">
            <CreditCard size={18} aria-hidden />
            Wallet
          </Link>
        </div>
      </section>
      <ScopeList
        empty="Course enrollments and learning scopes will appear here."
        scopes={session.courses}
        title="Courses"
      />
    </>
  );
}
