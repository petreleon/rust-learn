"use client";

import { type ReactNode } from "react";
import styles from "../page.module.css";

export function SectionTitle({ icon, title }: { icon: ReactNode; title: string }) {
  return (
    <div className={styles.sectionTitle}>
      {icon}
      <h2>{title}</h2>
    </div>
  );
}
