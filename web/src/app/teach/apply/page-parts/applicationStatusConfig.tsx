"use client";

import { AlertCircle, CheckCircle2, Clock3, FileText, XCircle } from "lucide-react";
import { type ReactNode } from "react";
import { type TeacherApplication } from "@/lib/teacher";

export function applicationStatusConfig(application: TeacherApplication | null): {
  detail: string;
  heading: string;
  icon: ReactNode;
  label: string;
  tone: "good" | "neutral" | "warn" | "bad";
} {
  if (!application) {
    return {
      detail: "No teacher application is on file yet. Submit one when you are ready for platform review.",
      heading: "Ready to apply",
      icon: <FileText size={20} aria-hidden />,
      label: "No application",
      tone: "neutral",
    };
  }

  switch (application.status) {
    case "approved":
      return {
        detail: "Your application has been approved. Teaching permissions are assigned from the approved scope.",
        heading: "Application approved",
        icon: <CheckCircle2 size={20} aria-hidden />,
        label: "Approved",
        tone: "good",
      };
    case "needs_changes":
      return {
        detail: "A reviewer asked for changes. This page shows the reason and blocks fake resubmission until the edit contract exists.",
        heading: "Changes requested",
        icon: <AlertCircle size={20} aria-hidden />,
        label: "Needs changes",
        tone: "warn",
      };
    case "rejected":
      return {
        detail: "The latest application was rejected. You can start a fresh application with a stronger summary or portfolio.",
        heading: "Application rejected",
        icon: <XCircle size={20} aria-hidden />,
        label: "Rejected",
        tone: "bad",
      };
    case "submitted":
    default:
      return {
        detail: "Your application is in the review queue. Duplicate submissions are blocked so reviewer state stays clear.",
        heading: "Application submitted",
        icon: <Clock3 size={20} aria-hidden />,
        label: "Submitted",
        tone: "neutral",
      };
  }
}
