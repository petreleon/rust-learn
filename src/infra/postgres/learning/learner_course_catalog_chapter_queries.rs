use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::learner_course_catalog::{
    LearnerCourseCatalogChapterOutput, LearnerCourseCatalogContentOutput, LearnerCourseCatalogError,
};
use crate::infra::postgres::schema::{chapters, contents};

pub async fn load_learner_course_catalog_chapters(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseCatalogChapterOutput>, LearnerCourseCatalogError> {
    let chapter_rows = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .order(chapters::order.asc())
        .select((chapters::id, chapters::title, chapters::order))
        .load::<(i32, String, i32)>(conn)
        .await
        .map_err(map_catalog_error)?;

    let mut result = Vec::with_capacity(chapter_rows.len());
    for (id, title, order) in chapter_rows {
        let contents = load_catalog_contents(conn, id).await?;
        result.push(LearnerCourseCatalogChapterOutput {
            id,
            title,
            order,
            contents,
        });
    }

    Ok(result)
}

async fn load_catalog_contents(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
) -> Result<Vec<LearnerCourseCatalogContentOutput>, LearnerCourseCatalogError> {
    let rows = contents::table
        .filter(contents::chapter_id.eq(chapter_id))
        .order(contents::order.asc())
        .select((contents::id, contents::order, contents::content_type))
        .load::<(i32, i32, String)>(conn)
        .await
        .map_err(map_catalog_error)?;

    Ok(rows
        .into_iter()
        .map(
            |(id, order, content_type)| LearnerCourseCatalogContentOutput {
                id,
                order,
                content_type,
            },
        )
        .collect())
}

fn map_catalog_error(error: diesel::result::Error) -> LearnerCourseCatalogError {
    match error {
        diesel::result::Error::NotFound => LearnerCourseCatalogError::NotFound,
        other => LearnerCourseCatalogError::Database(other.to_string()),
    }
}
