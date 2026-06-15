#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentItemOutput {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}

pub(crate) struct ContentItemFact {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}

pub(crate) fn content_item_output(fact: ContentItemFact) -> ContentItemOutput {
    ContentItemOutput {
        id: fact.id,
        chapter_id: fact.chapter_id,
        order: fact.order,
        content_type: fact.content_type,
        data: fact.data,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateContentItemOutput {
    pub content: ContentItemOutput,
    pub notification_recipient_ids: Vec<i32>,
    pub notification_recipient_lookup_error: Option<String>,
}
