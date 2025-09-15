#!/bin/bash

echo "🧪 Jenkins Pipeline Test Script"
echo "==============================="

# Test if Jenkins is running
echo "1. Testing Jenkins connectivity..."
if curl -s http://localhost:8081/login > /dev/null; then
    echo "✅ Jenkins is accessible"
else
    echo "❌ Jenkins is not accessible"
    exit 1
fi

# Test if Couchbase is running
echo "2. Testing Couchbase connectivity..."
if curl -s http://localhost:8091/pools/default > /dev/null; then
    echo "✅ Couchbase is accessible"
else
    echo "❌ Couchbase is not accessible"
    exit 1
fi

# Test Docker image build
echo "3. Testing Docker image build..."
if docker build -t test-couchbase-admin . > /dev/null 2>&1; then
    echo "✅ Docker image builds successfully"
else
    echo "❌ Docker image build failed"
    exit 1
fi

# Test Docker image run
echo "4. Testing Docker image execution..."
docker run -d --name test-container \
    -e COUCHBASE_HOST=http://localhost:8091 \
    -e COUCHBASE_USERNAME=Administrator \
    -e COUCHBASE_PASSWORD=123456 \
    -e AUTH_ENABLED=true \
    -e AUTH_USERNAME=admin \
    -e AUTH_PASSWORD=admin \
    -e RUST_LOG=info \
    test-couchbase-admin

sleep 10

if docker ps | grep test-container; then
    echo "✅ Docker container started successfully"
    
    # Test health endpoint
    if curl -f http://localhost:8080/health > /dev/null 2>&1; then
        echo "✅ Health endpoint working"
    else
        echo "❌ Health endpoint failed"
    fi
    
    # Clean up
    docker stop test-container
    docker rm test-container
else
    echo "❌ Docker container failed to start"
    docker logs test-container
    docker rm test-container
    exit 1
fi

echo ""
echo "🎉 All tests passed! Jenkins pipeline should work correctly."
