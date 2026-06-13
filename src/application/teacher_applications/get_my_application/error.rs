#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationSelfError {
    Connection(String),
    Database(String),
}
