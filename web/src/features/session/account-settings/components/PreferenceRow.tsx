"use client";

import styles from "@/features/session/account-settings/page.module.css";

export function PreferenceRow({
  checked,
  detail,
  disabled,
  label,
  onChange,
}: {
  checked: boolean;
  detail: string;
  disabled?: boolean;
  label: string;
  onChange?: (checked: boolean) => void;
}) {
  return (
    <label className={styles.preferenceRow}>
      <input
        checked={checked}
        disabled={disabled || !onChange}
        onChange={onChange ? (event) => onChange(event.target.checked) : undefined}
        readOnly={!onChange}
        type="checkbox"
      />
      <span>
        <strong>{label}</strong>
        <small>{detail}</small>
      </span>
    </label>
  );
}
