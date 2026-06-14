use actix_web::dev::ServiceRequest;
use serde_urlencoded::from_str;

#[derive(Clone, Copy)]
pub enum ParamType {
    Header,
    Query,
    Path,
}

pub fn extract_param(
    req: &ServiceRequest,
    param_name: &str,
    param_type: ParamType,
) -> Option<String> {
    match param_type {
        ParamType::Header => req
            .headers()
            .get(param_name)
            .and_then(|hv| hv.to_str().ok())
            .map(|s| s.to_string()),
        ParamType::Query => req
            .uri()
            .query()
            .and_then(|query| from_str::<std::collections::HashMap<String, String>>(query).ok())
            .and_then(|params| params.get(param_name).cloned()),
        ParamType::Path => req.match_info().get(param_name).map(|s| s.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn srv_req() -> actix_web::test::TestRequest {
        actix_web::test::TestRequest::default()
    }

    #[actix_web::test]
    async fn extracts_path_param() {
        let req = srv_req().param("course_id", "42").to_srv_request();
        assert_eq!(
            extract_param(&req, "course_id", ParamType::Path),
            Some("42".into())
        );
    }

    #[actix_web::test]
    async fn path_param_missing_returns_none() {
        let req = srv_req().to_srv_request();
        assert_eq!(extract_param(&req, "missing", ParamType::Path), None);
    }

    #[actix_web::test]
    async fn path_param_wrong_name_returns_none() {
        let req = srv_req().param("course_id", "42").to_srv_request();
        assert_eq!(extract_param(&req, "wrong_name", ParamType::Path), None);
    }

    #[actix_web::test]
    async fn extracts_query_param() {
        let req = srv_req().uri("/?search=rust&page=1").to_srv_request();
        assert_eq!(
            extract_param(&req, "search", ParamType::Query),
            Some("rust".into())
        );
        assert_eq!(
            extract_param(&req, "page", ParamType::Query),
            Some("1".into())
        );
    }

    #[actix_web::test]
    async fn query_param_missing_returns_none() {
        let req = srv_req().uri("/?search=rust").to_srv_request();
        assert_eq!(extract_param(&req, "missing", ParamType::Query), None);
    }

    #[actix_web::test]
    async fn query_param_empty_uri_returns_none() {
        let req = srv_req().uri("/").to_srv_request();
        assert_eq!(extract_param(&req, "anything", ParamType::Query), None);
    }

    #[actix_web::test]
    async fn extracts_header_param() {
        let req = srv_req()
            .insert_header(("x-custom-header", "header-value"))
            .to_srv_request();
        assert_eq!(
            extract_param(&req, "x-custom-header", ParamType::Header),
            Some("header-value".into())
        );
    }

    #[actix_web::test]
    async fn header_param_missing_returns_none() {
        let req = srv_req().to_srv_request();
        assert_eq!(
            extract_param(&req, "x-missing-header", ParamType::Header),
            None
        );
    }

    #[actix_web::test]
    async fn header_param_wrong_name_returns_none() {
        let req = srv_req()
            .insert_header(("x-custom", "value"))
            .to_srv_request();
        assert_eq!(extract_param(&req, "x-other", ParamType::Header), None);
    }

    #[actix_web::test]
    async fn param_type_path_does_not_leak_into_query() {
        let req = srv_req()
            .param("course_id", "42")
            .uri("/?course_id=99")
            .to_srv_request();
        assert_eq!(
            extract_param(&req, "course_id", ParamType::Path),
            Some("42".into())
        );
        assert_eq!(
            extract_param(&req, "course_id", ParamType::Query),
            Some("99".into())
        );
    }

    #[actix_web::test]
    async fn param_type_query_does_not_leak_into_path() {
        let req = srv_req()
            .param("organization_id", "7")
            .uri("/?organization_id=3")
            .to_srv_request();
        assert_eq!(
            extract_param(&req, "organization_id", ParamType::Path),
            Some("7".into())
        );
        assert_eq!(
            extract_param(&req, "organization_id", ParamType::Query),
            Some("3".into())
        );
    }
}
