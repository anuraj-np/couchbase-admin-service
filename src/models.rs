use serde::{Deserialize, Serialize};

// Bucket Management Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBucketRequest {
    pub bucket_name: String,
    pub ram_quota_mb: Option<u32>,
    pub replica_number: Option<u32>,
    pub eviction_policy: Option<String>,
    pub compression_mode: Option<String>,
    pub conflict_resolution_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketInfo {
    pub name: String,
    pub ram_quota_mb: u32,
    pub replica_number: u32,
    pub eviction_policy: String,
    pub compression_mode: String,
    pub conflict_resolution_type: String,
    pub status: String,
}

// Scope Management Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScopeRequest {
    pub scope_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeInfo {
    pub name: String,
    pub collections: Vec<CollectionInfo>,
}

// Collection Management Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    pub collection_name: String,
    pub max_ttl: Option<u32>,
    pub history: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub name: String,
    pub max_ttl: Option<u32>,
    pub history: Option<bool>,
    pub scope: String,
}

// User Management Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub roles: Vec<Role>,
    pub groups: Option<Vec<String>>,
    pub display_name: Option<String>,
    pub email: Option<String>,
}

impl CreateUserRequest {
    #[allow(dead_code)] // Available for future use
    pub fn has_console_access(&self) -> bool {
        self.roles.iter().any(|role| roles::has_console_access(&role.role))
    }
    
    #[allow(dead_code)] // Available for future use
    pub fn get_bucket_permissions(&self) -> Vec<String> {
        self.roles
            .iter()
            .filter_map(|role| role.bucket.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
    
    pub fn validate(&self) -> Result<(), String> {
        // Validate username
        if self.username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        
        if self.username.len() < 3 {
            return Err("Username must be at least 3 characters long".to_string());
        }
        
        if !self.username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("Username can only contain alphanumeric characters, underscores, and hyphens".to_string());
        }
        
        // Validate password
        if self.password.is_empty() {
            return Err("Password cannot be empty".to_string());
        }
        
        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }
        
        // Validate roles
        if self.roles.is_empty() {
            return Err("At least one role must be specified".to_string());
        }
        
        for role in &self.roles {
            if !roles::is_valid_role(&role.role) {
                return Err(format!("Invalid role: '{}'. Valid roles are: {:?}", 
                    role.role, roles::ALL_ROLES));
            }
            
            // Validate role-specific requirements
            if roles::is_data_access_role(&role.role) && role.bucket.is_none() {
                return Err(format!("Data access role '{}' requires a bucket to be specified", role.role));
            }
            
            if role.scope.is_some() && role.bucket.is_none() {
                return Err("Scope can only be specified when bucket is also specified".to_string());
            }
            
            if role.collection.is_some() && role.scope.is_none() {
                return Err("Collection can only be specified when scope is also specified".to_string());
            }
        }
        
        Ok(())
    }
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    pub role: String,
    pub bucket: Option<String>,
    pub scope: Option<String>,
    pub collection: Option<String>,
}

impl Role {
    #[allow(dead_code)] // Available for future use
    pub fn new(role: String, bucket: Option<String>, scope: Option<String>, collection: Option<String>) -> Self {
        Self {
            role,
            bucket,
            scope,
            collection,
        }
    }
    
    #[allow(dead_code)] // Available for future use
    pub fn is_console_access(&self) -> bool {
        matches!(self.role.as_str(), 
            "admin" | "cluster_admin" | "replication_admin" | "views_admin" | "query_manage"
        )
    }
    
    #[allow(dead_code)] // Available for future use
    pub fn is_bucket_specific(&self) -> bool {
        self.bucket.is_some()
    }
    
    #[allow(dead_code)] // Available for future use
    pub fn is_scope_specific(&self) -> bool {
        self.scope.is_some()
    }
    
    #[allow(dead_code)] // Available for future use
    pub fn is_collection_specific(&self) -> bool {
        self.collection.is_some()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub roles: Vec<Role>,
    pub groups: Vec<String>,
}

// RBAC Role Constants
pub mod roles {
    // Data Access Roles
    pub const DATA_READER: &str = "data_reader";
    pub const DATA_WRITER: &str = "data_writer";
    pub const DATA_DCP_READER: &str = "data_dcp_reader";
    pub const BUCKET_FULL_ACCESS: &str = "bucket_admin";
    
    // Administrative Roles (Console Access)
    pub const ADMIN: &str = "admin";
    pub const CLUSTER_ADMIN: &str = "cluster_admin";
    pub const REPLICATION_ADMIN: &str = "replication_admin";
    pub const VIEWS_ADMIN: &str = "views_admin";
    pub const QUERY_MANAGE: &str = "query_manage";
    
    // Query Roles
    pub const QUERY_SELECT: &str = "query_select";
    pub const QUERY_INSERT: &str = "query_insert";
    pub const QUERY_UPDATE: &str = "query_update";
    pub const QUERY_DELETE: &str = "query_delete";
    
    // Role Categories
    pub const CONSOLE_ACCESS_ROLES: &[&str] = &[
        ADMIN, CLUSTER_ADMIN, REPLICATION_ADMIN, VIEWS_ADMIN, QUERY_MANAGE
    ];
    
    pub const DATA_ACCESS_ROLES: &[&str] = &[
        DATA_READER, DATA_WRITER, DATA_DCP_READER, BUCKET_FULL_ACCESS
    ];
    
    pub const QUERY_ROLES: &[&str] = &[
        QUERY_SELECT, QUERY_INSERT, QUERY_UPDATE, QUERY_DELETE, QUERY_MANAGE
    ];
    
    pub const ALL_ROLES: &[&str] = &[
        DATA_READER, DATA_WRITER, DATA_DCP_READER, BUCKET_FULL_ACCESS,
        ADMIN, CLUSTER_ADMIN, REPLICATION_ADMIN, VIEWS_ADMIN,
        QUERY_SELECT, QUERY_INSERT, QUERY_UPDATE, QUERY_DELETE, QUERY_MANAGE
    ];
    
    pub fn is_valid_role(role: &str) -> bool {
        ALL_ROLES.contains(&role)
    }
    
    pub fn has_console_access(role: &str) -> bool {
        CONSOLE_ACCESS_ROLES.contains(&role)
    }
    
    pub fn is_data_access_role(role: &str) -> bool {
        DATA_ACCESS_ROLES.contains(&role)
    }
    
    pub fn is_query_role(role: &str) -> bool {
        QUERY_ROLES.contains(&role)
    }
}

// API Response Models
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

// Couchbase REST API Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CouchbaseBucketConfig {
    pub name: String,
    pub ram_quota_mb: u32,
    pub replica_number: u32,
    pub eviction_policy: String,
    pub compression_mode: String,
    pub conflict_resolution_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CouchbaseScopeConfig {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CouchbaseCollectionConfig {
    pub name: String,
    pub max_ttl: Option<u32>,
    pub history: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CouchbaseUserConfig {
    pub name: String,
    pub password: String,
    pub roles: Vec<CouchbaseRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CouchbaseRole {
    pub role: String,
    pub bucket_name: Option<String>,
    pub scope_name: Option<String>,
    pub collection_name: Option<String>,
}

// Health Check Models
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: String,
    pub timestamp: String,
    pub version: String,
    pub uptime: u64,
}

// Metrics Models
#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_error: u64,
    pub couchbase_operations_total: u64,
    pub couchbase_operations_success: u64,
    pub couchbase_operations_error: u64,
}
