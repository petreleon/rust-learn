#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentItemOutput {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateContentItemOutput {
    pub content: ContentItemOutput,
    pub notification_recipient_ids: Vec<i32>,
    pub notification_recipient_lookup_error: Option<String>,
}
