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
    }
}

diesel::table! {
    db_version_control (id) {
        id -> Int4,
        version -> Int4,
    }
}

diesel::table! {
    external_transactions (id) {
        id -> Int8,
        amount -> Numeric,
        blockchain_address -> Text,
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

diesel::joinable!(authentications -> users (user_id));
diesel::joinable!(chapters -> courses (course_id));
diesel::joinable!(contents -> chapters (chapter_id));
diesel::joinable!(internal_transactions -> wallets (wallet_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(paths_courses -> courses (course_id));
diesel::joinable!(paths_courses -> paths (path_id));
diesel::joinable!(role_course_hierarchy -> course_roles (course_role_id));
diesel::joinable!(role_organization_hierarchy -> organization_roles (organization_role_id));
diesel::joinable!(role_permission_course -> course_roles (course_role_id));
diesel::joinable!(role_permission_course -> courses (course_id));
diesel::joinable!(role_permission_organization -> organization_roles (organization_role_id));
diesel::joinable!(role_permission_organization -> organizations (organization_id));
diesel::joinable!(role_permission_platform -> platform_roles (platform_role_id));
diesel::joinable!(role_platform_hierarchy -> platform_roles (platform_role_id));
diesel::joinable!(transactions_external_transactions -> external_transactions (external_transaction_id));
diesel::joinable!(transactions_external_transactions -> transactions (transaction_id));
diesel::joinable!(transactions_internal_transactions -> internal_transactions (internal_transaction_id));
diesel::joinable!(transactions_internal_transactions -> transactions (transaction_id));
diesel::joinable!(user_role_course -> course_roles (course_role_id));
diesel::joinable!(user_role_course -> courses (course_id));
diesel::joinable!(user_role_course -> users (user_id));
diesel::joinable!(user_role_organization -> organization_roles (organization_role_id));
diesel::joinable!(user_role_organization -> organizations (organization_id));
diesel::joinable!(user_role_organization -> users (user_id));
diesel::joinable!(user_role_platform -> platform_roles (platform_role_id));
diesel::joinable!(user_role_platform -> users (user_id));
diesel::joinable!(wallets -> organizations (organization_id));
diesel::joinable!(wallets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    authentications,chapters,contents,course_roles,courses,db_version_control,external_transactions,internal_transactions,notifications,organization_roles,organizations,paths,paths_courses,persistent_states,platform_roles,role_course_hierarchy,role_organization_hierarchy,role_permission_course,role_permission_organization,role_permission_platform,role_platform_hierarchy,transactions,transactions_external_transactions,transactions_internal_transactions,user_role_course,user_role_organization,user_role_platform,users,wallets,);
