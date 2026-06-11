"use client";

import { AlertCircle, CheckCircle2, Clock3, FileText, Send } from "lucide-react";
import { type TeacherApplication } from "@/lib/teacher";

export function applicationConfig(application: TeacherApplication | null, canSubmitApplication: boolean) {
  if (!application) {
    return {
      action: "Apply to teach",
      detail: canSubmitApplication
        ? "No teacher application is currently on file for this account."
        : "No teacher application is visible for this account.",
      href: canSubmitApplication ? "/teach/apply" : null,
      icon: <Send size={20} aria-hidden />,
      title: "Teacher application",
    };
  }

  if (application.status === "approved") {
    return {
      action: "View application",
      detail: "Your teacher application is approved. Course-scoped teaching work appears in this dashboard.",
      href: "/teach/apply",
      icon: <CheckCircle2 size={20} aria-hidden />,
      title: "Application approved",
    };
  }

  if (application.status === "rejected") {
    return {
      action: canSubmitApplication ? "Start again" : "View application",
      detail: "The latest teacher application was rejected. Review the decision before starting a fresh application.",
      href: "/teach/apply",
      icon: <AlertCircle size={20} aria-hidden />,
      title: "Application rejected",
    };
  }

  if (application.status === "needs_changes") {
    return {
      action: "View feedback",
      detail: "A reviewer requested changes. The application route shows the reason and current resubmission limits.",
      href: "/teach/apply",
      icon: <FileText size={20} aria-hidden />,
      title: "Changes requested",
    };
  }

  return {
    action: "Track review",
    detail: "Your application is in review. Teaching courses will appear here after scope approval.",
    href: "/teach/apply",
    icon: <Clock3 size={20} aria-hidden />,
    title: "Application submitted",
  };
}
