pub fn is_video_content_type(content_type: &str) -> bool {
    let normalized = content_type.trim().to_ascii_lowercase();
    normalized == "video" || normalized.starts_with("video/")
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
}
