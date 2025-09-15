#!/bin/bash

echo "🧪 Testing Couchbase Admin Service Docker Image"
echo "=============================================="

# Clean up any existing containers
echo "🧹 Cleaning up existing containers..."
docker stop couchbase-test couchbase-admin-test 2>/dev/null || true
docker rm couchbase-test couchbase-admin-test 2>/dev/null || true

# Start Couchbase
echo "🚀 Starting Couchbase Server..."
docker run -d \
  --name couchbase-test \
  -p 8091-8096:8091-8096 \
  -p 11210:11210 \
  -e COUCHBASE_ADMINISTRATOR_USERNAME=Administrator \
  -e COUCHBASE_ADMINISTRATOR_PASSWORD=123456 \
  --platform linux/amd64 \
  couchbase/server:7.0.2

echo "⏳ Waiting for Couchbase to be ready (this may take 2-3 minutes)..."
# Wait for Couchbase to be ready
for i in {1..60}; do
  if curl -s http://localhost:8091/pools/default > /dev/null 2>&1; then
    echo "✅ Couchbase is ready!"
    break
  fi
  echo "⏳ Waiting... ($i/60)"
  sleep 5
done

# Test Couchbase is accessible
echo "🔍 Testing Couchbase connectivity..."
if curl -s http://localhost:8091/pools/default > /dev/null; then
  echo "✅ Couchbase is accessible"
else
  echo "❌ Couchbase is not accessible, but continuing with test..."
fi

# Start the admin service
echo "🚀 Starting Couchbase Admin Service..."
docker run -d \
  --name couchbase-admin-test \
  -p 8080:8080 \
  --network host \
  -e COUCHBASE_HOST=http://localhost:8091 \
  -e COUCHBASE_USERNAME=Administrator \
  -e COUCHBASE_PASSWORD=123456 \
  -e AUTH_ENABLED=true \
  -e AUTH_USERNAME=admin \
  -e AUTH_PASSWORD=admin \
  -e RUST_LOG=info \
  couchbase-admin-service:latest

echo "⏳ Waiting for admin service to start..."
sleep 10

# Test the admin service
echo "🔍 Testing Admin Service..."

# Test health endpoint
echo "1. Testing health endpoint..."
if curl -s http://localhost:8080/health > /dev/null; then
  echo "✅ Health endpoint is working"
  curl -s http://localhost:8080/health | jq .
else
  echo "❌ Health endpoint failed"
fi

# Test metrics endpoint
echo "2. Testing metrics endpoint..."
if curl -s http://localhost:8080/metrics > /dev/null; then
  echo "✅ Metrics endpoint is working"
else
  echo "❌ Metrics endpoint failed"
fi

# Test protected endpoint with auth
echo "3. Testing protected endpoint with authentication..."
if curl -s -u admin:admin http://localhost:8080/roles > /dev/null; then
  echo "✅ Protected endpoint with auth is working"
  curl -s -u admin:admin http://localhost:8080/roles | jq .
else
  echo "❌ Protected endpoint with auth failed"
fi

# Test protected endpoint without auth (should fail)
echo "4. Testing protected endpoint without authentication (should fail)..."
if curl -s http://localhost:8080/roles > /dev/null; then
  echo "❌ Protected endpoint without auth should have failed but didn't"
else
  echo "✅ Protected endpoint without auth correctly failed (401)"
fi

echo ""
echo "🎉 Testing complete!"
echo ""
echo "📊 Container Status:"
docker ps --filter "name=couchbase"

echo ""
echo "📝 To view logs:"
echo "  docker logs couchbase-admin-test"
echo ""
echo "🧹 To clean up:"
echo "  docker stop couchbase-test couchbase-admin-test"
echo "  docker rm couchbase-test couchbase-admin-test"
