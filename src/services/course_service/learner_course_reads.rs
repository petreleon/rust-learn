use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, courses_organizations};
use crate::models::course::Course;
use diesel::prelude::*;
use diesel::{EscapeExpressionMethods, PgTextExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::catalog_dashboard_builders::build_learner_course_catalog_item;
use super::errors::LearnerCourseCatalogError;
use super::learner_enrollment::load_learner_course_chapters;
use super::learner_learning_helpers::load_learner_course_learning_chapters;
use super::learner_permissions::course_visible_to_learner;
use super::query_types::{
    LearnerCourseCatalogQuery, LearnerCourseCatalogResponse, LearnerCourseDetailResponse,
    LearnerCourseLearningResponse,
};
use super::shared_helpers::course_title_search_pattern;
use super::LIKE_ESCAPE_CHAR;

pub async fn discover_learner_course_catalog(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    catalog_query: LearnerCourseCatalogQuery,
) -> Result<LearnerCourseCatalogResponse, LearnerCourseCatalogError> {
    let mut course_query = courses::table.into_boxed();

    if let Some(search) = catalog_query.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        course_query = course_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(organization_id) = catalog_query.organization_id {
        course_query = course_query.filter(
            courses::id.eq_any(
                courses_organizations::table
                    .filter(courses_organizations::organization_id.eq(organization_id))
                    .select(courses_organizations::course_id),
            ),
        );
    }

    if let Some(status) = catalog_query.lifecycle_status.as_deref() {
        course_query = course_query.filter(courses::lifecycle_status.eq(status));
    }

    let candidate_courses = course_query
        .order(courses::id.asc())
        .load::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut items = Vec::new();
    for course in candidate_courses {
        if !course_visible_to_learner(conn, actor_user_id, &course).await? {
            continue;
        }

        let item = build_learner_course_catalog_item(conn, actor_user_id, course).await?;

        if let Some(reward_available) = catalog_query.reward_available {
            if item.rewards.available != reward_available {
                continue;
            }
        }

        if let Some(enrollment_status) = catalog_query.enrollment_status.as_deref() {
            if item.enrollment.state != enrollment_status {
                continue;
            }
        }

        items.push(item);
    }

    let total = items.len() as i64;
    let courses = items
        .into_iter()
        .skip(catalog_query.offset as usize)
        .take(catalog_query.limit as usize)
        .collect();

    Ok(LearnerCourseCatalogResponse {
        courses,
        total,
        limit: catalog_query.limit,
        offset: catalog_query.offset,
        search: catalog_query.search,
        organization_id: catalog_query.organization_id,
        lifecycle_status: catalog_query.lifecycle_status,
        enrollment_status: catalog_query.enrollment_status,
        reward_available: catalog_query.reward_available,
    })
}

pub async fn get_learner_course_detail(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<LearnerCourseDetailResponse, LearnerCourseCatalogError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    if !course_visible_to_learner(conn, actor_user_id, &course).await? {
        return Err(LearnerCourseCatalogError::NotFound);
    }

    let course = build_learner_course_catalog_item(conn, actor_user_id, course).await?;
    let chapters = load_learner_course_chapters(conn, course_id).await?;
    let prerequisites = course.prerequisites.clone();

    Ok(LearnerCourseDetailResponse {
        course,
        chapters,
        prerequisites,
    })
}

pub async fn get_learner_course_learning(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<LearnerCourseLearningResponse, LearnerCourseCatalogError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    if !course_visible_to_learner(conn, actor_user_id, &course).await? {
        return Err(LearnerCourseCatalogError::NotFound);
    }

    let course = build_learner_course_catalog_item(conn, actor_user_id, course).await?;
    if !course.access.can_view_content {
        return Err(LearnerCourseCatalogError::PermissionDenied(
            Permissions::VIEW_CONTENT.to_string(),
        ));
    }

    let chapters = load_learner_course_learning_chapters(conn, course_id).await?;
    let active_content_id = chapters
        .iter()
        .flat_map(|chapter| {
            chapter
                .contents
                .iter()
                .map(move |content| (chapter.order, chapter.id, content.order, content.id))
        })
        .min_by_key(|(chapter_order, chapter_id, content_order, content_id)| {
            (*chapter_order, *chapter_id, *content_order, *content_id)
        })
        .map(|(_, _, _, content_id)| content_id);

    let progress_supported = course.enrollment.state == "enrolled";

    Ok(LearnerCourseLearningResponse {
        course,
        chapters,
        active_content_id,
        progress_supported,
    })
}
