use axum::{
    extract::{Path, State},
    response::Json,
};

use crate::{
    error::Result,
    models::{ApiResponse, CollectionInfo, CreateCollectionRequest},
    services::CouchbaseService,
};

pub async fn create_collection(
    State(couchbase_service): State<CouchbaseService>,
    Path((bucket, scope)): Path<(String, String)>,
    Json(payload): Json<CreateCollectionRequest>,
) -> Result<Json<ApiResponse<CollectionInfo>>> {
    // Validate collection name
    if payload.collection_name.is_empty() {
        return Ok(Json(ApiResponse::error(
            "Collection name cannot be empty".to_string(),
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

    // Check if scope exists
    let scopes = couchbase_service.list_scopes(&bucket).await?;
    if !scopes.iter().any(|s| s.name == scope) {
        return Ok(Json(ApiResponse::error(format!(
            "Scope '{}' not found in bucket '{}'",
            scope, bucket
        ))));
    }

    // Check if collection already exists
    let existing_collections = couchbase_service.list_collections(&bucket, &scope).await?;
    if existing_collections.iter().any(|c| c.name == payload.collection_name) {
        return Ok(Json(ApiResponse::error(format!(
            "Collection '{}' already exists in scope '{}' of bucket '{}'",
            payload.collection_name, scope, bucket
        ))));
    }

    // Create the collection
    couchbase_service
        .create_collection(
            &bucket,
            &scope,
            &payload.collection_name,
            payload.max_ttl,
            payload.history,
        )
        .await?;

    // Return the created collection info
    let collection_info = CollectionInfo {
        name: payload.collection_name,
        max_ttl: payload.max_ttl,
        history: payload.history,
        scope: scope.clone(),
    };

    Ok(Json(ApiResponse::success(collection_info)))
}

pub async fn list_collections(
    State(couchbase_service): State<CouchbaseService>,
    Path((bucket, scope)): Path<(String, String)>,
) -> Result<Json<ApiResponse<Vec<CollectionInfo>>>> {
    // Check if bucket exists
    let buckets = couchbase_service.list_buckets().await?;
    if !buckets.iter().any(|b| b.name == bucket) {
        return Ok(Json(ApiResponse::error(format!(
            "Bucket '{}' not found",
            bucket
        ))));
    }

    // Check if scope exists
    let scopes = couchbase_service.list_scopes(&bucket).await?;
    if !scopes.iter().any(|s| s.name == scope) {
        return Ok(Json(ApiResponse::error(format!(
            "Scope '{}' not found in bucket '{}'",
            scope, bucket
        ))));
    }

    let collections = couchbase_service.list_collections(&bucket, &scope).await?;
    Ok(Json(ApiResponse::success(collections)))
}
