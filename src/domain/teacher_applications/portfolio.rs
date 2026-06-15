pub type TeacherApplicationPortfolioLinks = serde_json::Value;

pub fn portfolio_links_from_urls(urls: Vec<String>) -> TeacherApplicationPortfolioLinks {
    serde_json::json!(urls)
}
