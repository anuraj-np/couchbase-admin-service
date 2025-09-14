#!/bin/bash

# Couchbase Admin Service Deployment Script
# Usage: ./deploy.sh [local|eks] [environment]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
DEPLOYMENT_TYPE=${1:-local}
ENVIRONMENT=${2:-dev}

echo -e "${BLUE}🚀 Couchbase Admin Service Deployment${NC}"
echo "=================================="
echo -e "Type: ${YELLOW}$DEPLOYMENT_TYPE${NC}"
echo -e "Environment: ${YELLOW}$ENVIRONMENT${NC}"
echo ""

# Function to check prerequisites
check_prerequisites() {
    echo -e "${BLUE}Checking prerequisites...${NC}"
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Rust/Cargo not found. Please install Rust first.${NC}"
        exit 1
    fi
    
    if ! command -v git &> /dev/null; then
        echo -e "${RED}❌ Git not found. Please install Git first.${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Prerequisites check passed${NC}"
}

# Function for local deployment
deploy_local() {
    echo -e "${BLUE}🖥️  Deploying locally...${NC}"
    
    # Check if .env exists
    if [ ! -f .env ]; then
        echo -e "${YELLOW}⚠️  .env file not found. Creating from template...${NC}"
        cp env.example .env
        echo -e "${YELLOW}Please edit .env file with your configuration and run again.${NC}"
        exit 1
    fi
    
    # Build the project
    echo -e "${BLUE}Building project...${NC}"
    cargo build --release
    
    # Check if service is already running
    if pgrep -f "couchbase-admin-service" > /dev/null; then
        echo -e "${YELLOW}⚠️  Service is already running. Stopping it...${NC}"
        pkill -f "couchbase-admin-service"
        sleep 2
    fi
    
    # Start the service
    echo -e "${BLUE}Starting service...${NC}"
    nohup cargo run > service.log 2>&1 &
    
    # Wait for service to start
    echo -e "${BLUE}Waiting for service to start...${NC}"
    sleep 5
    
    # Test the service
    if curl -s http://localhost:8080/health > /dev/null; then
        echo -e "${GREEN}✅ Service started successfully!${NC}"
        echo -e "Health check: ${GREEN}http://localhost:8080/health${NC}"
        echo -e "Logs: ${BLUE}tail -f service.log${NC}"
    else
        echo -e "${RED}❌ Service failed to start. Check logs: tail -f service.log${NC}"
        exit 1
    fi
}

# Function for EKS deployment
deploy_eks() {
    echo -e "${BLUE}☁️  Deploying to EKS...${NC}"
    
    # Check prerequisites
    if ! command -v aws &> /dev/null; then
        echo -e "${RED}❌ AWS CLI not found. Please install AWS CLI first.${NC}"
        exit 1
    fi
    
    if ! command -v kubectl &> /dev/null; then
        echo -e "${RED}❌ kubectl not found. Please install kubectl first.${NC}"
        exit 1
    fi
    
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}❌ Docker not found. Please install Docker first.${NC}"
        exit 1
    fi
    
    # Set variables
    export AWS_REGION=${AWS_REGION:-us-west-2}
    export EKS_CLUSTER_NAME=${EKS_CLUSTER_NAME:-couchbase-admin-cluster}
    export ECR_REPOSITORY=${ECR_REPOSITORY:-couchbase-admin-service}
    export NAMESPACE=${NAMESPACE:-couchbase-admin}
    
    echo -e "AWS Region: ${YELLOW}$AWS_REGION${NC}"
    echo -e "EKS Cluster: ${YELLOW}$EKS_CLUSTER_NAME${NC}"
    echo -e "ECR Repository: ${YELLOW}$ECR_REPOSITORY${NC}"
    
    # Get AWS account ID
    AWS_ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)
    ECR_URI="$AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/$ECR_REPOSITORY"
    
    echo -e "ECR URI: ${YELLOW}$ECR_URI${NC}"
    
    # Build and push Docker image
    echo -e "${BLUE}Building Docker image...${NC}"
    docker build -t $ECR_REPOSITORY:latest .
    
    echo -e "${BLUE}Tagging image for ECR...${NC}"
    docker tag $ECR_REPOSITORY:latest $ECR_URI:latest
    
    echo -e "${BLUE}Logging into ECR...${NC}"
    aws ecr get-login-password --region $AWS_REGION | \
        docker login --username AWS --password-stdin $ECR_URI
    
    echo -e "${BLUE}Pushing image to ECR...${NC}"
    docker push $ECR_URI:latest
    
    # Update kubeconfig
    echo -e "${BLUE}Updating kubeconfig...${NC}"
    aws eks update-kubeconfig --region $AWS_REGION --name $EKS_CLUSTER_NAME
    
    # Update deployment image
    echo -e "${BLUE}Updating deployment image...${NC}"
    sed -i.bak "s|<your-account-id>|$AWS_ACCOUNT_ID|g" k8s/deployment.yaml
    
    # Deploy to Kubernetes
    echo -e "${BLUE}Deploying to Kubernetes...${NC}"
    kubectl apply -f k8s/namespace.yaml
    kubectl apply -f k8s/configmap.yaml
    kubectl apply -f k8s/deployment.yaml
    kubectl apply -f k8s/service.yaml
    kubectl apply -f k8s/ingress.yaml
    
    # Wait for deployment
    echo -e "${BLUE}Waiting for deployment to be ready...${NC}"
    kubectl wait --for=condition=available --timeout=300s deployment/couchbase-admin-service -n $NAMESPACE
    
    # Get service info
    echo -e "${GREEN}✅ Deployment completed!${NC}"
    echo -e "Pods: ${BLUE}kubectl get pods -n $NAMESPACE${NC}"
    echo -e "Services: ${BLUE}kubectl get services -n $NAMESPACE${NC}"
    echo -e "Logs: ${BLUE}kubectl logs -f deployment/couchbase-admin-service -n $NAMESPACE${NC}"
}

# Main deployment logic
case $DEPLOYMENT_TYPE in
    local)
        check_prerequisites
        deploy_local
        ;;
    eks)
        check_prerequisites
        deploy_eks
        ;;
    *)
        echo -e "${RED}❌ Invalid deployment type. Use 'local' or 'eks'${NC}"
        echo "Usage: $0 [local|eks] [environment]"
        exit 1
        ;;
esac

echo -e "${GREEN}🎉 Deployment completed successfully!${NC}"
