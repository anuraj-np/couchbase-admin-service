# Couchbase Admin Service

A production-ready Rust microservice for managing Couchbase clusters, providing REST APIs for bucket, scope, collection, and user management with RBAC support.

## 🚀 Features

- **Bucket Management**: Create and list Couchbase buckets
- **Scope Management**: Create and manage scopes within buckets
- **Collection Management**: Create and manage collections within scopes
- **User Management**: Create, list, and manage users with RBAC roles
- **REST API**: Clean, RESTful API design with proper HTTP status codes
- **Authentication**: Basic authentication middleware for API security
- **Observability**: Prometheus metrics and structured logging
- **CI/CD**: Jenkins pipeline for automated deployment
- **Cloud Ready**: Docker containerization and AWS deployment support

## 🛠️ Tech Stack

- **Language**: Rust (stable, edition 2021)
- **Web Framework**: Axum (async, modern)
- **HTTP Client**: reqwest (for Couchbase REST API calls)
- **Async Runtime**: Tokio
- **Configuration**: dotenv + config crate
- **Error Handling**: thiserror + anyhow
- **Logging**: tracing + tracing-subscriber
- **Metrics**: Prometheus
- **Authentication**: Basic Auth (extensible to JWT/OAuth)
- **Containerization**: Docker (multi-stage build)
- **CI/CD**: Jenkins with parameterized pipelines

## 📋 Prerequisites

- Rust 1.70+ (stable)
- Docker & Docker Compose
- Couchbase Server 7.0+
- Make (optional, for convenience scripts)

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/couchbase-admin-service.git
cd couchbase-admin-service
```

### 2. Environment Setup

```bash
cp env.example .env
# Edit .env with your Couchbase connection details
```

### 3. Run with Docker Compose

```bash
docker-compose up -d
```

### 4. Run Locally (Development)

```bash
# Install dependencies
cargo build

# Run the service
cargo run
```

## 📚 API Documentation

### Authentication

All API endpoints require Basic Authentication. Use the admin credentials configured in your environment.

### Core Endpoints

#### Bucket Management
- `POST /buckets` - Create a new bucket
- `GET /buckets` - List all buckets

#### Scope Management
- `POST /buckets/{bucket}/scopes` - Create a new scope
- `GET /buckets/{bucket}/scopes` - List scopes in a bucket

#### Collection Management
- `POST /buckets/{bucket}/scopes/{scope}/collections` - Create a new collection
- `GET /buckets/{bucket}/scopes/{scope}/collections` - List collections in a scope

#### User Management
- `POST /users` - Create a new user
- `GET /users` - List all users
- `GET /users/{username}` - Get user details
- `PUT /users/{username}/roles` - Update user roles
- `DELETE /users/{username}` - Delete a user
- `GET /users/{username}/permissions` - Get user permissions
- `GET /roles` - List available roles

### Example: Create a User with Restricted Access

```bash
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -H "Authorization: Basic YWRtaW46YWRtaW4=" \
  -d '{
    "username": "restricted-user",
    "password": "SecurePassword123!",
    "display_name": "Restricted User",
    "email": "restricted@example.com",
    "roles": [
      {
        "role": "cluster_admin"
      },
      {
        "role": "data_reader",
        "bucket": "DigitalFlightShopping",
        "scope": "Test",
        "collection": "Test"
      }
    ]
  }'
```

## 🔧 Configuration

The service uses environment variables for configuration. See `env.example` for all available options:

```bash
# Couchbase Configuration
COUCHBASE_HOST=localhost
COUCHBASE_PORT=8091
COUCHBASE_USERNAME=admin
COUCHBASE_PASSWORD=admin

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Logging
RUST_LOG=info
```

## 🐳 Docker Deployment

### Build Image

```bash
docker build -t couchbase-admin-service .
```

### Run Container

```bash
docker run -d \
  --name couchbase-admin \
  -p 8080:8080 \
  -e COUCHBASE_HOST=your-couchbase-host \
  -e COUCHBASE_USERNAME=admin \
  -e COUCHBASE_PASSWORD=admin \
  couchbase-admin-service
```

## ☸️ Kubernetes Deployment

See the `k8s/` directory for Kubernetes manifests:

```bash
kubectl apply -f k8s/
```

## 📊 Monitoring

The service exposes Prometheus metrics at `/metrics`:

```bash
curl http://localhost:8080/metrics
```

## 🧪 Testing

Run the test suite:

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# API tests
./test-api.sh
```

## 🚀 CI/CD

The project includes a Jenkins pipeline (`Jenkinsfile`) for automated:

- Code compilation and testing
- Docker image building
- Security scanning
- Deployment to AWS ECS/EKS

## 📝 Development

### Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── error.rs             # Error handling
├── middleware.rs        # Authentication middleware
├── models.rs            # Data models and DTOs
├── routes/              # API route handlers
│   ├── buckets.rs
│   ├── scopes.rs
│   ├── collections.rs
│   └── users.rs
└── services.rs          # Couchbase service integration
```

### Adding New Features

1. Define models in `src/models.rs`
2. Implement service logic in `src/services.rs`
3. Create route handlers in `src/routes/`
4. Add routes to `src/main.rs`
5. Write tests and update documentation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Modern web framework for Rust
- [Couchbase](https://www.couchbase.com/) - NoSQL database platform
- [Tokio](https://tokio.rs/) - Async runtime for Rust

## 📞 Support

For support and questions:

- Create an issue in this repository
- Check the [documentation](docs/)
- Review the [API examples](examples/)

---

**Note**: This service is designed for production use with proper security measures. Ensure you configure appropriate authentication and network security for your deployment environment.
