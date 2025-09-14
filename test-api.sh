#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 Couchbase Admin Service API Test${NC}"
echo "=================================="

# Base URL
BASE_URL="http://localhost:8080"

# Create base64 encoded credentials (admin:admin)
AUTH_HEADER="Authorization: Basic $(echo -n 'admin:admin' | base64)"

echo -e "\n${YELLOW}1. Testing Health Check (No Auth Required)${NC}"
echo "GET $BASE_URL/health"
curl -s "$BASE_URL/health" | jq . || echo "Service not running or error occurred"

echo -e "\n${YELLOW}2. Testing Metrics Endpoint (No Auth Required)${NC}"
echo "GET $BASE_URL/metrics"
curl -s "$BASE_URL/metrics" | head -5 || echo "Service not running or error occurred"

echo -e "\n${YELLOW}3. Testing Bucket List (Auth Required)${NC}"
echo "GET $BASE_URL/buckets"
curl -s -H "$AUTH_HEADER" "$BASE_URL/buckets" | jq . || echo "Service not running or error occurred"

echo -e "\n${YELLOW}4. Testing User List (Auth Required)${NC}"
echo "GET $BASE_URL/users"
curl -s -H "$AUTH_HEADER" "$BASE_URL/users" | jq . || echo "Service not running or error occurred"

echo -e "\n${YELLOW}5. Testing Create Bucket (Auth Required)${NC}"
echo "POST $BASE_URL/buckets"
curl -s -X POST \
  -H "Content-Type: application/json" \
  -H "$AUTH_HEADER" \
  -d '{"bucket_name": "test-bucket", "ram_quota_mb": 100}' \
  "$BASE_URL/buckets" | jq . || echo "Service not running or error occurred"

echo -e "\n${GREEN}✅ API Test Complete!${NC}"
echo -e "\n${BLUE}Note: If you see connection errors, make sure:${NC}"
echo "1. The service is running: cargo run"
echo "2. Couchbase is running (if testing with real Couchbase)"
echo "3. Or use Docker Compose: docker-compose up"
