# Contributing to Couchbase Admin Service

Thank you for your interest in contributing to the Couchbase Admin Service! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (stable)
- Docker & Docker Compose
- Git
- Make (optional)

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/yourusername/couchbase-admin-service.git
   cd couchbase-admin-service
   ```

2. **Environment Setup**
   ```bash
   cp env.example .env
   # Edit .env with your Couchbase connection details
   ```

3. **Install Dependencies**
   ```bash
   cargo build
   ```

4. **Run Tests**
   ```bash
   cargo test
   ```

## 📝 Development Guidelines

### Code Style

- Follow Rust conventions and idioms
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Write comprehensive tests for new features

### Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Examples:
- `feat(users): add role validation`
- `fix(api): handle connection timeout`
- `docs(readme): update installation guide`

### Pull Request Process

1. **Create a Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**
   - Write code following the style guidelines
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Your Changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit and Push**
   ```bash
   git add .
   git commit -m "feat: add your feature"
   git push origin feature/your-feature-name
   ```

5. **Create Pull Request**
   - Provide a clear description of changes
   - Reference any related issues
   - Ensure all CI checks pass

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# Integration tests
cargo test --test integration

# API tests
./test-api.sh
```

### Test Guidelines

- Write unit tests for all new functions
- Add integration tests for API endpoints
- Test error conditions and edge cases
- Ensure tests are deterministic and isolated

## 📚 Documentation

### Code Documentation

- Document all public APIs with doc comments
- Use `///` for module and function documentation
- Include examples in doc comments where helpful

### README Updates

- Update README.md for new features
- Add usage examples
- Update API documentation

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Environment Details**
   - Rust version
   - Operating system
   - Couchbase version

2. **Steps to Reproduce**
   - Clear, numbered steps
   - Expected vs actual behavior

3. **Additional Context**
   - Error messages
   - Logs (if applicable)
   - Screenshots (if relevant)

## ✨ Feature Requests

When requesting features, please include:

1. **Use Case**
   - Why is this feature needed?
   - What problem does it solve?

2. **Proposed Solution**
   - How should it work?
   - Any implementation ideas?

3. **Alternatives**
   - Other ways to solve the problem?

## 🏗️ Architecture

### Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── error.rs             # Error handling
├── middleware.rs        # Authentication middleware
├── models.rs            # Data models and DTOs
├── routes/              # API route handlers
│   ├── buckets.rs
│   ├── scopes.rs
│   ├── collections.rs
│   └── users.rs
└── services.rs          # Couchbase service integration
```

### Key Principles

- **Separation of Concerns**: Clear separation between routes, services, and models
- **Error Handling**: Comprehensive error handling with proper HTTP status codes
- **Async/Await**: Use async patterns throughout
- **Type Safety**: Leverage Rust's type system for safety

## 🔒 Security

### Security Guidelines

- Never commit secrets or credentials
- Use environment variables for sensitive data
- Validate all input data
- Follow OWASP guidelines for web security

### Reporting Security Issues

For security vulnerabilities, please:

1. **DO NOT** create a public issue
2. Email security concerns to: [security@yourdomain.com]
3. Include detailed information about the vulnerability
4. Allow time for response before public disclosure

## 📞 Getting Help

- **Issues**: Create an issue for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check the README and inline docs

## 🎉 Recognition

Contributors will be recognized in:

- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to the Couchbase Admin Service! 🚀
