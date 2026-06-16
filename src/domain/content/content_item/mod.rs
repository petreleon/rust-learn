pub fn is_video_content_type(content_type: &str) -> bool {
    let normalized = content_type.trim().to_ascii_lowercase();
    normalized == "video" || normalized.starts_with("video/")
}

pub const CONTENT_PUBLICATION_STATUS_PUBLISHED: &str = "published";
pub const CONTENT_PUBLICATION_STATUS_UNPUBLISHED: &str = "unpublished";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentPublicationStatus {
    Published,
    Unpublished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentPublicationStatusError {
    pub value: String,
}

impl ContentPublicationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Published => CONTENT_PUBLICATION_STATUS_PUBLISHED,
            Self::Unpublished => CONTENT_PUBLICATION_STATUS_UNPUBLISHED,
        }
    }

    pub fn normalize(value: &str) -> Result<Self, ContentPublicationStatusError> {
        match value.trim().to_ascii_lowercase().as_str() {
            CONTENT_PUBLICATION_STATUS_PUBLISHED => Ok(Self::Published),
            CONTENT_PUBLICATION_STATUS_UNPUBLISHED => Ok(Self::Unpublished),
            other => Err(ContentPublicationStatusError {
                value: other.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_content_type_accepts_legacy_and_mime_values() {
        assert!(is_video_content_type("video"));
        assert!(is_video_content_type(" video/mp4 "));
        assert!(is_video_content_type("VIDEO/WEBM"));
    }

    #[test]
    fn video_content_type_rejects_non_video_values() {
        assert!(!is_video_content_type("audio/mp3"));
        assert!(!is_video_content_type("application/pdf"));
        assert!(!is_video_content_type(""));
    }

    #[test]
    fn publication_status_normalizes_known_values() {
        assert_eq!(
            ContentPublicationStatus::normalize(" Published ").unwrap(),
            ContentPublicationStatus::Published
        );
        assert_eq!(
            ContentPublicationStatus::Unpublished.as_str(),
            CONTENT_PUBLICATION_STATUS_UNPUBLISHED
        );
    }

    #[test]
    fn publication_status_rejects_unknown_values() {
        assert!(ContentPublicationStatus::normalize("hidden").is_err());
    }
}
