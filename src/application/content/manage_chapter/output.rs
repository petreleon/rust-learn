#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChapterOutput {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub order: i32,
}

pub(crate) struct ChapterFact {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub order: i32,
}

pub(crate) fn chapter_output(fact: ChapterFact) -> ChapterOutput {
    ChapterOutput {
        id: fact.id,
        course_id: fact.course_id,
        title: fact.title,
        order: fact.order,
    }
}
