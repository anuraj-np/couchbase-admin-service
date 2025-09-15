#!/bin/bash

echo "🚀 Setting up Jenkins for Couchbase Admin Service"
echo "================================================"

# Create jenkins directory if it doesn't exist
mkdir -p jenkins

# Start Jenkins
echo "📦 Starting Jenkins and Couchbase..."
cd jenkins
docker compose up -d

echo "⏳ Waiting for Jenkins to start (this may take 2-3 minutes)..."
# Wait for Jenkins to be ready
for i in {1..60}; do
    if curl -s http://localhost:8081/login > /dev/null 2>&1; then
        echo "✅ Jenkins is ready!"
        break
    fi
    echo "⏳ Waiting... ($i/60)"
    sleep 5
done

# Get Jenkins admin password
echo "🔑 Getting Jenkins admin password..."
JENKINS_PASSWORD=$(docker exec jenkins-local cat /var/jenkins_home/secrets/initialAdminPassword 2>/dev/null || echo "Password not available yet")

echo ""
echo "🎉 Jenkins setup complete!"
echo "========================="
echo ""
echo "📋 Access Information:"
echo "  Jenkins URL: http://localhost:8081"
echo "  Admin Password: $JENKINS_PASSWORD"
echo ""
echo "📝 Next Steps:"
echo "  1. Open http://localhost:8081 in your browser"
echo "  2. Login with admin and the password above"
echo "  3. Install suggested plugins"
echo "  4. Create a new Pipeline job"
echo "  5. Point it to your Git repository"
echo "  6. Use the Jenkinsfile in the root directory"
echo ""
echo "🔧 Jenkins Commands:"
echo "  View logs: docker logs jenkins-local"
echo "  Stop: docker compose down"
echo "  Restart: docker compose restart"
echo ""
echo "🧪 Test Commands:"
echo "  Test Jenkins: curl http://localhost:8081/login"
echo "  Test Couchbase: curl http://localhost:8091/pools/default"
echo ""

# Show running containers
echo "📊 Running Containers:"
docker ps --filter "name=jenkins"
