#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformCsvExportError {
    Connection(String),
    Database(String),
}
