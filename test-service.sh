#!/bin/bash

# Couchbase Admin Service Test Script
# Usage: ./test-service.sh [base-url] [username] [password]

set -e

# Default values
BASE_URL=${1:-http://localhost:8080}
USERNAME=${2:-admin}
PASSWORD=${3:-admin}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 Testing Couchbase Admin Service${NC}"
echo "=================================="
echo -e "Base URL: ${YELLOW}$BASE_URL${NC}"
echo -e "Username: ${YELLOW}$USERNAME${NC}"
echo ""

# Create base64 auth header
AUTH_HEADER=$(echo -n "$USERNAME:$PASSWORD" | base64)

# Test function
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local description=$4
    
    echo -e "${BLUE}Testing: $description${NC}"
    
    if [ -n "$data" ]; then
        response=$(curl -s -X $method "$BASE_URL$endpoint" \
            -H "Content-Type: application/json" \
            -H "Authorization: Basic $AUTH_HEADER" \
            -d "$data" \
            -w "\n%{http_code}")
    else
        response=$(curl -s -X $method "$BASE_URL$endpoint" \
            -H "Authorization: Basic $AUTH_HEADER" \
            -w "\n%{http_code}")
    fi
    
    # Extract status code (last line)
    status_code=$(echo "$response" | tail -n 1)
    # Extract response body (all but last line)
    body=$(echo "$response" | sed '$d')
    
    if [ "$status_code" -ge 200 ] && [ "$status_code" -lt 300 ]; then
        echo -e "${GREEN}✅ Success (HTTP $status_code)${NC}"
        echo "$body" | jq . 2>/dev/null || echo "$body"
    else
        echo -e "${RED}❌ Failed (HTTP $status_code)${NC}"
        echo "$body"
    fi
    echo ""
}

# Test 1: Health Check
test_endpoint "GET" "/health" "" "Health Check"

# Test 2: Metrics
test_endpoint "GET" "/metrics" "" "Metrics Endpoint"

# Test 3: List Buckets
test_endpoint "GET" "/buckets" "" "List Buckets"

# Test 4: Create Test Bucket
BUCKET_NAME="test-bucket-$(date +%s)"
test_endpoint "POST" "/buckets" "{\"bucket_name\": \"$BUCKET_NAME\", \"ram_quota_mb\": 100}" "Create Test Bucket: $BUCKET_NAME"

# Test 5: List Buckets Again
test_endpoint "GET" "/buckets" "" "List Buckets (After Creation)"

# Test 6: List Scopes
test_endpoint "GET" "/buckets/$BUCKET_NAME/scopes" "" "List Scopes in $BUCKET_NAME"

# Test 7: List Collections
test_endpoint "GET" "/buckets/$BUCKET_NAME/scopes/_default/collections" "" "List Collections in $BUCKET_NAME"

# Test 8: List Users
test_endpoint "GET" "/users" "" "List Users"

echo -e "${GREEN}🎉 All tests completed!${NC}"
echo ""
echo -e "${BLUE}Service is working correctly if all tests show ✅${NC}"
echo -e "${YELLOW}Note: Some tests may fail if Couchbase is not properly configured${NC}"
