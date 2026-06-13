#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleCatalogScope {
    Platform,
    Organization,
    Course,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleCatalogEntry {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleCatalogError {
    Connection(String),
    Database(String),
}
