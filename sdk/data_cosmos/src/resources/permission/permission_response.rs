use super::Permission;
use crate::headers::from_headers::*;
use azure_core::{
    headers::{etag_from_headers, session_token_from_headers},
    Response as HttpResponse,
};

#[derive(Debug, Clone)]
pub struct PermissionResponse {
    pub permission: Permission,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl PermissionResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<PermissionResponse> {
        let (_status_code, headers, body) = response.deconstruct();

        Ok(Self {
            permission: body.json().await?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
        })
    }
}
