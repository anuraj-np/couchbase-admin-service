use axum::{
    extract::State,
    response::Json,
};

use crate::{
    error::Result,
    models::{ApiResponse, BucketInfo, CreateBucketRequest, CouchbaseBucketConfig},
    services::CouchbaseService,
};

pub async fn create_bucket(
    State(couchbase_service): State<CouchbaseService>,
    Json(payload): Json<CreateBucketRequest>,
) -> Result<Json<ApiResponse<BucketInfo>>> {
    // Validate bucket name
    if payload.bucket_name.is_empty() {
        return Ok(Json(ApiResponse::error(
            "Bucket name cannot be empty".to_string(),
        )));
    }

    // Check if bucket already exists
    let existing_buckets = couchbase_service.list_buckets().await?;
    if existing_buckets.iter().any(|b| b.name == payload.bucket_name) {
        return Ok(Json(ApiResponse::error(format!(
            "Bucket '{}' already exists",
            payload.bucket_name
        ))));
    }

    // Create bucket configuration
    let bucket_config = CouchbaseBucketConfig {
        name: payload.bucket_name.clone(),
        ram_quota_mb: payload.ram_quota_mb.unwrap_or(100),
        replica_number: payload.replica_number.unwrap_or(1),
        eviction_policy: payload.eviction_policy.unwrap_or_else(|| "valueOnly".to_string()),
        compression_mode: payload.compression_mode.unwrap_or_else(|| "passive".to_string()),
        conflict_resolution_type: payload.conflict_resolution_type
            .unwrap_or_else(|| "seqno".to_string()),
    };

    // Create the bucket
    couchbase_service.create_bucket(&bucket_config).await?;

    // Return the created bucket info
    let bucket_info = BucketInfo {
        name: bucket_config.name,
        ram_quota_mb: bucket_config.ram_quota_mb,
        replica_number: bucket_config.replica_number,
        eviction_policy: bucket_config.eviction_policy,
        compression_mode: bucket_config.compression_mode,
        conflict_resolution_type: bucket_config.conflict_resolution_type,
        status: "healthy".to_string(),
    };

    Ok(Json(ApiResponse::success(bucket_info)))
}

pub async fn list_buckets(
    State(couchbase_service): State<CouchbaseService>,
) -> Result<Json<ApiResponse<Vec<BucketInfo>>>> {
    let buckets = couchbase_service.list_buckets().await?;
    Ok(Json(ApiResponse::success(buckets)))
}
