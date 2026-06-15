use actix_web::{CustomizeResponder, Responder};

pub type CsvDownload = CustomizeResponder<String>;

pub fn csv_download(filename: impl AsRef<str>, body: String) -> CsvDownload {
    body.customize()
        .insert_header(("Content-Type", "text/csv; charset=utf-8"))
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename.as_ref()),
        ))
}

pub(crate) fn csv_value(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub(crate) fn csv_optional(value: Option<impl ToString>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{csv_download, csv_optional, csv_value};
    use actix_web::{http::header, test::TestRequest, Responder};

    #[test]
    fn escapes_csv_values_like_legacy_exports() {
        assert_eq!(csv_value("hello"), "hello");
        assert_eq!(csv_value("a,b"), "\"a,b\"");
        assert_eq!(csv_value(r#"say "hi""#), r#""say ""hi""""#);
        assert_eq!(csv_value("line1\nline2"), "\"line1\nline2\"");
        assert_eq!(csv_optional(Some(42)), "42");
        assert_eq!(csv_optional(None::<i32>), "");
    }

    #[test]
    fn csv_download_sets_legacy_download_headers() {
        let request = TestRequest::default().to_http_request();
        let response = csv_download("report.csv", "a,b\n".to_string()).respond_to(&request);

        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/csv; charset=utf-8"
        );
        assert_eq!(
            response.headers().get(header::CONTENT_DISPOSITION).unwrap(),
            "attachment; filename=\"report.csv\""
        );
    }
}
