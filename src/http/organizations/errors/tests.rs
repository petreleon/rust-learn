use super::organization_management_error;
use super::organization_member_role_assignment_error;
use crate::application::organizations::assign_organization_member_role::OrganizationMemberRoleAssignmentError;
use crate::application::organizations::manage_organizations::OrganizationManagementError;
use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
use serde_json::Value;

#[actix_web::test]
async fn organization_not_found_uses_api_error_envelope() {
    let response = organization_management_error(
        OrganizationManagementError::NotFound,
        Some(7),
        "organization_fetch_failed",
        "Failed to fetch organization",
    )
    .error_response();
    let body = parse_body(response).await;

    assert_eq!(body.status, StatusCode::NOT_FOUND);
    assert_eq!(body.value["error"]["code"], "organization_not_found");
    assert_eq!(body.value["error"]["message"], "Organization not found");
}

#[actix_web::test]
async fn hierarchy_violation_uses_forbidden_envelope() {
    let response = organization_member_role_assignment_error(
        OrganizationMemberRoleAssignmentError::HierarchyViolation,
        7,
        1,
        2,
        "ADMIN",
    )
    .error_response();
    let body = parse_body(response).await;

    assert_eq!(body.status, StatusCode::FORBIDDEN);
    assert_eq!(body.value["error"]["code"], "hierarchy_denied");
    assert_eq!(
        body.value["error"]["message"],
        "Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."
    );
}

struct ParsedErrorBody {
    status: StatusCode,
    value: Value,
}

async fn parse_body(response: actix_web::HttpResponse) -> ParsedErrorBody {
    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let value = serde_json::from_slice(&body).unwrap();
    ParsedErrorBody { status, value }
}
