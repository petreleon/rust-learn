import { AlertCircle } from "lucide-react";
import { type AuthFormError } from "../model/AuthFormError";
import styles from "../auth.module.css";

type AuthErrorBoxProps = {
  error: AuthFormError;
};

export function AuthErrorBox({ error }: AuthErrorBoxProps) {
  return (
    <div className={styles.errorBox} role="status">
      <AlertCircle size={18} aria-hidden />
      <span>
        <strong>{error.code}</strong>
        {error.message}
      </span>
    </div>
  );
}
