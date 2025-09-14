use axum::{
    extract::{Path, State},
    response::Json,
};

use crate::{
    error::Result,
    models::{ApiResponse, CreateScopeRequest, ScopeInfo},
    services::CouchbaseService,
};

pub async fn create_scope(
    State(couchbase_service): State<CouchbaseService>,
    Path(bucket): Path<String>,
    Json(payload): Json<CreateScopeRequest>,
) -> Result<Json<ApiResponse<ScopeInfo>>> {
    // Validate scope name
    if payload.scope_name.is_empty() {
        return Ok(Json(ApiResponse::error(
            "Scope name cannot be empty".to_string(),
        )));
    }

    // Check if bucket exists
    let buckets = couchbase_service.list_buckets().await?;
    if !buckets.iter().any(|b| b.name == bucket) {
        return Ok(Json(ApiResponse::error(format!(
            "Bucket '{}' not found",
            bucket
        ))));
    }

    // Check if scope already exists
    let existing_scopes = couchbase_service.list_scopes(&bucket).await?;
    if existing_scopes.iter().any(|s| s.name == payload.scope_name) {
        return Ok(Json(ApiResponse::error(format!(
            "Scope '{}' already exists in bucket '{}'",
            payload.scope_name, bucket
        ))));
    }

    // Create the scope
    couchbase_service
        .create_scope(&bucket, &payload.scope_name)
        .await?;

    // Return the created scope info
    let scope_info = ScopeInfo {
        name: payload.scope_name,
        collections: vec![],
    };

    Ok(Json(ApiResponse::success(scope_info)))
}

pub async fn list_scopes(
    State(couchbase_service): State<CouchbaseService>,
    Path(bucket): Path<String>,
) -> Result<Json<ApiResponse<Vec<ScopeInfo>>>> {
    // Check if bucket exists
    let buckets = couchbase_service.list_buckets().await?;
    if !buckets.iter().any(|b| b.name == bucket) {
        return Ok(Json(ApiResponse::error(format!(
            "Bucket '{}' not found",
            bucket
        ))));
    }

    let scopes = couchbase_service.list_scopes(&bucket).await?;
    Ok(Json(ApiResponse::success(scopes)))
}
