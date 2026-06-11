/// Kubernetes smoke tests for platform admin routes.
/// These tests run against in-cluster endpoints via port-forward or
/// from a pod with cluster DNS access.
///
/// Run manually with:
///   kubectl port-forward -n rust-learn svc/web 3000:3000 &
///   kubectl port-forward -n rust-learn svc/rust-app 8080:8080 &
///   cargo test --test kubernetes_smoke -- --ignored
#[cfg(test)]
mod kubernetes_smoke {
    use reqwest::Client;
    use std::time::Duration;

    fn app_url() -> String {
        std::env::var("K8S_APP_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
    }

    fn web_url() -> String {
        std::env::var("K8S_WEB_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
    }

    async fn http_client() -> Client {
        Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .expect("reqwest client")
    }

    #[tokio::test]
    #[ignore = "requires Kubernetes port-forward or in-cluster endpoints"]
    async fn k8s_web_healthz_ok() {
        let client = http_client().await;
        let resp = client
            .get(format!("{}/healthz", web_url()))
            .send()
            .await
            .expect("web healthz reachable in k8s");
        assert_eq!(resp.status().as_u16(), 200);
    }

    #[tokio::test]
    #[ignore = "requires Kubernetes port-forward or in-cluster endpoints"]
    async fn k8s_api_health_ok() {
        let client = http_client().await;
        let resp = client
            .get(format!("{}/health", app_url()))
            .send()
            .await
            .expect("api health reachable in k8s");
        assert_eq!(resp.status().as_u16(), 200);
        let body: serde_json::Value = resp.json().await.expect("json body");
        assert_eq!(body["status"].as_str(), Some("ok"));
    }

    #[tokio::test]
    #[ignore = "requires Kubernetes port-forward or in-cluster endpoints"]
    async fn k8s_api_ready_returns_valid_status() {
        let client = http_client().await;
        let resp = client
            .get(format!("{}/ready", app_url()))
            .send()
            .await
            .expect("api ready reachable in k8s");
        let status = resp.status().as_u16();
        assert!(
            status == 200 || status == 503,
            "ready should be 200 or 503, got {status}"
        );
        let body: serde_json::Value = resp.json().await.expect("json body");
        assert!(
            body["status"].is_string(),
            "ready body should have status string"
        );
        let checks = body["checks"].as_array().expect("checks array");
        assert!(!checks.is_empty(), "ready should return at least one check");
    }

    #[tokio::test]
    #[ignore = "requires Kubernetes port-forward or in-cluster endpoints"]
    async fn k8s_web_admin_route_renders() {
        let client = http_client().await;
        let resp = client
            .get(format!("{}/admin", web_url()))
            .send()
            .await
            .expect("web /admin reachable in k8s");
        assert_eq!(resp.status().as_u16(), 200);
        let body = resp.text().await.expect("text body");
        assert!(
            body.contains("Platform admin"),
            "admin page should render in k8s"
        );
    }

    #[tokio::test]
    #[ignore = "requires Kubernetes port-forward or in-cluster endpoints"]
    async fn k8s_web_system_route_renders() {
        let client = http_client().await;
        let resp = client
            .get(format!("{}/admin/system", web_url()))
            .send()
            .await
            .expect("web /admin/system reachable in k8s");
        assert_eq!(resp.status().as_u16(), 200);
        let body = resp.text().await.expect("text body");
        assert!(
            body.contains("System status"),
            "system page should render in k8s"
        );
    }

    #[tokio::test]
    #[ignore = "requires Kubernetes port-forward or in-cluster endpoints"]
    async fn k8s_api_reports_summary_json() {
        let client = http_client().await;
        let resp = client
            .get(format!("{}/api/reports/platform/summary", app_url()))
            .send()
            .await
            .expect("reports summary reachable in k8s");
        // 401 without auth is expected; 200 with auth is expected.
        // We just verify the route is mounted and does not 404 or 500.
        let status = resp.status().as_u16();
        assert!(
            status == 200 || status == 401 || status == 403,
            "reports summary should be mounted, got {status}"
        );
    }
}
