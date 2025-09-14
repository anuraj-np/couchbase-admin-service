use axum::{
    extract::{Path, State},
    response::Json,
};

use crate::{
    error::Result,
    models::{
        ApiResponse, CreateUserRequest, CouchbaseRole, CouchbaseUserConfig, UserInfo, Role,
        roles,
    },
    services::CouchbaseService,
};

pub async fn create_user(
    State(couchbase_service): State<CouchbaseService>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserInfo>>> {
    // Validate the request
    if let Err(validation_error) = payload.validate() {
        return Ok(Json(ApiResponse::error(validation_error)));
    }

    // Check if user already exists
    match couchbase_service.get_user(&payload.username).await {
        Ok(_) => {
            return Ok(Json(ApiResponse::error(format!(
                "User '{}' already exists",
                payload.username
            ))));
        }
        Err(_) => {
            // User doesn't exist, which is what we want
        }
    }

    // Convert roles to Couchbase format
    let couchbase_roles: Vec<CouchbaseRole> = payload
        .roles
        .iter()
        .map(|role| CouchbaseRole {
            role: role.role.clone(),
            bucket_name: role.bucket.clone(),
            scope_name: role.scope.clone(),
            collection_name: role.collection.clone(),
        })
        .collect();

    // Create user configuration
    let user_config = CouchbaseUserConfig {
        name: payload.username.clone(),
        password: payload.password,
        roles: couchbase_roles,
    };

    // Create the user
    couchbase_service.create_user(&user_config).await?;

    // Return the created user info
    let user_info = UserInfo {
        username: payload.username,
        roles: payload.roles,
        groups: payload.groups.unwrap_or_default(),
    };

    Ok(Json(ApiResponse::success(user_info)))
}

pub async fn list_users(
    State(couchbase_service): State<CouchbaseService>,
) -> Result<Json<ApiResponse<Vec<UserInfo>>>> {
    let users = couchbase_service.list_users().await?;
    Ok(Json(ApiResponse::success(users)))
}

pub async fn get_user(
    State(couchbase_service): State<CouchbaseService>,
    Path(username): Path<String>,
) -> Result<Json<ApiResponse<UserInfo>>> {
    let user = couchbase_service.get_user(&username).await?;
    Ok(Json(ApiResponse::success(user)))
}

pub async fn delete_user(
    State(couchbase_service): State<CouchbaseService>,
    Path(username): Path<String>,
) -> Result<Json<ApiResponse<()>>> {
    couchbase_service.delete_user(&username).await?;
    Ok(Json(ApiResponse::success(())))
}

// Get available roles
pub async fn get_available_roles() -> Result<Json<ApiResponse<serde_json::Value>>> {
    let roles_info = serde_json::json!({
        "console_access_roles": roles::CONSOLE_ACCESS_ROLES,
        "data_access_roles": roles::DATA_ACCESS_ROLES,
        "query_roles": roles::QUERY_ROLES,
        "all_roles": roles::ALL_ROLES,
        "role_descriptions": {
            "admin": "Full administrative access to Couchbase console and all resources",
            "cluster_admin": "Cluster administration access",
            "replication_admin": "Replication management access",
            "views_admin": "Views management access",
            "query_manage": "Query management access",
            "data_reader": "Read access to data (requires bucket)",
            "data_writer": "Write access to data (requires bucket)",
            "data_dcp_reader": "DCP read access (requires bucket)",
            "bucket_full_access": "Full access to specific bucket",
            "query_select": "SELECT query access",
            "query_insert": "INSERT query access",
            "query_update": "UPDATE query access",
            "query_delete": "DELETE query access"
        }
    });
    
    Ok(Json(ApiResponse::success(roles_info)))
}

// Update user roles
pub async fn update_user_roles(
    State(couchbase_service): State<CouchbaseService>,
    Path(username): Path<String>,
    Json(roles): Json<Vec<Role>>,
) -> Result<Json<ApiResponse<UserInfo>>> {
    // Validate roles
    for role in &roles {
        if !roles::is_valid_role(&role.role) {
            return Ok(Json(ApiResponse::error(format!(
                "Invalid role: '{}'. Valid roles are: {:?}", 
                role.role, roles::ALL_ROLES
            ))));
        }
    }

    // Check if user exists
    let mut user = couchbase_service.get_user(&username).await?;
    
    // Update roles
    user.roles = roles;
    
    // Convert to Couchbase format and update
    let couchbase_roles: Vec<CouchbaseRole> = user
        .roles
        .iter()
        .map(|role| CouchbaseRole {
            role: role.role.clone(),
            bucket_name: role.bucket.clone(),
            scope_name: role.scope.clone(),
            collection_name: role.collection.clone(),
        })
        .collect();

    let user_config = CouchbaseUserConfig {
        name: username.clone(),
        password: "SecurePassword123!".to_string(), // Use a default password for updates
        roles: couchbase_roles,
    };

    couchbase_service.update_user(&user_config).await?;
    
    Ok(Json(ApiResponse::success(user)))
}

// Get user permissions summary
pub async fn get_user_permissions(
    State(couchbase_service): State<CouchbaseService>,
    Path(username): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    let user = couchbase_service.get_user(&username).await?;
    
    let console_access = user.roles.iter().any(|role| roles::has_console_access(&role.role));
    let bucket_permissions: Vec<String> = user
        .roles
        .iter()
        .filter_map(|role| role.bucket.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    
    let permissions = serde_json::json!({
        "username": user.username,
        "console_access": console_access,
        "bucket_permissions": bucket_permissions,
        "roles": user.roles,
        "groups": user.groups,
        "permission_summary": {
            "can_access_console": console_access,
            "can_read_data": user.roles.iter().any(|r| r.role == "data_reader"),
            "can_write_data": user.roles.iter().any(|r| r.role == "data_writer"),
            "can_run_queries": user.roles.iter().any(|r| roles::is_query_role(&r.role)),
            "can_manage_buckets": user.roles.iter().any(|r| r.role == "bucket_full_access"),
            "can_administer_cluster": user.roles.iter().any(|r| r.role == "admin" || r.role == "cluster_admin")
        }
    });
    
    Ok(Json(ApiResponse::success(permissions)))
}
