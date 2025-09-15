# Jenkins Setup Guide for Couchbase Admin Service

## 🚀 Quick Start

### 1. Start Jenkins
```bash
./setup-jenkins.sh
```

### 2. Access Jenkins
- **URL**: http://localhost:8081
- **Username**: admin
- **Password**: Check the output of `setup-jenkins.sh` or run:
  ```bash
  docker exec jenkins-local cat /var/jenkins_home/secrets/initialAdminPassword
  ```

## 📋 Jenkins Pipeline Features

The Jenkins pipeline includes the following parameters:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `IMAGE_TAG` | `latest` | Docker image tag to build |
| `COUCHBASE_HOST` | `http://couchbase:8091` | Couchbase host URL |
| `COUCHBASE_USERNAME` | `Administrator` | Couchbase username |
| `COUCHBASE_PASSWORD` | `123456` | Couchbase password |
| `RUN_TESTS` | `true` | Run integration tests |
| `PUSH_TO_REGISTRY` | `false` | Push image to registry |
| `REGISTRY_URL` | `` | Docker registry URL (if pushing) |

## 🔧 Pipeline Stages

### 1. Checkout
- Checks out the source code from Git
- Sets up build environment variables

### 2. Build Docker Image
- Builds the Docker image with the specified tag
- Creates additional tags with build number and Git commit

### 3. Test Image
- Tests basic Docker image functionality
- Verifies health and metrics endpoints
- Tests authentication (with and without credentials)

### 4. Integration Tests
- Starts Couchbase if not running
- Runs comprehensive API tests
- Tests all endpoints with real Couchbase integration

### 5. Push to Registry (Optional)
- Pushes image to specified Docker registry
- Creates tagged versions for traceability

## 🧪 Testing

### Test Jenkins Setup
```bash
./test-jenkins.sh
```

### Manual Testing
```bash
# Test Jenkins
curl http://localhost:8081/login

# Test Couchbase
curl http://localhost:8091/pools/default

# Test your service
curl http://localhost:8080/health
```

## 📁 File Structure

```
couchbase-admin-service/
├── Jenkinsfile                 # Main Jenkins pipeline
├── setup-jenkins.sh           # Jenkins setup script
├── test-jenkins.sh            # Test script
├── jenkins/
│   └── docker-compose.yml     # Jenkins + Couchbase setup
└── JENKINS_SETUP_GUIDE.md     # This guide
```

## 🎯 Creating a Jenkins Job

### 1. Create New Pipeline Job
1. Go to Jenkins → New Item
2. Enter job name (e.g., "couchbase-admin-service")
3. Select "Pipeline"
4. Click OK

### 2. Configure Pipeline
1. In the Pipeline section:
   - **Definition**: Pipeline script from SCM
   - **SCM**: Git
   - **Repository URL**: Your Git repository URL
   - **Script Path**: Jenkinsfile

### 3. Build with Parameters
1. Click "Build with Parameters"
2. Adjust parameters as needed
3. Click "Build"

## 🔍 Monitoring

### View Logs
```bash
# Jenkins logs
docker logs jenkins-local

# Couchbase logs
docker logs jenkins-couchbase

# Your service logs (if running)
docker logs <container-name>
```

### Container Status
```bash
docker ps --filter "name=jenkins"
```

## 🛠️ Troubleshooting

### Common Issues

1. **Port Conflicts**
   - Stop existing containers: `docker stop $(docker ps -q)`
   - Restart Jenkins: `cd jenkins && docker compose restart`

2. **Couchbase Not Ready**
   - Wait 2-3 minutes for Couchbase to fully start
   - Check logs: `docker logs jenkins-couchbase`

3. **Docker Permission Issues**
   - Ensure Docker socket is accessible
   - Check Jenkins container permissions

4. **Build Failures**
   - Check Docker image build locally first
   - Verify all dependencies are installed
   - Check Jenkins agent has Docker access

### Reset Everything
```bash
# Stop all containers
docker stop $(docker ps -q)

# Remove all containers
docker rm $(docker ps -aq)

# Remove volumes (WARNING: This will delete Jenkins data)
docker volume rm jenkins_jenkins_home jenkins_couchbase_data

# Restart
./setup-jenkins.sh
```

## 🚀 Advanced Usage

### Custom Registry Integration
1. Set `PUSH_TO_REGISTRY=true`
2. Provide `REGISTRY_URL` (e.g., `your-registry.com`)
3. Ensure Jenkins has registry credentials

### Custom Test Scripts
- Modify `test-jenkins.sh` for additional tests
- Add custom test stages in `Jenkinsfile`
- Use environment variables for configuration

### Multiple Environments
- Create separate Jenkins jobs for different environments
- Use different parameter values for dev/staging/prod
- Implement promotion pipelines

## 📊 Success Criteria

A successful pipeline run should show:
- ✅ Docker image builds successfully
- ✅ Container starts and runs
- ✅ Health endpoint responds (200)
- ✅ Metrics endpoint responds (200)
- ✅ Protected endpoints work with authentication
- ✅ Protected endpoints fail without authentication
- ✅ Integration tests pass with Couchbase

## 🎉 Next Steps

1. **Set up Git integration** - Connect Jenkins to your Git repository
2. **Configure webhooks** - Automatic builds on code changes
3. **Add notifications** - Email/Slack notifications on build status
4. **Set up staging environment** - Deploy to staging after successful builds
5. **Implement security scanning** - Add vulnerability scanning to the pipeline
6. **Add performance testing** - Include load testing in the pipeline

## 📞 Support

If you encounter issues:
1. Check the troubleshooting section above
2. Review Jenkins and Docker logs
3. Test components individually
4. Verify network connectivity between containers
