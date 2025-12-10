use actix_web::dev::ServiceRequest;

use crate::models::param_type::ParamType;
use serde_urlencoded::from_str;

pub fn extract_param(req: &ServiceRequest, param_name: &str, param_type: ParamType) -> Option<String> {
    match param_type {
        ParamType::Header => req.headers().get(param_name).and_then(|hv| hv.to_str().ok()).map(|s| s.to_string()),
        ParamType::Query => {
            req.uri().query().and_then(|query| from_str::<std::collections::HashMap<String, String>>(query).ok()).and_then(|params| params.get(param_name).cloned())
        },
        ParamType::Path => req.match_info().get(param_name).map(|s| s.to_string()),
    }
}
