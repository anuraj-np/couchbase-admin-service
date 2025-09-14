# 🚀 Quick Setup Guide for New Machine

This guide will help you set up the Couchbase Admin Service on a new machine without Cursor.

## 📋 Prerequisites

### For Local Deployment
- **Rust 1.75+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository
- **Couchbase Server**: Running locally or accessible

### For AWS EKS Deployment
- **Rust 1.75+**
- **Git**
- **AWS CLI**: Install from [AWS CLI docs](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
- **kubectl**: Install from [Kubernetes docs](https://kubernetes.io/docs/tasks/tools/)
- **Docker**: Install from [Docker docs](https://docs.docker.com/get-docker/)
- **EKS Cluster**: Running in AWS

## ⚡ Quick Start (5 minutes)

### 1. Clone and Setup
```bash
# Clone the repository
git clone <your-repository-url>
cd couchbase-admin-service

# Verify Rust installation
rustc --version
cargo --version
```

### 2. Configure Environment
```bash
# Copy environment template
cp env.example .env

# Edit with your Couchbase details
nano .env  # or vim, code, etc.
```

**Required .env settings:**
```env
COUCHBASE_HOST=http://your-couchbase-host:8091
COUCHBASE_USERNAME=Administrator
COUCHBASE_PASSWORD=your-password
AUTH_USERNAME=admin
AUTH_PASSWORD=your-secure-password
```

### 3. Deploy

#### Option A: Local Deployment
```bash
# Use the deployment script
./deploy.sh local

# Or manually
cargo build --release
cargo run
```

#### Option B: AWS EKS Deployment
```bash
# Set AWS environment variables
export AWS_REGION=us-west-2
export EKS_CLUSTER_NAME=your-cluster-name
export ECR_REPOSITORY=couchbase-admin-service

# Deploy to EKS
./deploy.sh eks
```

### 4. Test the Service
```bash
# Health check
curl http://localhost:8080/health

# Test with authentication
curl -H "Authorization: Basic $(echo -n 'admin:your-password' | base64)" \
  http://localhost:8080/buckets
```

## 🔧 Manual Setup (if scripts don't work)

### Local Deployment
```bash
# 1. Build the project
cargo build --release

# 2. Run the service
cargo run

# 3. Test
curl http://localhost:8080/health
```

### EKS Deployment
```bash
# 1. Build Docker image
docker build -t couchbase-admin-service .

# 2. Tag for ECR
docker tag couchbase-admin-service:latest \
  <account-id>.dkr.ecr.<region>.amazonaws.com/couchbase-admin-service:latest

# 3. Push to ECR
docker push <account-id>.dkr.ecr.<region>.amazonaws.com/couchbase-admin-service:latest

# 4. Deploy to Kubernetes
kubectl apply -f k8s/
```

## 🐛 Troubleshooting

### Common Issues

#### 1. "Address already in use"
```bash
# Find and kill the process
lsof -i :8080
kill -9 <PID>

# Or use the script
./deploy.sh local
```

#### 2. "Couchbase connection failed"
- Check Couchbase is running: `curl http://your-couchbase-host:8091/pools`
- Verify credentials in `.env`
- Check network connectivity

#### 3. "Permission denied" on deploy.sh
```bash
chmod +x deploy.sh
```

#### 4. EKS deployment fails
- Verify AWS credentials: `aws sts get-caller-identity`
- Check EKS cluster exists: `aws eks list-clusters`
- Verify kubectl context: `kubectl config current-context`

### Logs and Debugging

#### View Service Logs
```bash
# Local deployment
tail -f service.log

# Kubernetes deployment
kubectl logs -f deployment/couchbase-admin-service -n couchbase-admin

# Docker deployment
docker logs -f <container-id>
```

#### Check Service Status
```bash
# Local
curl http://localhost:8080/health

# Kubernetes
kubectl get pods -n couchbase-admin
kubectl get services -n couchbase-admin
```

## 📚 API Usage Examples

### Create a Bucket
```bash
curl -X POST http://localhost:8080/buckets \
  -H "Content-Type: application/json" \
  -H "Authorization: Basic $(echo -n 'admin:password' | base64)" \
  -d '{
    "bucket_name": "my-bucket",
    "ram_quota_mb": 100,
    "replica_number": 1,
    "eviction_policy": "valueOnly",
    "compression_mode": "passive",
    "conflict_resolution_type": "seqno"
  }'
```

### List Buckets
```bash
curl -H "Authorization: Basic $(echo -n 'admin:password' | base64)" \
  http://localhost:8080/buckets
```

### List Scopes
```bash
curl -H "Authorization: Basic $(echo -n 'admin:password' | base64)" \
  http://localhost:8080/buckets/my-bucket/scopes
```

### List Collections
```bash
curl -H "Authorization: Basic $(echo -n 'admin:password' | base64)" \
  http://localhost:8080/buckets/my-bucket/scopes/_default/collections
```

## 🔒 Security Notes

- Change default passwords in production
- Use environment variables for sensitive data
- Enable TLS/SSL in production
- Use proper RBAC for Kubernetes
- Consider using AWS Secrets Manager

## 📞 Support

If you encounter issues:
1. Check the logs first
2. Verify all prerequisites are installed
3. Ensure Couchbase is accessible
4. Check the README.md for detailed documentation
5. Review the troubleshooting section above

## 🎯 Next Steps

After successful deployment:
1. Test all API endpoints
2. Set up monitoring (Prometheus/Grafana)
3. Configure alerts
4. Set up CI/CD pipeline
5. Plan for production scaling
