// @generated automatically by Diesel CLI.

diesel::table! {
    authentications (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        type_authentication -> Varchar,
        info_auth -> Nullable<Text>,
    }
}

diesel::table! {
    chapters (id) {
        id -> Int4,
        course_id -> Nullable<Int4>,
        title -> Varchar,
        order -> Int4,
    }
}

diesel::table! {
    contents (id) {
        id -> Int4,
        chapter_id -> Nullable<Int4>,
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

diesel::allow_tables_to_appear_in_same_query!(
    authentications,
    chapters,
    contents,
    courses,
    organizations,
    paths,
    paths_courses,
    users,
);
