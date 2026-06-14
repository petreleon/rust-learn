#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessableContent {
    pub content_type: String,
    pub object_key: Option<String>,
}
