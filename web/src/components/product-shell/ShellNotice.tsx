"use client";
export type ShellNotice = {
  actionHref?: string;
  actionLabel?: string;
  message: string;
  title: string;
  tone: "info" | "success" | "warn" | "error";
};
