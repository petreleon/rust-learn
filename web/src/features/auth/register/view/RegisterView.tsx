import { KeyRound, Mail, UserPlus } from "lucide-react";
import { type Dispatch, type FormEventHandler, type SetStateAction } from "react";
import { AuthHeader } from "../../shared/components/AuthHeader";
import { AuthHero } from "../../shared/components/AuthHero";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import { type PasswordPolicyResult } from "../../shared/model/passwordPolicy";
import styles from "../../shared/auth.module.css";
import { RegisterFormBody } from "../components/RegisterFormBody";
import { RegistrationSuccessPanel } from "../components/RegistrationSuccessPanel";
import { type RegisterSubmitState } from "../model/RegisterSubmitState";

type RegisterViewProps = {
  canSubmit: boolean;
  dateOfBirth: string;
  email: string;
  error: AuthFormError | null;
  name: string;
  password: string;
  policyResults: PasswordPolicyResult[];
  setDateOfBirth: (value: string) => void;
  setEmail: (value: string) => void;
  setName: (value: string) => void;
  setPassword: (value: string) => void;
  setShowPassword: Dispatch<SetStateAction<boolean>>;
  showPassword: boolean;
  submitRegistration: FormEventHandler<HTMLFormElement>;
  submitState: RegisterSubmitState;
};

export function RegisterView(props: RegisterViewProps) {
  return (
    <main className={styles.page}>
      <AuthHeader
        navLabel="Registration links"
        links={[
          { href: "/login", label: "Login" },
          { href: "/session", label: "Session" },
        ]}
      />
      <section className={styles.layout}>
        <AuthHero
          eyebrow="Register"
          title="Create your RustLearn account"
          description="Start with learner access, verify your email, then grow into teacher, organization, or platform scopes as permissions are granted."
          statusItems={[
            { icon: <Mail size={16} aria-hidden />, label: "Verification email required" },
            { icon: <KeyRound size={16} aria-hidden />, label: "Strong password policy" },
          ]}
        />
        <form className={styles.panel} onSubmit={props.submitRegistration}>
          <div className={styles.panelHeader}>
            <UserPlus size={22} aria-hidden />
            <h2>Register</h2>
          </div>
          {props.submitState === "success" ? (
            <RegistrationSuccessPanel />
          ) : (
            <RegisterFormBody {...props} />
          )}
        </form>
      </section>
    </main>
  );
}
