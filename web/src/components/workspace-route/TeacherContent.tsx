"use client";

import { BookOpen, BriefcaseBusiness, Send, ShieldCheck } from "lucide-react";
import Link from "next/link";
import { countDelegatedPermissions, hasTeacherApplicationAccess, hasTeacherCourseAccess } from "@/lib/access";
import { type CurrentSession } from "@/lib/session";
import styles from "../workspace-route.module.css";
import { ScopeList } from "./ScopeList";
import { SummaryCard } from "./SummaryCard";

export function TeacherContent({ session }: { session: CurrentSession }) {
  const teacherCourses = session.courses.filter(hasTeacherCourseAccess);
  const canApply = hasTeacherApplicationAccess(session);
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Teaching courses" value={teacherCourses.length} />
        <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Application access" value={canApply ? "Available" : "Not needed"} />
        <SummaryCard icon={<BriefcaseBusiness size={20} aria-hidden />} label="Delegations" value={countDelegatedPermissions(session)} />
      </section>
      <ScopeList
        empty="Approved course teaching scopes will appear here."
        scopes={teacherCourses}
        title="Teaching scopes"
      />
      {canApply ? (
        <section className={styles.statePanel}>
          <div className={styles.panelHeader}>
            <Send size={20} aria-hidden />
            <h2>Teacher application</h2>
          </div>
          <p className={styles.muted}>
            Apply for teaching access or track your submitted application without using the operations console.
          </p>
          <Link className={styles.primaryLink} href="/teach/apply">
            <Send size={18} aria-hidden />
            Open application
          </Link>
        </section>
      ) : null}
    </>
  );
}
