include!("authentication_flow/01_imports.rs");
include!("authentication_flow/02_login_requires_email_verification_and_returns_jwt_after_verification.rs");
include!("authentication_flow/03_verify_email_marks_user_verified_and_reports_already_verified_replay.rs");
include!("authentication_flow/04_verify_email_reports_expired_and_invalid_tokens.rs");
include!("authentication_flow/05_register_rejects_duplicate_email_without_panicking.rs");
