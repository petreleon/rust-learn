"use client";

import { type TeacherApplicationScope } from "@/lib/teacher";
import styles from "../page.module.css";

export function ScopeOption({
  checked,
  detail,
  disabled = false,
  label,
  name,
  onChange,
  value,
}: {
  checked: boolean;
  detail: string;
  disabled?: boolean;
  label: string;
  name: string;
  onChange: () => void;
  value: TeacherApplicationScope;
}) {
  return (
    <label className={`${styles.scopeOption} ${checked ? styles.scopeOptionActive : ""} ${disabled ? styles.scopeOptionDisabled : ""}`}>
      <input checked={checked} disabled={disabled} name={name} onChange={onChange} type="radio" value={value} />
      <strong>{label}</strong>
      <span>{detail}</span>
    </label>
  );
}
