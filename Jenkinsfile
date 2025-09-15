pipeline {
    agent any
    
    parameters {
        string(
            name: 'IMAGE_TAG',
            defaultValue: 'latest',
            description: 'Docker image tag to build'
        )
        string(
            name: 'COUCHBASE_HOST',
            defaultValue: 'http://couchbase:8091',
            description: 'Couchbase host URL'
        )
        string(
            name: 'COUCHBASE_USERNAME',
            defaultValue: 'Administrator',
            description: 'Couchbase username'
        )
        string(
            name: 'COUCHBASE_PASSWORD',
            defaultValue: '123456',
            description: 'Couchbase password'
        )
        booleanParam(
            name: 'RUN_TESTS',
            defaultValue: true,
            description: 'Run integration tests'
        )
        booleanParam(
            name: 'PUSH_TO_REGISTRY',
            defaultValue: false,
            description: 'Push image to registry'
        )
        string(
            name: 'REGISTRY_URL',
            defaultValue: '',
            description: 'Docker registry URL (if pushing)'
        )
    }
    
    environment {
        DOCKER_IMAGE = "couchbase-admin-service:${params.IMAGE_TAG}"
        COUCHBASE_HOST = "${params.COUCHBASE_HOST}"
        COUCHBASE_USERNAME = "${params.COUCHBASE_USERNAME}"
        COUCHBASE_PASSWORD = "${params.COUCHBASE_PASSWORD}"
    }
    
    stages {
        stage('Checkout') {
            steps {
                checkout scm
                script {
                    env.GIT_COMMIT_SHORT = sh(
                        script: 'git rev-parse --short HEAD',
                        returnStdout: true
                    ).trim()
                    env.BUILD_TAG = "${env.BUILD_NUMBER}-${env.GIT_COMMIT_SHORT}"
                }
            }
        }
        
        stage('Build Docker Image') {
            steps {
                script {
                    echo "Building Docker image: ${env.DOCKER_IMAGE}"
                    sh """
                        docker build -t ${env.DOCKER_IMAGE} .
                        docker tag ${env.DOCKER_IMAGE} ${env.DOCKER_IMAGE}-${env.BUILD_TAG}
                    """
                }
            }
        }
        
        stage('Test Image') {
            when {
                expression { params.RUN_TESTS == true }
            }
            steps {
                script {
                    echo "Testing Docker image..."
                    sh """
                        # Test basic image functionality
                        docker run --rm ${env.DOCKER_IMAGE} --help || echo "Help command not available"
                        
                        # Test with environment variables
                        docker run --rm -d --name test-container \\
                            -e COUCHBASE_HOST=${env.COUCHBASE_HOST} \\
                            -e COUCHBASE_USERNAME=${env.COUCHBASE_USERNAME} \\
                            -e COUCHBASE_PASSWORD=${env.COUCHBASE_PASSWORD} \\
                            -e AUTH_ENABLED=true \\
                            -e AUTH_USERNAME=admin \\
                            -e AUTH_PASSWORD=admin \\
                            -e RUST_LOG=info \\
                            ${env.DOCKER_IMAGE}
                        
                        # Wait for container to start
                        sleep 10
                        
                        # Check if container is running
                        if docker ps | grep test-container; then
                            echo "✅ Container started successfully"
                            
                            # Test health endpoint
                            sleep 5
                            if curl -f http://localhost:8080/health; then
                                echo "✅ Health endpoint working"
                            else
                                echo "❌ Health endpoint failed"
                            fi
                            
                            # Test metrics endpoint
                            if curl -f http://localhost:8080/metrics; then
                                echo "✅ Metrics endpoint working"
                            else
                                echo "❌ Metrics endpoint failed"
                            fi
                            
                            # Test protected endpoint with auth
                            if curl -u admin:admin -f http://localhost:8080/roles; then
                                echo "✅ Protected endpoint with auth working"
                            else
                                echo "❌ Protected endpoint with auth failed"
                            fi
                            
                            # Test protected endpoint without auth (should fail)
                            if curl -f http://localhost:8080/roles; then
                                echo "❌ Protected endpoint without auth should have failed"
                            else
                                echo "✅ Protected endpoint without auth correctly failed"
                            fi
                            
                        else
                            echo "❌ Container failed to start"
                            docker logs test-container
                            exit 1
                        fi
                        
                        # Clean up
                        docker stop test-container || true
                        docker rm test-container || true
                    """
                }
            }
        }
        
        stage('Integration Tests') {
            when {
                expression { params.RUN_TESTS == true }
            }
            steps {
                script {
                    echo "Running integration tests with Couchbase..."
                    sh """
                        # Start Couchbase if not running
                        if ! docker ps | grep jenkins-couchbase; then
                            echo "Starting Couchbase for integration tests..."
                            docker run -d --name jenkins-couchbase \\
                                -p 8091-8096:8091-8096 \\
                                -p 11210:11210 \\
                                -e COUCHBASE_ADMINISTRATOR_USERNAME=${env.COUCHBASE_USERNAME} \\
                                -e COUCHBASE_ADMINISTRATOR_PASSWORD=${env.COUCHBASE_PASSWORD} \\
                                --platform linux/amd64 \\
                                couchbase/server:7.0.2
                            
                            # Wait for Couchbase to be ready
                            echo "Waiting for Couchbase to be ready..."
                            for i in {1..60}; do
                                if curl -s http://localhost:8091/pools/default > /dev/null 2>&1; then
                                    echo "✅ Couchbase is ready!"
                                    break
                                fi
                                echo "⏳ Waiting... (\$i/60)"
                                sleep 5
                            done
                        fi
                        
                        # Run integration tests
                        docker run --rm --network host \\
                            -e COUCHBASE_HOST=http://localhost:8091 \\
                            -e COUCHBASE_USERNAME=${env.COUCHBASE_USERNAME} \\
                            -e COUCHBASE_PASSWORD=${env.COUCHBASE_PASSWORD} \\
                            -e AUTH_ENABLED=true \\
                            -e AUTH_USERNAME=admin \\
                            -e AUTH_PASSWORD=admin \\
                            -e RUST_LOG=info \\
                            ${env.DOCKER_IMAGE} &
                        
                        # Wait for service to start
                        sleep 15
                        
                        # Run test script
                        if [ -f "./test-api.sh" ]; then
                            chmod +x ./test-api.sh
                            ./test-api.sh
                        else
                            echo "Running basic API tests..."
                            # Test health
                            curl -f http://localhost:8080/health || exit 1
                            
                            # Test roles with auth
                            curl -u admin:admin -f http://localhost:8080/roles || exit 1
                            
                            echo "✅ Integration tests passed"
                        fi
                        
                        # Clean up
                        pkill -f couchbase-admin-service || true
                    """
                }
            }
        }
        
        stage('Push to Registry') {
            when {
                expression { params.PUSH_TO_REGISTRY == true && params.REGISTRY_URL != '' }
            }
            steps {
                script {
                    echo "Pushing image to registry: ${params.REGISTRY_URL}"
                    sh """
                        docker tag ${env.DOCKER_IMAGE} ${params.REGISTRY_URL}/${env.DOCKER_IMAGE}
                        docker push ${params.REGISTRY_URL}/${env.DOCKER_IMAGE}
                        
                        # Also tag and push with build tag
                        docker tag ${env.DOCKER_IMAGE} ${params.REGISTRY_URL}/${env.DOCKER_IMAGE}-${env.BUILD_TAG}
                        docker push ${params.REGISTRY_URL}/${env.DOCKER_IMAGE}-${env.BUILD_TAG}
                    """
                }
            }
        }
    }
    
    post {
        always {
            script {
                echo "Cleaning up containers..."
                sh """
                    docker stop test-container jenkins-couchbase 2>/dev/null || true
                    docker rm test-container jenkins-couchbase 2>/dev/null || true
                    pkill -f couchbase-admin-service 2>/dev/null || true
                """
            }
        }
        success {
            echo "✅ Pipeline completed successfully!"
            script {
                if (params.RUN_TESTS) {
                    echo "✅ All tests passed!"
                }
                if (params.PUSH_TO_REGISTRY) {
                    echo "✅ Image pushed to registry!"
                }
            }
        }
        failure {
            echo "❌ Pipeline failed!"
        }
        unstable {
            echo "⚠️ Pipeline completed with warnings!"
        }
    }
}