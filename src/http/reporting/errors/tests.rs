use super::{
    organization_summary_error, platform_csv_export_error, platform_summary_error, ReportOperation,
};
use crate::application::reporting::organization_summary::OrganizationSummaryError;
use crate::application::reporting::platform_csv_exports::PlatformCsvExportError;
use crate::application::reporting::platform_summary::PlatformSummaryError;
use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
use serde_json::Value;

#[actix_web::test]
async fn organization_not_found_uses_api_error_envelope() {
    let body = parse_body(
        organization_summary_error(OrganizationSummaryError::NotFound, ReportOperation::Load, 7)
            .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::NOT_FOUND);
    assert_eq!(body.value["error"]["code"], "organization_not_found");
    assert_eq!(body.value["error"]["message"], "Organization not found");
}

#[actix_web::test]
async fn platform_summary_export_error_names_report_action() {
    let body = parse_body(
        platform_summary_error(
            PlatformSummaryError::Database("boom".to_string()),
            ReportOperation::Export,
        )
        .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body.value["error"]["code"], "reporting_request_failed");
    assert_eq!(
        body.value["error"]["message"],
        "Failed to export platform report"
    );
}

#[actix_web::test]
async fn csv_export_error_preserves_dataset_label() {
    let body = parse_body(
        platform_csv_export_error(
            PlatformCsvExportError::Database("boom".to_string()),
            "wallet_credits",
            "wallet credits",
        )
        .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body.value["error"]["code"], "reporting_request_failed");
    assert_eq!(
        body.value["error"]["message"],
        "Failed to export wallet credits"
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
