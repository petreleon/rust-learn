// @generated automatically by Diesel CLI.

diesel::table! {
    authentications (id) {
        id -> Int4,
        user_id -> Int4,
        type_authentication -> Varchar,
        info_auth -> Nullable<Text>,
    }
}

diesel::table! {
    chapters (id) {
        id -> Int4,
        course_id -> Int4,
        title -> Varchar,
        order -> Int4,
    }
}

diesel::table! {
    contents (id) {
        id -> Int4,
        chapter_id -> Int4,
        order -> Int4,
        content_type -> Varchar,
        data -> Nullable<Text>,
    }
}

diesel::table! {
    course_join_requests (id) {
        id -> Int8,
        course_id -> Int4,
        requester_user_id -> Int4,
        #[max_length = 32]
        status -> Varchar,
        reviewer_user_id -> Nullable<Int4>,
        decision_reason -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        decided_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    course_progress (id) {
        id -> Int4,
        user_id -> Int4,
        course_id -> Int4,
        content_id -> Int4,
        viewed_at -> Timestamptz,
    }
}

diesel::table! {
    course_roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    courses (id) {
        id -> Int4,
        title -> Varchar,
        #[max_length = 32]
        lifecycle_status -> Varchar,
    }
}

diesel::table! {
    courses_organizations (id) {
        course_id -> Int4,
        organization_id -> Int4,
        order -> Int4,
        id -> Int4,
    }
}

diesel::table! {
    db_version_control (id) {
        id -> Int4,
        version -> Int4,
    }
}

diesel::table! {
    delegated_permissions (id) {
        id -> Int8,
        grantor_user_id -> Int4,
        grantee_user_id -> Int4,
        #[max_length = 128]
        permission -> Varchar,
        #[max_length = 32]
        scope_type -> Varchar,
        organization_id -> Nullable<Int4>,
        course_id -> Nullable<Int4>,
        reason -> Nullable<Text>,
        expires_at -> Nullable<Timestamptz>,
        revoked_at -> Nullable<Timestamptz>,
        revoked_by_user_id -> Nullable<Int4>,
        revoke_reason -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    email_verification_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token_hash -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        used_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    external_transactions (id) {
        id -> Int8,
        amount -> Numeric,
        blockchain_address -> Text,
        chain_id -> Nullable<Int8>,
        contract_address -> Nullable<Text>,
        transaction_hash -> Nullable<Text>,
        log_index -> Nullable<Int8>,
        #[max_length = 50]
        event_type -> Nullable<Varchar>,
        from_address -> Nullable<Text>,
        to_address -> Nullable<Text>,
    }
}

diesel::table! {
    internal_transactions (id) {
        id -> Int8,
        wallet_id -> Int4,
        amount -> Numeric,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int8,
        user_id -> Nullable<Int4>,
        title -> Text,
        body -> Text,
        created_at -> Timestamptz,
        read -> Bool,
    }
}

diesel::table! {
    organization_roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        name -> Varchar,
        website_link -> Nullable<Varchar>,
        profile_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    paths (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    paths_courses (path_id, course_id) {
        path_id -> Int4,
        course_id -> Int4,
        order -> Int4,
    }
}

diesel::table! {
    pending_course_organization_invites (id) {
        id -> Int4,
        course_id -> Int4,
        organization_id -> Int4,
        order -> Int4,
    }
}

diesel::table! {
    persistent_states (id) {
        id -> Int4,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    platform_roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    reward_audit_events (id) {
        id -> Int8,
        reward_candidate_id -> Int8,
        actor_user_id -> Nullable<Int4>,
        #[max_length = 64]
        event_type -> Varchar,
        #[max_length = 32]
        from_status -> Nullable<Varchar>,
        #[max_length = 32]
        to_status -> Varchar,
        reason -> Nullable<Text>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    reward_candidates (id) {
        id -> Int8,
        course_id -> Int4,
        student_user_id -> Int4,
        submitter_user_id -> Int4,
        #[max_length = 32]
        source_scope -> Varchar,
        source_organization_id -> Nullable<Int4>,
        #[max_length = 64]
        event_type -> Varchar,
        idempotency_key -> Text,
        evidence -> Jsonb,
        #[max_length = 32]
        status -> Varchar,
        teacher_approver_user_id -> Nullable<Int4>,
        teacher_decision_reason -> Nullable<Text>,
        teacher_decided_at -> Nullable<Timestamptz>,
        amount_reviewer_user_id -> Nullable<Int4>,
        approved_amount -> Nullable<Numeric>,
        amount_decision_reason -> Nullable<Text>,
        amount_decided_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    reward_compensation_records (id) {
        id -> Int8,
        reward_candidate_id -> Int8,
        wallet_id -> Int4,
        transaction_id -> Int8,
        internal_transaction_id -> Int8,
        amount -> Numeric,
        reason -> Text,
        idempotency_key -> Text,
        created_by_user_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    reward_execution_jobs (id) {
        id -> Int8,
        reward_candidate_id -> Int8,
        #[max_length = 32]
        status -> Varchar,
        attempts -> Int4,
        run_after -> Timestamptz,
        last_error -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    reward_fraud_blocks (id) {
        id -> Int8,
        #[max_length = 32]
        scope_type -> Varchar,
        teacher_user_id -> Nullable<Int4>,
        organization_id -> Nullable<Int4>,
        course_id -> Nullable<Int4>,
        reward_policy_id -> Nullable<Int8>,
        reason -> Text,
        evidence_reference -> Nullable<Text>,
        created_by_user_id -> Int4,
        expires_at -> Nullable<Timestamptz>,
        revoked_by_user_id -> Nullable<Int4>,
        revoked_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    reward_payout_records (id) {
        id -> Int8,
        reward_candidate_id -> Int8,
        transaction_id -> Int8,
        external_transaction_id -> Int8,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    reward_policies (id) {
        id -> Int8,
        #[max_length = 32]
        scope_type -> Varchar,
        organization_id -> Nullable<Int4>,
        course_id -> Nullable<Int4>,
        #[max_length = 64]
        event_type -> Varchar,
        version -> Int4,
        token_amount -> Numeric,
        multiplier -> Numeric,
        max_payout -> Nullable<Numeric>,
        cooldown_seconds -> Int8,
        #[max_length = 32]
        payment_strategy -> Varchar,
        active -> Bool,
        created_by_user_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    reward_wallet_credit_records (id) {
        id -> Int8,
        reward_candidate_id -> Int8,
        wallet_id -> Int4,
        transaction_id -> Int8,
        internal_transaction_id -> Int8,
        notification_id -> Nullable<Int8>,
        notified_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    role_course_hierarchy (id) {
        id -> Int4,
        course_role_id -> Nullable<Int4>,
        hierarchy_level -> Int4,
    }
}

diesel::table! {
    role_organization_hierarchy (id) {
        id -> Int4,
        organization_role_id -> Nullable<Int4>,
        hierarchy_level -> Int4,
    }
}

diesel::table! {
    role_permission_course (id) {
        id -> Int4,
        course_id -> Nullable<Int4>,
        course_role_id -> Nullable<Int4>,
        permission -> Varchar,
    }
}

diesel::table! {
    role_permission_organization (id) {
        id -> Int4,
        organization_id -> Nullable<Int4>,
        organization_role_id -> Nullable<Int4>,
        permission -> Varchar,
    }
}

diesel::table! {
    role_permission_platform (id) {
        id -> Int4,
        platform_role_id -> Nullable<Int4>,
        permission -> Varchar,
    }
}

diesel::table! {
    role_platform_hierarchy (id) {
        id -> Int4,
        platform_role_id -> Nullable<Int4>,
        hierarchy_level -> Int4,
    }
}

diesel::table! {
    teacher_application_audit_events (id) {
        id -> Int8,
        application_id -> Int8,
        actor_user_id -> Nullable<Int4>,
        #[max_length = 64]
        event_type -> Varchar,
        #[max_length = 32]
        from_status -> Nullable<Varchar>,
        #[max_length = 32]
        to_status -> Varchar,
        reason -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    teacher_applications (id) {
        id -> Int8,
        applicant_user_id -> Int4,
        #[max_length = 32]
        requested_scope -> Varchar,
        requested_organization_id -> Nullable<Int4>,
        requested_course_id -> Nullable<Int4>,
        experience_summary -> Text,
        organization_sponsor_id -> Nullable<Int4>,
        portfolio_links -> Jsonb,
        #[max_length = 32]
        status -> Varchar,
        reviewer_id -> Nullable<Int4>,
        decision_reason -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        decided_at -> Nullable<Timestamptz>,
        idempotency_key -> Nullable<Text>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int8,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    transactions_external_transactions (transaction_id, external_transaction_id) {
        transaction_id -> Int8,
        external_transaction_id -> Int8,
    }
}

diesel::table! {
    transactions_internal_transactions (transaction_id, internal_transaction_id) {
        transaction_id -> Int8,
        internal_transaction_id -> Int8,
    }
}

diesel::table! {
    upload_jobs (id) {
        id -> Int8,
        bucket -> Varchar,
        object -> Text,
        user_id -> Nullable<Int4>,
        status -> Varchar,
        attempts -> Int4,
        last_error -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_role_course (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        course_role_id -> Nullable<Int4>,
        course_id -> Nullable<Int4>,
    }
}

diesel::table! {
    user_role_organization (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        organization_role_id -> Nullable<Int4>,
        organization_id -> Nullable<Int4>,
    }
}

diesel::table! {
    user_role_platform (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        platform_role_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        date_of_birth -> Nullable<Date>,
        created_at -> Timestamp,
        kyc_verified -> Bool,
        email_verified -> Bool,
    }
}

diesel::table! {
    wallet_token_deposit_intents (id) {
        id -> Int8,
        user_id -> Int4,
        wallet_id -> Int4,
        ethereum_address -> Text,
        platform_address -> Text,
        amount -> Numeric,
        #[max_length = 32]
        gas_payer -> Varchar,
        tax_amount -> Numeric,
        #[max_length = 32]
        status -> Varchar,
        chain_id -> Nullable<Int8>,
        contract_address -> Nullable<Text>,
        transaction_hash -> Nullable<Text>,
        log_index -> Nullable<Int8>,
        #[max_length = 50]
        event_type -> Nullable<Varchar>,
        external_transaction_id -> Nullable<Int8>,
        transaction_id -> Nullable<Int8>,
        #[max_length = 32]
        wallet_provider -> Varchar,
        metamask_required -> Bool,
        #[max_length = 64]
        wallet_action -> Varchar,
        last_error -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        credited_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    wallets (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        organization_id -> Nullable<Int4>,
        value -> Numeric,
    }
}

diesel::joinable!(authentications -> users (user_id));
diesel::joinable!(chapters -> courses (course_id));
diesel::joinable!(contents -> chapters (chapter_id));
diesel::joinable!(course_join_requests -> courses (course_id));
diesel::joinable!(course_progress -> contents (content_id));
diesel::joinable!(course_progress -> courses (course_id));
diesel::joinable!(course_progress -> users (user_id));
diesel::joinable!(courses_organizations -> courses (course_id));
diesel::joinable!(courses_organizations -> organizations (organization_id));
diesel::joinable!(delegated_permissions -> courses (course_id));
diesel::joinable!(delegated_permissions -> organizations (organization_id));
diesel::joinable!(email_verification_tokens -> users (user_id));
diesel::joinable!(internal_transactions -> wallets (wallet_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(paths_courses -> courses (course_id));
diesel::joinable!(paths_courses -> paths (path_id));
diesel::joinable!(pending_course_organization_invites -> courses (course_id));
diesel::joinable!(pending_course_organization_invites -> organizations (organization_id));
diesel::joinable!(reward_audit_events -> reward_candidates (reward_candidate_id));
diesel::joinable!(reward_audit_events -> users (actor_user_id));
diesel::joinable!(reward_candidates -> courses (course_id));
diesel::joinable!(reward_candidates -> organizations (source_organization_id));
diesel::joinable!(reward_compensation_records -> internal_transactions (internal_transaction_id));
diesel::joinable!(reward_compensation_records -> reward_candidates (reward_candidate_id));
diesel::joinable!(reward_compensation_records -> transactions (transaction_id));
diesel::joinable!(reward_compensation_records -> users (created_by_user_id));
diesel::joinable!(reward_compensation_records -> wallets (wallet_id));
diesel::joinable!(reward_execution_jobs -> reward_candidates (reward_candidate_id));
diesel::joinable!(reward_fraud_blocks -> courses (course_id));
diesel::joinable!(reward_fraud_blocks -> organizations (organization_id));
diesel::joinable!(reward_fraud_blocks -> reward_policies (reward_policy_id));
diesel::joinable!(reward_payout_records -> external_transactions (external_transaction_id));
diesel::joinable!(reward_payout_records -> reward_candidates (reward_candidate_id));
diesel::joinable!(reward_payout_records -> transactions (transaction_id));
diesel::joinable!(reward_policies -> courses (course_id));
diesel::joinable!(reward_policies -> organizations (organization_id));
diesel::joinable!(reward_policies -> users (created_by_user_id));
diesel::joinable!(reward_wallet_credit_records -> internal_transactions (internal_transaction_id));
diesel::joinable!(reward_wallet_credit_records -> notifications (notification_id));
diesel::joinable!(reward_wallet_credit_records -> reward_candidates (reward_candidate_id));
diesel::joinable!(reward_wallet_credit_records -> transactions (transaction_id));
diesel::joinable!(reward_wallet_credit_records -> wallets (wallet_id));
diesel::joinable!(role_course_hierarchy -> course_roles (course_role_id));
diesel::joinable!(role_organization_hierarchy -> organization_roles (organization_role_id));
diesel::joinable!(role_permission_course -> course_roles (course_role_id));
diesel::joinable!(role_permission_course -> courses (course_id));
diesel::joinable!(role_permission_organization -> organization_roles (organization_role_id));
diesel::joinable!(role_permission_organization -> organizations (organization_id));
diesel::joinable!(role_permission_platform -> platform_roles (platform_role_id));
diesel::joinable!(role_platform_hierarchy -> platform_roles (platform_role_id));
diesel::joinable!(teacher_application_audit_events -> teacher_applications (application_id));
diesel::joinable!(teacher_application_audit_events -> users (actor_user_id));
diesel::joinable!(teacher_applications -> courses (requested_course_id));
diesel::joinable!(transactions_external_transactions -> external_transactions (external_transaction_id));
diesel::joinable!(transactions_external_transactions -> transactions (transaction_id));
diesel::joinable!(transactions_internal_transactions -> internal_transactions (internal_transaction_id));
diesel::joinable!(transactions_internal_transactions -> transactions (transaction_id));
diesel::joinable!(upload_jobs -> users (user_id));
diesel::joinable!(user_role_course -> course_roles (course_role_id));
diesel::joinable!(user_role_course -> courses (course_id));
diesel::joinable!(user_role_course -> users (user_id));
diesel::joinable!(user_role_organization -> organization_roles (organization_role_id));
diesel::joinable!(user_role_organization -> organizations (organization_id));
diesel::joinable!(user_role_organization -> users (user_id));
diesel::joinable!(user_role_platform -> platform_roles (platform_role_id));
diesel::joinable!(user_role_platform -> users (user_id));
diesel::joinable!(wallet_token_deposit_intents -> external_transactions (external_transaction_id));
diesel::joinable!(wallet_token_deposit_intents -> transactions (transaction_id));
diesel::joinable!(wallet_token_deposit_intents -> users (user_id));
diesel::joinable!(wallet_token_deposit_intents -> wallets (wallet_id));
diesel::joinable!(wallets -> organizations (organization_id));
diesel::joinable!(wallets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    authentications,
    chapters,
    contents,
    course_join_requests,
    course_progress,
    course_roles,
    courses,
    courses_organizations,
    db_version_control,
    delegated_permissions,
    email_verification_tokens,
    external_transactions,
    internal_transactions,
    notifications,
    organization_roles,
    organizations,
    paths,
    paths_courses,
    pending_course_organization_invites,
    persistent_states,
    platform_roles,
    reward_audit_events,
    reward_candidates,
    reward_compensation_records,
    reward_execution_jobs,
    reward_fraud_blocks,
    reward_payout_records,
    reward_policies,
    reward_wallet_credit_records,
    role_course_hierarchy,
    role_organization_hierarchy,
    role_permission_course,
    role_permission_organization,
    role_permission_platform,
    role_platform_hierarchy,
    teacher_application_audit_events,
    teacher_applications,
    transactions,
    transactions_external_transactions,
    transactions_internal_transactions,
    upload_jobs,
    user_role_course,
    user_role_organization,
    user_role_platform,
    users,
    wallet_token_deposit_intents,
    wallets,
);
