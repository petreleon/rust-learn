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
    courses (id) {
        id -> Int4,
        title -> Varchar,
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
    role_organization_hierarchy (id) {
        id -> Int4,
        role_id -> Int4,
        organization_id -> Int4,
        hierarchy_level -> Int4,
    }
}

diesel::table! {
    role_permission_organization (id) {
        id -> Int4,
        organization_id -> Int4,
        role_id -> Int4,
        permission -> Varchar,
    }
}

diesel::table! {
    role_permission_platform (id) {
        id -> Int4,
        role_id -> Int4,
        permission -> Varchar,
    }
}

diesel::table! {
    role_platform_hierarchy (id) {
        id -> Int4,
        role_id -> Int4,
        hierarchy_level -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    user_role_organization (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        organization_id -> Int4,
    }
}

diesel::table! {
    user_role_platform (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
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

diesel::joinable!(authentications -> users (user_id));
diesel::joinable!(chapters -> courses (course_id));
diesel::joinable!(contents -> chapters (chapter_id));
diesel::joinable!(paths_courses -> courses (course_id));
diesel::joinable!(paths_courses -> paths (path_id));
diesel::joinable!(role_organization_hierarchy -> organizations (organization_id));
diesel::joinable!(role_organization_hierarchy -> roles (role_id));
diesel::joinable!(role_permission_organization -> organizations (organization_id));
diesel::joinable!(role_permission_organization -> roles (role_id));
diesel::joinable!(role_permission_platform -> roles (role_id));
diesel::joinable!(role_platform_hierarchy -> roles (role_id));
diesel::joinable!(user_role_organization -> organizations (organization_id));
diesel::joinable!(user_role_organization -> roles (role_id));
diesel::joinable!(user_role_organization -> users (user_id));
diesel::joinable!(user_role_platform -> roles (role_id));
diesel::joinable!(user_role_platform -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    authentications,
    chapters,
    contents,
    courses,
    organizations,
    paths,
    paths_courses,
    role_organization_hierarchy,
    role_permission_organization,
    role_permission_platform,
    role_platform_hierarchy,
    roles,
    user_role_organization,
    user_role_platform,
    users,
);
