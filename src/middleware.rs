use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use base64::{engine::general_purpose, Engine as _};
use std::str;

use crate::config::Config;

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Get config from extensions or create a default one
    let config = request
        .extensions()
        .get::<Config>()
        .cloned()
        .unwrap_or_else(|| Config {
            server: crate::config::ServerConfig {
                port: 8080,
                host: "0.0.0.0".to_string(),
            },
            couchbase: crate::config::CouchbaseConfig {
                host: "http://localhost:8091".to_string(),
                username: "Administrator".to_string(),
                password: "password".to_string(),
                timeout_seconds: 30,
            },
            auth: crate::config::AuthConfig {
                enabled: true,
                username: "admin".to_string(),
                password: "admin".to_string(),
            },
        });

    // Skip auth for health check and metrics endpoints
    let path = request.uri().path();
    if path == "/health" || path == "/metrics" {
        return Ok(next.run(request).await);
    }

    if !config.auth.enabled {
        return Ok(next.run(request).await);
    }

    // Extract Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

    // Check if it's Basic auth
    if !auth_header.starts_with("Basic ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid authorization type".to_string()));
    }

    // Decode base64 credentials
    let encoded_credentials = &auth_header[6..]; // Remove "Basic " prefix
    let decoded_credentials = general_purpose::STANDARD
        .decode(encoded_credentials)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid base64 encoding".to_string()))?;

    let credentials = str::from_utf8(&decoded_credentials)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid UTF-8 in credentials".to_string()))?;

    // Parse username:password
    let parts: Vec<&str> = credentials.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials format".to_string()));
    }

    let (username, password) = (parts[0], parts[1]);

    // Validate credentials
    if username != config.auth.username || password != config.auth.password {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Add user info to request extensions for use in handlers
    request.extensions_mut().insert(UserInfo {
        username: username.to_string(),
    });

    Ok(next.run(request).await)
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    #[allow(dead_code)] // Reserved for future use
    pub username: String,
}
