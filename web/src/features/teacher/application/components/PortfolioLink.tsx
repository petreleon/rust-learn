"use client";

import { ExternalLink } from "lucide-react";
import styles from "@/features/teacher/application/page.module.css";
import { isSafeHttpUrl } from "../model/isSafeHttpUrl";

export function PortfolioLink({ link }: { link: string }) {
  if (!isSafeHttpUrl(link)) {
    return <span className={styles.portfolioText}>{link}</span>;
  }

  return (
    <a className={styles.portfolioLink} href={link} rel="noreferrer" target="_blank">
      <ExternalLink size={15} aria-hidden />
      {link}
    </a>
  );
}
