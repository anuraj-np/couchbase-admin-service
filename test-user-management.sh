#!/bin/bash

# Enhanced User Management Test Script
# Tests comprehensive RBAC functionality with bucket/scope/collection access

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
BASE_URL=${1:-http://localhost:8080}
USERNAME=${2:-admin}
PASSWORD=${3:-admin}

echo -e "${BLUE}🧪 Testing Enhanced User Management with RBAC${NC}"
echo "=================================================="
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

# Test 1: Get Available Roles
test_endpoint "GET" "/roles" "" "Get Available Roles and Descriptions"

# Test 2: Create Admin User (Console Access)
test_endpoint "POST" "/users" '{
    "username": "admin-user",
    "password": "securepassword123",
    "display_name": "Admin User",
    "email": "admin@example.com",
    "roles": [
        {
            "role": "admin"
        }
    ]
}' "Create Admin User with Console Access"

# Test 3: Create Data Reader User (Bucket-specific)
test_endpoint "POST" "/users" '{
    "username": "data-reader",
    "password": "securepassword123",
    "display_name": "Data Reader",
    "roles": [
        {
            "role": "data_reader",
            "bucket": "my-test-bucket"
        }
    ]
}' "Create Data Reader User for Specific Bucket"

# Test 4: Create Data Writer User (Scope-specific)
test_endpoint "POST" "/users" '{
    "username": "data-writer",
    "password": "securepassword123",
    "display_name": "Data Writer",
    "roles": [
        {
            "role": "data_writer",
            "bucket": "my-test-bucket",
            "scope": "_default"
        }
    ]
}' "Create Data Writer User for Specific Scope"

# Test 5: Create Query User (Collection-specific)
test_endpoint "POST" "/users" '{
    "username": "query-user",
    "password": "securepassword123",
    "display_name": "Query User",
    "roles": [
        {
            "role": "query_select",
            "bucket": "my-test-bucket",
            "scope": "_default",
            "collection": "_default"
        },
        {
            "role": "query_insert",
            "bucket": "my-test-bucket",
            "scope": "_default",
            "collection": "_default"
        }
    ]
}' "Create Query User for Specific Collection"

# Test 6: Create Multi-Role User
test_endpoint "POST" "/users" '{
    "username": "multi-role-user",
    "password": "securepassword123",
    "display_name": "Multi Role User",
    "groups": ["developers", "analysts"],
    "roles": [
        {
            "role": "data_reader",
            "bucket": "my-test-bucket"
        },
        {
            "role": "query_select"
        },
        {
            "role": "views_admin"
        }
    ]
}' "Create Multi-Role User with Console Access"

# Test 7: List All Users
test_endpoint "GET" "/users" "" "List All Users"

# Test 8: Get User Permissions
test_endpoint "GET" "/users/admin-user/permissions" "" "Get Admin User Permissions"

test_endpoint "GET" "/users/data-reader/permissions" "" "Get Data Reader User Permissions"

test_endpoint "GET" "/users/multi-role-user/permissions" "" "Get Multi-Role User Permissions"

# Test 9: Update User Roles
test_endpoint "PUT" "/users/data-reader/roles" '[
    {
        "role": "data_reader",
        "bucket": "my-test-bucket"
    },
    {
        "role": "data_writer",
        "bucket": "my-test-bucket"
    },
    {
        "role": "query_select"
    }
]' "Update Data Reader User Roles"

# Test 10: Test Invalid Role (Should Fail)
test_endpoint "POST" "/users" '{
    "username": "invalid-role-user",
    "password": "securepassword123",
    "roles": [
        {
            "role": "invalid_role",
            "bucket": "my-test-bucket"
        }
    ]
}' "Test Invalid Role (Should Fail)"

# Test 11: Test Missing Bucket for Data Role (Should Fail)
test_endpoint "POST" "/users" '{
    "username": "missing-bucket-user",
    "password": "securepassword123",
    "roles": [
        {
            "role": "data_reader"
        }
    ]
}' "Test Missing Bucket for Data Role (Should Fail)"

# Test 12: Test Invalid Username (Should Fail)
test_endpoint "POST" "/users" '{
    "username": "ab",
    "password": "securepassword123",
    "roles": [
        {
            "role": "data_reader",
            "bucket": "my-test-bucket"
        }
    ]
}' "Test Invalid Username (Should Fail)"

# Test 13: Test Weak Password (Should Fail)
test_endpoint "POST" "/users" '{
    "username": "weak-password-user",
    "password": "123",
    "roles": [
        {
            "role": "data_reader",
            "bucket": "my-test-bucket"
        }
    ]
}' "Test Weak Password (Should Fail)"

# Test 14: Test Duplicate User (Should Fail)
test_endpoint "POST" "/users" '{
    "username": "admin-user",
    "password": "securepassword123",
    "roles": [
        {
            "role": "data_reader",
            "bucket": "my-test-bucket"
        }
    ]
}' "Test Duplicate User (Should Fail)"

# Test 15: Final User List
test_endpoint "GET" "/users" "" "Final User List"

echo -e "${GREEN}🎉 Enhanced User Management Tests Completed!${NC}"
echo ""
echo -e "${BLUE}Key Features Tested:${NC}"
echo "✅ Role validation and categorization"
echo "✅ Console access permissions"
echo "✅ Bucket-specific access control"
echo "✅ Scope-specific access control"
echo "✅ Collection-specific access control"
echo "✅ Multi-role user support"
echo "✅ Permission summary generation"
echo "✅ Role updates and management"
echo "✅ Comprehensive input validation"
echo "✅ Error handling for invalid inputs"
echo ""
echo -e "${YELLOW}Note: Some tests are expected to fail to demonstrate validation${NC}"
