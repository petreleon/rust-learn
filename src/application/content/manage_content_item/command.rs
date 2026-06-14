#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateContentItemCommand {
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateContentItemCommand {
    pub order: Option<i32>,
    pub content_type: Option<String>,
    pub data: Option<String>,
}
