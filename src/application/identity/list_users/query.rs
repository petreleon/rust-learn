#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListUsersQuery {
    search: Option<String>,
}

impl ListUsersQuery {
    pub fn new(search: Option<String>) -> Self {
        Self {
            search: search
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
        }
    }

    pub fn search_term(&self) -> Option<&str> {
        self.search.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::ListUsersQuery;

    #[test]
    fn trims_empty_search_terms() {
        assert_eq!(
            ListUsersQuery::new(Some("  ".to_string())).search_term(),
            None
        );
        assert_eq!(
            ListUsersQuery::new(Some("  Ada  ".to_string())).search_term(),
            Some("Ada")
        );
    }
}
