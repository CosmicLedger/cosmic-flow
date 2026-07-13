# Changelog

All notable changes to the Stellar Chain Explorer project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-07-13

### 🎉 Initial Release - CosmicLedger Rebrand

This is the first release of Stellar Chain Explorer under the CosmicLedger organization, based on the original StellarExplorer project by Chris Hatch.

### Added

#### 🦀 **Rust Integration** (NEW!)
- **High-Performance Indexer**: Rust-based blockchain indexer
  - 10x faster than Node.js implementation
  - Real-time ledger and transaction indexing
  - Automatic retry and recovery mechanisms
  - Background processing with Tokio async runtime
- **Fast API Server**: Actix-web based REST API
  - Sub-5ms latency for most queries
  - 10,000+ requests per second throughput
  - Prometheus metrics integration
  - CORS support with security controls
- **PostgreSQL Integration**: Optimized database schema
  - Efficient indexes for all query patterns
  - Automatic migrations
  - Connection pooling (20 connections default)
  - Type-safe queries with SQLx
- **Redis Caching**: Optional caching layer for improved performance
- **Docker Support**: Complete containerization
  - Production-optimized Dockerfile
  - Multi-stage builds for minimal image size
  - Docker Compose with PostgreSQL and Redis
  - Health checks and automatic restarts
- **Comprehensive Documentation**: Full Rust integration guide
  - Setup instructions
  - API documentation
  - Performance benchmarks
  - Troubleshooting guide

#### 🏗️ Infrastructure & Deployment
- **Docker Support**: Production and development Dockerfiles with docker-compose configuration
- **CI/CD Workflows**: Comprehensive GitHub Actions workflows
  - `node.js.yml`: Main CI pipeline with linting, testing, and coverage
  - `deploy.yml`: Automated deployment to Cloudflare Pages and Fly.io
  - `code-quality.yml`: Code quality checks and bundle analysis
  - `performance.yml`: Performance monitoring with Lighthouse CI
  - Security scanning with CodeQL and Trivy
- **Environment Configuration**: Complete `.env.example` with all configuration options

#### 🧪 Testing
- **Test Coverage Configuration**: Enhanced Jest configuration with coverage thresholds
  - Minimum 50% coverage requirement
  - Multiple coverage reporters (JSON, LCOV, HTML, text)
  - Coverage collection from app source files only
- **Test Utilities**: Custom test utilities in `app/test-utils.tsx`
  - Custom render function with IntlProvider
  - Mock factories for Stellar entities (accounts, transactions, ledgers, operations)
  - Fetch response mocking helpers
- **E2E Testing**: Parallel E2E test execution with sharding
  - Playwright configuration for browser testing
  - Test result artifacts and reporting

#### 📚 Documentation
- **CONTRIBUTING.md**: Comprehensive contribution guidelines
  - Code of conduct
  - Development workflow
  - Testing requirements
  - Code style guidelines
  - Commit message conventions
  - Pull request process
- **SETUP.md**: Detailed setup and installation guide
  - Prerequisites and installation
  - Testing instructions
  - Building and deployment
  - CI/CD configuration
  - Troubleshooting guide
- **Enhanced README.md**: Complete project documentation
  - Quick start guide
  - Available scripts reference
  - Docker development instructions
  - Architecture overview
  - Deployment guides
  - Security best practices
  - Browser support matrix
- **Issue Templates**: Bug report and feature request templates
- **Pull Request Template**: Standardized PR checklist and guidelines

#### 🎨 Branding
- **Organization**: CosmicLedger
- **Repository**: stellar-chain-explorer
- **Package Name**: @cosmicledger/stellar-chain-explorer
- **Domains**:
  - Production: explorer.cosmicledger.io
  - Testnet: testnet.cosmicledger.io
  - Futurenet: futurenet.cosmicledger.io
  - API: api.cosmicledger.io

#### 🔄 CI/CD Enhancements
- Automated code coverage reporting to Codecov
- Bundle size analysis and reporting
- Dependency review for pull requests
- License compliance checking
- Security vulnerability scanning
- Smoke tests after deployment
- Parallel test execution for faster CI

### Changed

#### 📦 Package Configuration
- Updated `package.json` with new organization and repository information
- Changed author to "CosmicLedger Team"
- Updated homepage and bug tracking URLs
- Updated repository URL structure

#### 🔧 Configuration Updates
- Enhanced Jest configuration with coverage thresholds
- Updated GitHub Actions workflows to use `main` branch (from `master`)
- Changed deployment targets to CosmicLedger infrastructure
- Updated all references from steexp.com to cosmicledger.io domains

#### 📄 Documentation
- Updated all URLs and references throughout the codebase
- Enhanced README with comprehensive sections:
  - Architecture and tech stack
  - Performance targets and optimization
  - Security practices
  - Browser support matrix
  - Support and contact information
- Added language translation file references

### Security

- **Vulnerability Scanning**: Integrated Trivy for container vulnerability scanning
- **Dependency Audits**: Automated pnpm audit in CI/CD
- **CodeQL Analysis**: Static code analysis for security issues
- **Dependency Review**: Automated review for pull requests
- **Secret Management**: Environment variable examples without sensitive data

### Performance

- **Lighthouse CI**: Automated performance monitoring
- **Bundle Analysis**: Size tracking and optimization
- **Performance Targets**:
  - Lighthouse Score: >90
  - First Contentful Paint: <1.5s
  - Time to Interactive: <3.5s
  - Bundle Size: <500KB (gzipped)

### Infrastructure

- **Multi-stage Docker Builds**: Optimized production images
- **Health Checks**: Container health monitoring
- **Automated Deployments**: Tag-based deployment triggers
- **Environment Separation**: Production, staging, and development configs

## [Unreleased]

### Planned Features

- [ ] Real-time transaction monitoring
- [ ] Advanced analytics dashboard
- [ ] Custom alerting system
- [ ] Multi-language support expansion
- [ ] Mobile-optimized views
- [ ] GraphQL API integration
- [ ] WebSocket support for live updates
- [ ] Enhanced search capabilities
- [ ] Account watchlist feature
- [ ] Historical data visualization

### Future Improvements

- [ ] Performance optimization for large datasets
- [ ] Improved caching strategies
- [ ] Progressive Web App (PWA) support
- [ ] Dark mode theme
- [ ] Accessibility improvements (WCAG 2.1 AAA)
- [ ] API rate limiting UI feedback
- [ ] Batch operation analysis
- [ ] Custom dashboard widgets

---

## Version History

### Versioning Scheme

This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for added functionality in a backward compatible manner
- **PATCH** version for backward compatible bug fixes

### Release Tags

- `v*-app`: Web application releases
- `v*-api`: API service releases

### Credits

Based on [StellarExplorer](https://github.com/chatch/stellarexplorer) by Chris Hatch.

Maintained by the CosmicLedger Team.

---

[1.0.0]: https://github.com/CosmicLedger/stellar-chain-explorer/releases/tag/v1.0.0
