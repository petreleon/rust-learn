import { type ReactNode } from "react";
import styles from "../auth.module.css";

export type AuthHeroStatusItem = {
  icon: ReactNode;
  label: string;
};

type AuthHeroProps = {
  eyebrow: string;
  title: string;
  description: string;
  statusItems: AuthHeroStatusItem[];
};

export function AuthHero({ description, eyebrow, statusItems, title }: AuthHeroProps) {
  return (
    <div className={styles.hero}>
      <p className={styles.eyebrow}>{eyebrow}</p>
      <h1>{title}</h1>
      <p>{description}</p>
      <div className={styles.statusList}>
        {statusItems.map((item) => (
          <span key={item.label}>
            {item.icon}
            {item.label}
          </span>
        ))}
      </div>
    </div>
  );
}
