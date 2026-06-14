const DEFAULT_MEMBER_LIMIT: i64 = 25;
const MAX_MEMBER_LIMIT: i64 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberListQuery {
    pub actor_user_id: i32,
    pub organization_id: i32,
    pub search: Option<String>,
    pub role: Option<String>,
    pub permission: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl OrganizationMemberListQuery {
    pub fn new(
        actor_user_id: i32,
        organization_id: i32,
        search: Option<String>,
        role: Option<String>,
        permission: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        Self {
            actor_user_id,
            organization_id,
            search: normalize_optional_string(search),
            role: normalize_optional_string(role),
            permission: normalize_optional_string(permission),
            limit: limit
                .unwrap_or(DEFAULT_MEMBER_LIMIT)
                .clamp(1, MAX_MEMBER_LIMIT),
            offset: offset.unwrap_or(0).max(0),
        }
    }
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::OrganizationMemberListQuery;

    #[test]
    fn organization_member_list_query_normalizes_bounds() {
        let query = OrganizationMemberListQuery::new(
            7,
            9,
            Some("  delegated ".to_string()),
            Some(" TEACHER ".to_string()),
            Some(" VIEW_ORGANIZATION ".to_string()),
            Some(500),
            Some(-10),
        );

        assert_eq!(query.actor_user_id, 7);
        assert_eq!(query.organization_id, 9);
        assert_eq!(query.search.as_deref(), Some("delegated"));
        assert_eq!(query.role.as_deref(), Some("TEACHER"));
        assert_eq!(query.permission.as_deref(), Some("VIEW_ORGANIZATION"));
        assert_eq!(query.limit, 100);
        assert_eq!(query.offset, 0);
    }
}
