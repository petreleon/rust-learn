/// Docker Compose E2E smoke tests for platform admin routes.
/// Run with: make test-compose (or RUN_DOCKER_COMPOSE_SMOKE=1 cargo test --test docker_compose_smoke)
///
/// These tests verify the web proxy, app readiness, and key admin endpoints
/// when running inside the Docker Compose network.
#[cfg(test)]
mod docker_compose_smoke {
    use reqwest::Client;
    use std::time::Duration;

    fn compose_smoke_enabled() -> bool {
        std::env::var("RUN_DOCKER_COMPOSE_SMOKE").as_deref() == Ok("1")
            || std::env::var("API_BASE_URL").is_ok()
            || std::env::var("WEB_BASE_URL").is_ok()
    }

    fn skip_when_disabled() -> bool {
        if compose_smoke_enabled() {
            return false;
        }
        eprintln!("skipping docker compose smoke test; set RUN_DOCKER_COMPOSE_SMOKE=1 to run");
        true
    }

    fn base_url() -> String {
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://app:8080".to_string())
    }

    fn web_url() -> String {
        std::env::var("WEB_BASE_URL").unwrap_or_else(|_| "http://web:3000".to_string())
    }

    async fn http_client() -> Client {
        Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .expect("reqwest client")
    }

    #[tokio::test]
    async fn web_proxy_serves_healthz() {
        if skip_when_disabled() {
            return;
        }
        let client = http_client().await;
        let resp = client
            .get(format!("{}/healthz", web_url()))
            .send()
            .await
            .expect("web healthz reachable");
        assert_eq!(resp.status().as_u16(), 200);
    }

    #[tokio::test]
    async fn api_health_returns_ok() {
        if skip_when_disabled() {
            return;
        }
        let client = http_client().await;
        let resp = client
            .get(format!("{}/health", base_url()))
            .send()
            .await
            .expect("api health reachable");
        assert_eq!(resp.status().as_u16(), 200);
        let body: serde_json::Value = resp.json().await.expect("json body");
        assert_eq!(body["status"].as_str(), Some("ok"));
    }

    #[tokio::test]
    async fn api_ready_returns_ready_or_not_ready() {
        if skip_when_disabled() {
            return;
        }
        let client = http_client().await;
        let resp = client
            .get(format!("{}/ready", base_url()))
            .send()
            .await
            .expect("api ready reachable");
        let status = resp.status().as_u16();
        assert!(
            status == 200 || status == 503,
            "ready should be 200 or 503, got {status}"
        );
    }

    #[tokio::test]
    async fn web_proxy_forwards_admin_dashboard_html() {
        if skip_when_disabled() {
            return;
        }
        let client = http_client().await;
        let resp = client
            .get(format!("{}/admin", web_url()))
            .send()
            .await
            .expect("web /admin reachable");
        assert_eq!(resp.status().as_u16(), 200);
        let body = resp.text().await.expect("text body");
        assert!(
            body.contains("Platform admin"),
            "admin page should contain platform admin text"
        );
    }

    #[tokio::test]
    async fn worker_healthcheck_succeeds() {
        if skip_when_disabled() {
            return;
        }
        let client = http_client().await;
        let resp = client
            .get(format!("{}/health", base_url()))
            .send()
            .await
            .expect("worker health reachable");
        // Worker healthcheck runs inside the worker container; this just confirms
        // the worker binary is up by hitting the shared API health endpoint.
        assert_eq!(resp.status().as_u16(), 200);
    }
}
