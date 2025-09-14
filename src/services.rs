use crate::{
    config::Config,
    error::{AppError, Result},
    models::{
        BucketInfo, CollectionInfo, CouchbaseBucketConfig, CouchbaseUserConfig, Role, ScopeInfo, UserInfo,
    },
};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct CouchbaseService {
    client: Client,
    base_url: String,
    username: String,
    password: String,
}

impl CouchbaseService {
    pub fn new(config: &Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.couchbase.timeout_seconds))
            .build()?;

        Ok(Self {
            client,
            base_url: config.couchbase.host.clone(),
            username: config.couchbase.username.clone(),
            password: config.couchbase.password.clone(),
        })
    }

    // Bucket Management
    pub async fn create_bucket(&self, request: &CouchbaseBucketConfig) -> Result<()> {
        let url = format!("{}/pools/default/buckets", self.base_url);
        
        let params = [
            ("name", request.name.as_str()),
            ("ramQuotaMB", &request.ram_quota_mb.to_string()),
            ("replicaNumber", &request.replica_number.to_string()),
            ("evictionPolicy", request.eviction_policy.as_str()),
            ("compressionMode", request.compression_mode.as_str()),
            ("conflictResolutionType", request.conflict_resolution_type.as_str()),
        ];

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        Ok(())
    }

    pub async fn list_buckets(&self) -> Result<Vec<BucketInfo>> {
        let url = format!("{}/pools/default/buckets", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        let buckets: Vec<serde_json::Value> = response.json().await?;
        let bucket_infos: Result<Vec<BucketInfo>> = buckets
            .into_iter()
            .map(|bucket| {
                Ok(BucketInfo {
                    name: bucket["name"].as_str().unwrap_or("").to_string(),
                    ram_quota_mb: bucket["quota"]["ram"].as_u64().unwrap_or(0) as u32,
                    replica_number: bucket["replicaNumber"].as_u64().unwrap_or(0) as u32,
                    eviction_policy: bucket["evictionPolicy"].as_str().unwrap_or("").to_string(),
                    compression_mode: bucket["compressionMode"].as_str().unwrap_or("").to_string(),
                    conflict_resolution_type: bucket["conflictResolutionType"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    status: bucket["status"].as_str().unwrap_or("").to_string(),
                })
            })
            .collect();

        bucket_infos
    }

    // Scope Management
    pub async fn create_scope(&self, bucket_name: &str, scope_name: &str) -> Result<()> {
        let url = format!("{}/pools/default/buckets/{}/scopes", self.base_url, bucket_name);
        
        let params: Vec<(String, String)> = vec![
            ("name".to_string(), scope_name.to_string()),
        ];

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        Ok(())
    }

    pub async fn list_scopes(&self, bucket_name: &str) -> Result<Vec<ScopeInfo>> {
        let url = format!("{}/pools/default/buckets/{}/scopes", self.base_url, bucket_name);
        
        let response = self
            .client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        let scopes_data: serde_json::Value = response.json().await?;
        let empty_vec = vec![];
        let scopes_array = scopes_data["scopes"].as_array().unwrap_or(&empty_vec);
        
        let scope_infos: Result<Vec<ScopeInfo>> = scopes_array
            .iter()
            .map(|scope| {
                let empty_collections = vec![];
                let collections = scope["collections"]
                    .as_array()
                    .unwrap_or(&empty_collections)
                    .iter()
                    .map(|collection| CollectionInfo {
                        name: collection["name"].as_str().unwrap_or("").to_string(),
                        max_ttl: collection["maxTTL"].as_u64().map(|v| v as u32),
                        history: collection["history"].as_bool(),
                        scope: scope["name"].as_str().unwrap_or("").to_string(),
                    })
                    .collect();

                Ok(ScopeInfo {
                    name: scope["name"].as_str().unwrap_or("").to_string(),
                    collections,
                })
            })
            .collect();

        scope_infos
    }

    // Collection Management
    pub async fn create_collection(
        &self,
        bucket_name: &str,
        scope_name: &str,
        collection_name: &str,
        max_ttl: Option<u32>,
        history: Option<bool>,
    ) -> Result<()> {
        let url = format!(
            "{}/pools/default/buckets/{}/scopes/{}/collections",
            self.base_url, bucket_name, scope_name
        );
        
        let mut params: Vec<(String, String)> = vec![
            ("name".to_string(), collection_name.to_string()),
        ];

        if let Some(ttl) = max_ttl {
            params.push(("maxTTL".to_string(), ttl.to_string()));
        }

        if let Some(hist) = history {
            params.push(("history".to_string(), hist.to_string()));
        }

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        Ok(())
    }

    pub async fn list_collections(&self, bucket_name: &str, scope_name: &str) -> Result<Vec<CollectionInfo>> {
        let scopes = self.list_scopes(bucket_name).await?;
        
        let scope = scopes
            .into_iter()
            .find(|s| s.name == scope_name)
            .ok_or_else(|| AppError::NotFound(format!("Scope '{}' not found", scope_name)))?;

        Ok(scope.collections)
    }

    // User Management
    pub async fn create_user(&self, request: &CouchbaseUserConfig) -> Result<()> {
        let url = format!("{}/settings/rbac/users/local/{}", self.base_url, request.name);
        
        // Convert roles to form-encoded format
        let mut params: Vec<(String, String)> = vec![
            ("name".to_string(), request.name.clone()),
            ("password".to_string(), request.password.clone()),
        ];

        // Add roles - Couchbase expects roles as separate parameters
        // We need to send all roles in a single "roles" parameter as a comma-separated string
        let mut role_strings = Vec::new();
        for role in &request.roles {
            if role.bucket_name.is_none() && role.scope_name.is_none() && role.collection_name.is_none() {
                // Global role
                role_strings.push(role.role.clone());
            } else {
                // Bucket/scope/collection specific role - format as "role[bucket:scope:collection]"
                let role_spec = if let Some(bucket) = &role.bucket_name {
                    let mut spec = format!("{}[{}", role.role, bucket);
                    if let Some(scope) = &role.scope_name {
                        spec.push_str(&format!(":{}", scope));
                        if let Some(collection) = &role.collection_name {
                            spec.push_str(&format!(":{}", collection));
                        }
                    }
                    spec.push(']');
                    spec
                } else {
                    role.role.clone()
                };
                role_strings.push(role_spec);
            }
        }
        
        // Join all roles with comma separation
        if !role_strings.is_empty() {
            params.push(("roles".to_string(), role_strings.join(",")));
        }

        let response = self
            .client
            .put(&url)
            .basic_auth(&self.username, Some(&self.password))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        Ok(())
    }

    pub async fn list_users(&self) -> Result<Vec<UserInfo>> {
        let url = format!("{}/settings/rbac/users", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        let users_data: Vec<serde_json::Value> = response.json().await?;
        let user_infos: Result<Vec<UserInfo>> = users_data
            .into_iter()
            .map(|user| {
                let roles: Result<Vec<Role>> = user["roles"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|role| {
                        Ok(Role {
                            role: role["role"].as_str().unwrap_or("").to_string(),
                            bucket: role["bucket_name"].as_str().map(|s| s.to_string()),
                            scope: role["scope_name"].as_str().map(|s| s.to_string()),
                            collection: role["collection_name"].as_str().map(|s| s.to_string()),
                        })
                    })
                    .collect();

                Ok(UserInfo {
                    username: user["id"].as_str().unwrap_or("").to_string(),
                    roles: roles?,
                    groups: user["groups"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|g| g.as_str().unwrap_or("").to_string())
                        .collect(),
                })
            })
            .collect();

        user_infos
    }

    pub async fn get_user(&self, username: &str) -> Result<UserInfo> {
        let url = format!("{}/settings/rbac/users/local/{}", self.base_url, username);
        
        let response = self
            .client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            if status == 404 {
                return Err(AppError::NotFound(format!("User '{}' not found", username)));
            }
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        let user: serde_json::Value = response.json().await?;
        
        // Debug: Print the raw response to understand the structure
        println!("Raw Couchbase API response for user {}: {}", username, serde_json::to_string_pretty(&user).unwrap_or_else(|_| "Failed to serialize".to_string()));
        
        let roles: Result<Vec<Role>> = user["roles"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|role| {
                // Handle different role formats that Couchbase might return
                if role.is_string() {
                    // Simple string role (e.g., "cluster_admin")
                    Ok(Role {
                        role: role.as_str().unwrap_or("").to_string(),
                        bucket: None,
                        scope: None,
                        collection: None,
                    })
                } else if role.is_object() {
                    // Object role with bucket/scope/collection info
                    Ok(Role {
                        role: role["role"].as_str().unwrap_or("").to_string(),
                        bucket: role["bucket_name"].as_str().map(|s| s.to_string()),
                        scope: role["scope_name"].as_str().map(|s| s.to_string()),
                        collection: role["collection_name"].as_str().map(|s| s.to_string()),
                    })
                } else {
                    // Fallback - try to extract role name from any field
                    let role_str = role["role"].as_str()
                        .or_else(|| role["name"].as_str())
                        .or_else(|| role.as_str())
                        .unwrap_or("unknown");
                    
                    Ok(Role {
                        role: role_str.to_string(),
                        bucket: role["bucket_name"].as_str().map(|s| s.to_string()),
                        scope: role["scope_name"].as_str().map(|s| s.to_string()),
                        collection: role["collection_name"].as_str().map(|s| s.to_string()),
                    })
                }
            })
            .collect();

        Ok(UserInfo {
            username: user["id"].as_str().unwrap_or("").to_string(),
            roles: roles?,
            groups: user["groups"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|g| g.as_str().unwrap_or("").to_string())
                .collect(),
        })
    }

    pub async fn delete_user(&self, username: &str) -> Result<()> {
        let url = format!("{}/settings/rbac/users/local/{}", self.base_url, username);
        
        let response = self
            .client
            .delete(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            if status == 404 {
                return Err(AppError::NotFound(format!("User '{}' not found", username)));
            }
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        Ok(())
    }

    pub async fn update_user(&self, request: &CouchbaseUserConfig) -> Result<()> {
        let url = format!("{}/settings/rbac/users/local/{}", self.base_url, request.name);
        
        let mut params: Vec<(String, String)> = vec![
            ("name".to_string(), request.name.clone()),
            ("password".to_string(), request.password.clone()),
        ];

        // Add roles - Couchbase expects simple role names for basic roles
        for role in &request.roles {
            if role.bucket_name.is_none() && role.scope_name.is_none() && role.collection_name.is_none() {
                // Global role
                params.push(("roles".to_string(), role.role.clone()));
            } else {
                // Bucket/scope/collection specific role - format as "role[bucket:scope:collection]"
                let role_spec = if let Some(bucket) = &role.bucket_name {
                    let mut spec = format!("{}[{}", role.role, bucket);
                    if let Some(scope) = &role.scope_name {
                        spec.push_str(&format!(":{}", scope));
                        if let Some(collection) = &role.collection_name {
                            spec.push_str(&format!(":{}", collection));
                        }
                    }
                    spec.push(']');
                    spec
                } else {
                    role.role.clone()
                };
                params.push(("roles".to_string(), role_spec));
            }
        }

        let response = self
            .client
            .put(&url)
            .basic_auth(&self.username, Some(&self.password))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            if status == 404 {
                return Err(AppError::NotFound(format!("User '{}' not found", request.name)));
            }
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::CouchbaseApi {
                message: error_text,
                status,
            });
        }

        Ok(())
    }
}
