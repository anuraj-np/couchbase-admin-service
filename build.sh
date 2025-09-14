#!/bin/bash

# Couchbase Admin Service - Build and Package Script
# This script compiles the Rust service and creates various distribution packages

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

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --debug          Build debug version (default)"
    echo "  --release        Build optimized release version"
    echo "  --docker         Build Docker image"
    echo "  --all            Build all versions (debug, release, docker)"
    echo "  --clean          Clean build artifacts before building"
    echo "  --test           Run tests after building"
    echo "  --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --release --test"
    echo "  $0 --docker"
    echo "  $0 --all --clean"
}

# Default values
BUILD_DEBUG=false
BUILD_RELEASE=false
BUILD_DOCKER=false
CLEAN=false
TEST=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            BUILD_DEBUG=true
            shift
            ;;
        --release)
            BUILD_RELEASE=true
            shift
            ;;
        --docker)
            BUILD_DOCKER=true
            shift
            ;;
        --all)
            BUILD_DEBUG=true
            BUILD_RELEASE=true
            BUILD_DOCKER=true
            shift
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        --test)
            TEST=true
            shift
            ;;
        --help)
            show_usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# If no build type specified, default to debug
if [ "$BUILD_DEBUG" = false ] && [ "$BUILD_RELEASE" = false ] && [ "$BUILD_DOCKER" = false ]; then
    BUILD_DEBUG=true
fi

echo -e "${BLUE}=== Couchbase Admin Service Build Script ===${NC}"
echo

# Clean if requested
if [ "$CLEAN" = true ]; then
    print_info "Cleaning build artifacts..."
    cargo clean
    print_status $? "Clean completed"
    echo
fi

# Build debug version
if [ "$BUILD_DEBUG" = true ]; then
    print_info "Building debug version..."
    cargo build
    print_status $? "Debug build completed"
    
    if [ -f "target/debug/couchbase-admin-service" ]; then
        echo "  Binary: target/debug/couchbase-admin-service"
        ls -lh target/debug/couchbase-admin-service
    fi
    echo
fi

# Build release version
if [ "$BUILD_RELEASE" = true ]; then
    print_info "Building release version..."
    cargo build --release
    print_status $? "Release build completed"
    
    if [ -f "target/release/couchbase-admin-service" ]; then
        echo "  Binary: target/release/couchbase-admin-service"
        ls -lh target/release/couchbase-admin-service
    fi
    echo
fi

# Build Docker image
if [ "$BUILD_DOCKER" = true ]; then
    print_info "Building Docker image..."
    docker build -t couchbase-admin-service:latest .
    print_status $? "Docker build completed"
    
    echo "  Image: couchbase-admin-service:latest"
    docker images couchbase-admin-service:latest
    echo
fi

# Run tests if requested
if [ "$TEST" = true ]; then
    print_info "Running tests..."
    cargo test
    print_status $? "Tests completed"
    echo
fi

# Show summary
echo -e "${GREEN}=== Build Summary ===${NC}"
if [ "$BUILD_DEBUG" = true ]; then
    if [ -f "target/debug/couchbase-admin-service" ]; then
        print_status 0 "Debug binary ready"
    else
        print_status 1 "Debug binary not found"
    fi
fi

if [ "$BUILD_RELEASE" = true ]; then
    if [ -f "target/release/couchbase-admin-service" ]; then
        print_status 0 "Release binary ready"
    else
        print_status 1 "Release binary not found"
    fi
fi

if [ "$BUILD_DOCKER" = true ]; then
    if docker images couchbase-admin-service:latest | grep -q couchbase-admin-service; then
        print_status 0 "Docker image ready"
    else
        print_status 1 "Docker image not found"
    fi
fi

echo
print_info "Build completed successfully!"
