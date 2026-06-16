import Link from "next/link";
import styles from "../auth.module.css";

export type AuthNavLink = {
  href: string;
  label: string;
};

type AuthHeaderProps = {
  navLabel: string;
  links: AuthNavLink[];
  productLabel?: string;
};

export function AuthHeader({ links, navLabel, productLabel = "Workspace" }: AuthHeaderProps) {
  return (
    <header className={styles.topbar}>
      <Link className={styles.brand} href="/">
        <span className={styles.brandMark}>RL</span>
        <span>
          <strong>RustLearn</strong>
          <small>{productLabel}</small>
        </span>
      </Link>
      <nav className={styles.topActions} aria-label={navLabel}>
        {links.map((link) => (
          <Link className={styles.navLink} href={link.href} key={link.href}>
            {link.label}
          </Link>
        ))}
      </nav>
    </header>
  );
}
