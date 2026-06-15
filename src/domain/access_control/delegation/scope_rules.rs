use super::permission_rules::{
    ensure_course_permission_scope, ensure_organization_permission_scope,
    ensure_platform_permission_scope,
};
use super::DelegationRuleError;

pub const DELEGATED_SCOPE_COURSE: &str = "course";
pub const DELEGATED_SCOPE_ORGANIZATION: &str = "organization";
pub const DELEGATED_SCOPE_PLATFORM: &str = "platform";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelegatedScope {
    kind: DelegatedScopeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DelegatedScopeKind {
    Platform,
    Organization { organization_id: i32 },
    Course { course_id: i32 },
}

impl DelegatedScope {
    pub fn platform() -> Self {
        Self {
            kind: DelegatedScopeKind::Platform,
        }
    }

    pub fn organization(organization_id: i32) -> Self {
        Self {
            kind: DelegatedScopeKind::Organization { organization_id },
        }
    }

    pub fn course(course_id: i32) -> Self {
        Self {
            kind: DelegatedScopeKind::Course { course_id },
        }
    }

    pub fn scope_type(&self) -> &'static str {
        match self.kind {
            DelegatedScopeKind::Platform => DELEGATED_SCOPE_PLATFORM,
            DelegatedScopeKind::Organization { .. } => DELEGATED_SCOPE_ORGANIZATION,
            DelegatedScopeKind::Course { .. } => DELEGATED_SCOPE_COURSE,
        }
    }

    pub fn organization_id(&self) -> Option<i32> {
        match self.kind {
            DelegatedScopeKind::Organization { organization_id } => Some(organization_id),
            _ => None,
        }
    }

    pub fn course_id(&self) -> Option<i32> {
        match self.kind {
            DelegatedScopeKind::Course { course_id } => Some(course_id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationScope {
    pub delegated_scope: DelegatedScope,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
}

pub fn normalize_scope_type(scope_type: &str) -> Result<String, DelegationRuleError> {
    let normalized = scope_type.trim().to_ascii_lowercase().replace('-', "_");
    match normalized.as_str() {
        DELEGATED_SCOPE_PLATFORM | DELEGATED_SCOPE_ORGANIZATION | DELEGATED_SCOPE_COURSE => {
            Ok(normalized)
        }
        _ => Err(DelegationRuleError::InvalidInput(
            "unsupported delegated permission scope".to_string(),
        )),
    }
}

pub fn normalize_filter_scope_type(
    scope_type: Option<String>,
) -> Result<Option<String>, DelegationRuleError> {
    scope_type.as_deref().map(normalize_scope_type).transpose()
}

pub fn normalize_scope_ids(
    permission: &str,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<DelegationScope, DelegationRuleError> {
    match scope_type {
        DELEGATED_SCOPE_PLATFORM => {
            normalize_platform_scope(permission, organization_id, course_id)
        }
        DELEGATED_SCOPE_ORGANIZATION => {
            normalize_organization_scope(permission, organization_id, course_id)
        }
        DELEGATED_SCOPE_COURSE => normalize_course_scope(permission, organization_id, course_id),
        _ => Err(DelegationRuleError::InvalidInput(
            "unsupported delegated permission scope".to_string(),
        )),
    }
}

fn normalize_platform_scope(
    permission: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<DelegationScope, DelegationRuleError> {
    ensure_platform_permission_scope(permission)?;
    if organization_id.is_some() || course_id.is_some() {
        return Err(DelegationRuleError::InvalidInput(
            "platform delegation cannot include organization_id or course_id".to_string(),
        ));
    }
    Ok(scope(DelegatedScope::platform()))
}

fn normalize_organization_scope(
    permission: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<DelegationScope, DelegationRuleError> {
    ensure_organization_permission_scope(permission)?;
    let organization_id = organization_id.ok_or_else(|| {
        DelegationRuleError::InvalidInput(
            "organization delegation requires organization_id".to_string(),
        )
    })?;
    if course_id.is_some() {
        return Err(DelegationRuleError::InvalidInput(
            "organization delegation cannot include course_id".to_string(),
        ));
    }
    Ok(scope(DelegatedScope::organization(organization_id)))
}

fn normalize_course_scope(
    permission: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<DelegationScope, DelegationRuleError> {
    ensure_course_permission_scope(permission)?;
    let course_id = course_id.ok_or_else(|| {
        DelegationRuleError::InvalidInput("course delegation requires course_id".to_string())
    })?;
    if organization_id.is_some() {
        return Err(DelegationRuleError::InvalidInput(
            "course delegation cannot include organization_id".to_string(),
        ));
    }
    Ok(scope(DelegatedScope::course(course_id)))
}

fn scope(delegated_scope: DelegatedScope) -> DelegationScope {
    DelegationScope {
        course_id: delegated_scope.course_id(),
        delegated_scope,
        organization_id: delegated_scope.organization_id(),
        scope_type: delegated_scope.scope_type().to_string(),
    }
}
