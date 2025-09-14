pipeline {
    agent any
    
    parameters {
        string(
            name: 'BUCKET_NAME',
            defaultValue: 'customer-data',
            description: 'Name of the Couchbase bucket to create'
        )
        string(
            name: 'SCOPE_NAME',
            defaultValue: 'orders',
            description: 'Name of the scope to create'
        )
        string(
            name: 'COLLECTION_NAME',
            defaultValue: 'transactions',
            description: 'Name of the collection to create'
        )
        string(
            name: 'USERNAME',
            defaultValue: 'devA',
            description: 'Username for the new user'
        )
        choice(
            name: 'ROLE',
            choices: ['data_reader', 'data_writer', 'data_dcp_reader', 'bucket_full_access', 'query_select', 'query_insert', 'query_update', 'query_delete'],
            description: 'RBAC role to assign to the user'
        )
        string(
            name: 'AWS_REGION',
            defaultValue: 'us-west-2',
            description: 'AWS region for deployment'
        )
        string(
            name: 'ECR_REGISTRY',
            defaultValue: '123456789012.dkr.ecr.us-west-2.amazonaws.com',
            description: 'AWS ECR registry URL'
        )
        string(
            name: 'ECS_CLUSTER',
            defaultValue: 'couchbase-admin-cluster',
            description: 'ECS cluster name'
        )
        string(
            name: 'ECS_SERVICE',
            defaultValue: 'couchbase-admin-service',
            description: 'ECS service name'
        )
    }
    
    environment {
        AWS_DEFAULT_REGION = "${params.AWS_REGION}"
        ECR_REGISTRY = "${params.ECR_REGISTRY}"
        IMAGE_TAG = "${env.BUILD_NUMBER}"
        IMAGE_NAME = "couchbase-admin-service"
        ECS_CLUSTER = "${params.ECS_CLUSTER}"
        ECS_SERVICE = "${params.ECS_SERVICE}"
    }
    
    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
        
        stage('Build and Test') {
            steps {
                script {
                    echo "Building Rust application..."
                    sh '''
                        # Install Rust if not present
                        if ! command -v cargo &> /dev/null; then
                            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                            source $HOME/.cargo/env
                        fi
                        
                        # Build the application
                        cargo build --release
                        
                        # Run tests
                        cargo test
                        
                        # Run clippy for code quality
                        cargo clippy -- -D warnings
                        
                        # Run fmt check
                        cargo fmt -- --check
                    '''
                }
            }
        }
        
        stage('Docker Build and Push') {
            steps {
                script {
                    echo "Building Docker image..."
                    sh '''
                        # Login to AWS ECR
                        aws ecr get-login-password --region ${AWS_DEFAULT_REGION} | docker login --username AWS --password-stdin ${ECR_REGISTRY}
                        
                        # Build Docker image
                        docker build -t ${IMAGE_NAME}:${IMAGE_TAG} .
                        docker tag ${IMAGE_NAME}:${IMAGE_TAG} ${ECR_REGISTRY}/${IMAGE_NAME}:${IMAGE_TAG}
                        docker tag ${IMAGE_NAME}:${IMAGE_TAG} ${ECR_REGISTRY}/${IMAGE_NAME}:latest
                        
                        # Push to ECR
                        docker push ${ECR_REGISTRY}/${IMAGE_NAME}:${IMAGE_TAG}
                        docker push ${ECR_REGISTRY}/${IMAGE_NAME}:latest
                    '''
                }
            }
        }
        
        stage('Deploy to ECS') {
            steps {
                script {
                    echo "Deploying to ECS..."
                    sh '''
                        # Update ECS service with new image
                        aws ecs update-service \
                            --cluster ${ECS_CLUSTER} \
                            --service ${ECS_SERVICE} \
                            --force-new-deployment \
                            --region ${AWS_DEFAULT_REGION}
                        
                        # Wait for deployment to complete
                        aws ecs wait services-stable \
                            --cluster ${ECS_CLUSTER} \
                            --services ${ECS_SERVICE} \
                            --region ${AWS_DEFAULT_REGION}
                    '''
                }
            }
        }
        
        stage('Create Couchbase Resources') {
            steps {
                script {
                    echo "Creating Couchbase resources..."
                    sh '''
                        # Get the service endpoint
                        SERVICE_ENDPOINT=$(aws ecs describe-services \
                            --cluster ${ECS_CLUSTER} \
                            --services ${ECS_SERVICE} \
                            --region ${AWS_DEFAULT_REGION} \
                            --query 'services[0].loadBalancers[0].hostname' \
                            --output text)
                        
                        if [ "$SERVICE_ENDPOINT" = "None" ] || [ -z "$SERVICE_ENDPOINT" ]; then
                            echo "Service endpoint not found, using task IP..."
                            TASK_ARN=$(aws ecs list-tasks \
                                --cluster ${ECS_CLUSTER} \
                                --service-name ${ECS_SERVICE} \
                                --region ${AWS_DEFAULT_REGION} \
                                --query 'taskArns[0]' \
                                --output text)
                            
                            TASK_IP=$(aws ecs describe-tasks \
                                --cluster ${ECS_CLUSTER} \
                                --tasks ${TASK_ARN} \
                                --region ${AWS_DEFAULT_REGION} \
                                --query 'tasks[0].attachments[0].details[?name==`networkInterfaceId`].value' \
                                --output text)
                            
                            SERVICE_ENDPOINT="http://${TASK_IP}:8080"
                        else
                            SERVICE_ENDPOINT="http://${SERVICE_ENDPOINT}:8080"
                        fi
                        
                        echo "Service endpoint: ${SERVICE_ENDPOINT}"
                        
                        # Wait for service to be ready
                        echo "Waiting for service to be ready..."
                        for i in {1..30}; do
                            if curl -f -s "${SERVICE_ENDPOINT}/health" > /dev/null; then
                                echo "Service is ready!"
                                break
                            fi
                            echo "Attempt $i: Service not ready yet, waiting..."
                            sleep 10
                        done
                        
                        # Create bucket
                        echo "Creating bucket: ${BUCKET_NAME}"
                        curl -X POST "${SERVICE_ENDPOINT}/buckets" \
                            -H "Content-Type: application/json" \
                            -H "Authorization: Basic $(echo -n 'admin:admin' | base64)" \
                            -d "{\"bucket_name\": \"${BUCKET_NAME}\"}" \
                            -w "HTTP Status: %{http_code}\n"
                        
                        # Wait a bit for bucket creation
                        sleep 5
                        
                        # Create scope
                        echo "Creating scope: ${SCOPE_NAME}"
                        curl -X POST "${SERVICE_ENDPOINT}/buckets/${BUCKET_NAME}/scopes" \
                            -H "Content-Type: application/json" \
                            -H "Authorization: Basic $(echo -n 'admin:admin' | base64)" \
                            -d "{\"scope_name\": \"${SCOPE_NAME}\"}" \
                            -w "HTTP Status: %{http_code}\n"
                        
                        # Wait a bit for scope creation
                        sleep 5
                        
                        # Create collection
                        echo "Creating collection: ${COLLECTION_NAME}"
                        curl -X POST "${SERVICE_ENDPOINT}/buckets/${BUCKET_NAME}/scopes/${SCOPE_NAME}/collections" \
                            -H "Content-Type: application/json" \
                            -H "Authorization: Basic $(echo -n 'admin:admin' | base64)" \
                            -d "{\"collection_name\": \"${COLLECTION_NAME}\"}" \
                            -w "HTTP Status: %{http_code}\n"
                        
                        # Wait a bit for collection creation
                        sleep 5
                        
                        # Create user
                        echo "Creating user: ${USERNAME} with role: ${ROLE}"
                        curl -X POST "${SERVICE_ENDPOINT}/users" \
                            -H "Content-Type: application/json" \
                            -H "Authorization: Basic $(echo -n 'admin:admin' | base64)" \
                            -d "{
                                \"username\": \"${USERNAME}\",
                                \"password\": \"SecurePassword123!\",
                                \"roles\": [
                                    {
                                        \"role\": \"${ROLE}\",
                                        \"bucket\": \"${BUCKET_NAME}\",
                                        \"scope\": \"${SCOPE_NAME}\",
                                        \"collection\": \"${COLLECTION_NAME}\"
                                    }
                                ]
                            }" \
                            -w "HTTP Status: %{http_code}\n"
                        
                        echo "Couchbase resources created successfully!"
                    '''
                }
            }
        }
        
        stage('Verification') {
            steps {
                script {
                    echo "Verifying created resources..."
                    sh '''
                        # Get the service endpoint again
                        SERVICE_ENDPOINT=$(aws ecs describe-services \
                            --cluster ${ECS_CLUSTER} \
                            --services ${ECS_SERVICE} \
                            --region ${AWS_DEFAULT_REGION} \
                            --query 'services[0].loadBalancers[0].hostname' \
                            --output text)
                        
                        if [ "$SERVICE_ENDPOINT" = "None" ] || [ -z "$SERVICE_ENDPOINT" ]; then
                            TASK_ARN=$(aws ecs list-tasks \
                                --cluster ${ECS_CLUSTER} \
                                --service-name ${ECS_SERVICE} \
                                --region ${AWS_DEFAULT_REGION} \
                                --query 'taskArns[0]' \
                                --output text)
                            
                            TASK_IP=$(aws ecs describe-tasks \
                                --cluster ${ECS_CLUSTER} \
                                --tasks ${TASK_ARN} \
                                --region ${AWS_DEFAULT_REGION} \
                                --query 'tasks[0].attachments[0].details[?name==`networkInterfaceId`].value' \
                                --output text)
                            
                            SERVICE_ENDPOINT="http://${TASK_IP}:8080"
                        else
                            SERVICE_ENDPOINT="http://${SERVICE_ENDPOINT}:8080"
                        fi
                        
                        # List buckets
                        echo "Listing buckets:"
                        curl -s "${SERVICE_ENDPOINT}/buckets" \
                            -H "Authorization: Basic $(echo -n 'admin:admin' | base64)" | jq '.'
                        
                        # List scopes
                        echo "Listing scopes in bucket ${BUCKET_NAME}:"
                        curl -s "${SERVICE_ENDPOINT}/buckets/${BUCKET_NAME}/scopes" \
                            -H "Authorization: Basic $(echo -n 'admin:admin' | base64)" | jq '.'
                        
                        # List collections
                        echo "Listing collections in scope ${SCOPE_NAME}:"
                        curl -s "${SERVICE_ENDPOINT}/buckets/${BUCKET_NAME}/scopes/${SCOPE_NAME}/collections" \
                            -H "Authorization: Basic $(echo -n 'admin:admin' | base64)" | jq '.'
                        
                        # List users
                        echo "Listing users:"
                        curl -s "${SERVICE_ENDPOINT}/users" \
                            -H "Authorization: Basic $(echo -n 'admin:admin' | base64)" | jq '.'
                    '''
                }
            }
        }
    }
    
    post {
        always {
            echo "Pipeline completed!"
        }
        success {
            echo "Deployment successful! Resources created:"
            echo "- Bucket: ${params.BUCKET_NAME}"
            echo "- Scope: ${params.SCOPE_NAME}"
            echo "- Collection: ${params.COLLECTION_NAME}"
            echo "- User: ${params.USERNAME} with role: ${params.ROLE}"
        }
        failure {
            echo "Deployment failed! Check logs for details."
        }
    }
}
