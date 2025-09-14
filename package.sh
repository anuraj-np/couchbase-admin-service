#!/bin/bash

# Couchbase Admin Service - Package Script
# Creates distribution packages for different platforms and deployment methods

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ $2${NC}"
    else
        echo -e "${RED}✗ $2${NC}"
    fi
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | cut -d'"' -f2)
PACKAGE_NAME="couchbase-admin-service-${VERSION}"

echo -e "${BLUE}=== Couchbase Admin Service Package Script ===${NC}"
echo "Version: $VERSION"
echo "Package: $PACKAGE_NAME"
echo

# Create distribution directory
DIST_DIR="dist"
mkdir -p "$DIST_DIR"

# Clean previous packages
print_info "Cleaning previous packages..."
rm -rf "$DIST_DIR"/*
print_status $? "Clean completed"

# Build release binary
print_info "Building release binary..."
cargo build --release
print_status $? "Release build completed"

# Create binary package
print_info "Creating binary package..."
BINARY_PACKAGE="$DIST_DIR/$PACKAGE_NAME-binary"
mkdir -p "$BINARY_PACKAGE"

# Copy binary
cp target/release/couchbase-admin-service "$BINARY_PACKAGE/"

# Copy configuration files
cp env.example "$BINARY_PACKAGE/.env.example"
cp README.md "$BINARY_PACKAGE/"
cp SETUP_GUIDE.md "$BINARY_PACKAGE/"

# Copy deployment files
mkdir -p "$BINARY_PACKAGE/k8s"
cp k8s/*.yaml "$BINARY_PACKAGE/k8s/"

# Copy scripts
cp test-service.sh "$BINARY_PACKAGE/"
cp test-user-management.sh "$BINARY_PACKAGE/"
cp test-complete-workflow.sh "$BINARY_PACKAGE/"
cp deploy.sh "$BINARY_PACKAGE/"

# Create run script
cat > "$BINARY_PACKAGE/run.sh" << 'EOF'
#!/bin/bash
# Couchbase Admin Service Runner Script

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "Error: .env file not found. Please copy .env.example to .env and configure it."
    exit 1
fi

# Run the service
./couchbase-admin-service
EOF

chmod +x "$BINARY_PACKAGE/run.sh"

# Create tarball
print_info "Creating binary tarball..."
tar -czf "$DIST_DIR/$PACKAGE_NAME-binary.tar.gz" -C "$DIST_DIR" "$PACKAGE_NAME-binary"
print_status $? "Binary tarball created"

# Create Docker package
print_info "Creating Docker package..."
DOCKER_PACKAGE="$DIST_DIR/$PACKAGE_NAME-docker"
mkdir -p "$DOCKER_PACKAGE"

# Copy Docker files
cp Dockerfile "$DOCKER_PACKAGE/"
cp docker-compose.yml "$DOCKER_PACKAGE/"
cp .dockerignore "$DOCKER_PACKAGE/"

# Copy configuration
cp env.example "$DOCKER_PACKAGE/.env.example"

# Copy deployment files
mkdir -p "$DOCKER_PACKAGE/k8s"
cp k8s/*.yaml "$DOCKER_PACKAGE/k8s/"

# Copy documentation
cp README.md "$DOCKER_PACKAGE/"
cp SETUP_GUIDE.md "$DOCKER_PACKAGE/"

# Create Docker run script
cat > "$DOCKER_PACKAGE/run-docker.sh" << 'EOF'
#!/bin/bash
# Docker Run Script for Couchbase Admin Service

# Build the image
echo "Building Docker image..."
docker build -t couchbase-admin-service:latest .

# Run the container
echo "Starting container..."
docker run -d \
  --name couchbase-admin-service \
  -p 8080:8080 \
  --env-file .env \
  couchbase-admin-service:latest

echo "Service started. Check logs with: docker logs couchbase-admin-service"
EOF

chmod +x "$DOCKER_PACKAGE/run-docker.sh"

# Create Docker tarball
print_info "Creating Docker tarball..."
tar -czf "$DIST_DIR/$PACKAGE_NAME-docker.tar.gz" -C "$DIST_DIR" "$PACKAGE_NAME-docker"
print_status $? "Docker tarball created"

# Create Kubernetes package
print_info "Creating Kubernetes package..."
K8S_PACKAGE="$DIST_DIR/$PACKAGE_NAME-k8s"
mkdir -p "$K8S_PACKAGE"

# Copy Kubernetes manifests
cp k8s/*.yaml "$K8S_PACKAGE/"

# Copy configuration
cp env.example "$K8S_PACKAGE/.env.example"

# Copy documentation
cp README.md "$K8S_PACKAGE/"
cp SETUP_GUIDE.md "$K8S_PACKAGE/"

# Create deployment script
cat > "$K8S_PACKAGE/deploy-k8s.sh" << 'EOF'
#!/bin/bash
# Kubernetes Deployment Script

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "Error: kubectl is not installed or not in PATH"
    exit 1
fi

# Apply Kubernetes manifests
echo "Applying Kubernetes manifests..."
kubectl apply -f namespace.yaml
kubectl apply -f configmap.yaml
kubectl apply -f secret.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
kubectl apply -f ingress.yaml

echo "Deployment completed. Check status with: kubectl get pods -n couchbase-admin"
EOF

chmod +x "$K8S_PACKAGE/deploy-k8s.sh"

# Create Kubernetes tarball
print_info "Creating Kubernetes tarball..."
tar -czf "$DIST_DIR/$PACKAGE_NAME-k8s.tar.gz" -C "$DIST_DIR" "$PACKAGE_NAME-k8s"
print_status $? "Kubernetes tarball created"

# Create source package
print_info "Creating source package..."
SOURCE_PACKAGE="$DIST_DIR/$PACKAGE_NAME-source"
mkdir -p "$SOURCE_PACKAGE"

# Copy source files
cp -r src "$SOURCE_PACKAGE/"
cp Cargo.toml "$SOURCE_PACKAGE/"
cp Cargo.lock "$SOURCE_PACKAGE/"

# Copy configuration and documentation
cp env.example "$SOURCE_PACKAGE/"
cp README.md "$SOURCE_PACKAGE/"
cp SETUP_GUIDE.md "$SOURCE_PACKAGE/"

# Copy build scripts
cp build.sh "$SOURCE_PACKAGE/"
cp package.sh "$SOURCE_PACKAGE/"

# Copy deployment files
cp -r k8s "$SOURCE_PACKAGE/"
cp Dockerfile "$SOURCE_PACKAGE/"
cp docker-compose.yml "$SOURCE_PACKAGE/"
cp .dockerignore "$SOURCE_PACKAGE/"

# Copy test scripts
cp test-*.sh "$SOURCE_PACKAGE/"
cp deploy.sh "$SOURCE_PACKAGE/"

# Create source tarball
print_info "Creating source tarball..."
tar -czf "$DIST_DIR/$PACKAGE_NAME-source.tar.gz" -C "$DIST_DIR" "$PACKAGE_NAME-source"
print_status $? "Source tarball created"

# Show package summary
echo
echo -e "${GREEN}=== Package Summary ===${NC}"
echo "Distribution directory: $DIST_DIR"
echo
echo "Created packages:"
ls -lh "$DIST_DIR"/*.tar.gz | while read line; do
    echo "  $line"
done

echo
echo "Package contents:"
echo "  Binary Package: Ready-to-run binary with configuration"
echo "  Docker Package: Docker image and compose files"
echo "  Kubernetes Package: K8s manifests and deployment scripts"
echo "  Source Package: Complete source code and build scripts"

echo
print_info "Packaging completed successfully!"
echo "All packages are available in the 'dist' directory."
