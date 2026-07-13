# Stellar Chain Explorer - Setup Guide

## 🚀 Quick Start

This is a complete, production-ready blockchain explorer for the Stellar network, rebranded from the original StellarExplorer project.

### New Branding

- **Organization:** CosmicLedger
- **Repository:** stellar-chain-explorer
- **Package Name:** @cosmicledger/stellar-chain-explorer
- **Homepage:** https://explorer.cosmicledger.io

## 📋 Prerequisites

Before you begin, ensure you have:

- **Node.js** >= 24.0.0
- **pnpm** >= 11.8.0
- **Git** for version control

### Installing Prerequisites

#### Windows

```powershell
# Install Node.js from https://nodejs.org/
# Or use Windows Package Manager
winget install OpenJS.NodeJS.LTS

# Install pnpm
npm install -g pnpm
```

#### macOS

```bash
# Using Homebrew
brew install node
brew install pnpm
```

#### Linux

```bash
# Using nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 24
nvm use 24

# Install pnpm
npm install -g pnpm
```

## 🔧 Installation

1. **Clone the repository:**

```bash
git clone https://github.com/CosmicLedger/stellar-chain-explorer.git
cd stellar-chain-explorer
```

2. **Install dependencies:**

```bash
pnpm install
```

3. **Set up environment variables:**

```bash
cp .env.example .env
# Edit .env with your configuration
```

4. **Run the development server:**

```bash
pnpm dev
```

5. **Open your browser:**

Navigate to `http://localhost:3000`

## 🧪 Testing

### Run All Tests

```bash
# Unit tests
pnpm test

# Unit tests with coverage
pnpm test -- --coverage

# E2E tests
pnpm test:e2e

# E2E tests in UI mode
pnpm test:e2e:ui
```

### Test Coverage

The project includes comprehensive test coverage:

- **Unit Tests:** Jest + React Testing Library
- **E2E Tests:** Playwright
- **Coverage Reporting:** Codecov integration
- **Minimum Coverage:** 50% (configurable in jest.config.ts)

### Running Specific Tests

```bash
# Run a specific test file
pnpm test path/to/test.test.ts

# Run tests matching a pattern
pnpm test --testNamePattern="Component name"

# Watch mode
pnpm test:watch
```

## 🏗️ Building

### Production Build

```bash
# Build the application
pnpm build

# Preview the build
pnpm start
```

### Build Output

The build process creates:
- `build/client` - Client-side assets
- `build/server` - Server-side code

## 🐳 Docker Setup

### Using Docker

```bash
# Build the image
docker build -t stellar-chain-explorer .

# Run the container
docker run -p 3000:3000 stellar-chain-explorer
```

### Using Docker Compose

```bash
# Production mode
docker-compose up

# Development mode
docker-compose --profile dev up

# Stop containers
docker-compose down
```

## 🔍 Code Quality

### Linting

```bash
# Check for linting errors
pnpm lint

# Auto-fix linting errors
pnpm lint:fix
```

### Formatting

```bash
# Check code formatting
pnpm prettier

# Auto-fix formatting
pnpm prettier:fix
```

### Type Checking

```bash
# Run TypeScript type checking
pnpm typecheck
```

### Fix All Issues

```bash
# Fix linting and formatting in one command
pnpm fixall
```

## 🚀 Deployment

### Cloudflare Pages (Web App)

```bash
# Deploy to Cloudflare Pages
pnpm pages:deploy
```

Required environment variables:
- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`

### Fly.io (API)

```bash
# From the api directory
cd api
flyctl deploy
```

Required environment variable:
- `FLY_API_TOKEN`

### Manual Deployment

1. Build the application: `pnpm build`
2. Upload `build/client` to your hosting provider
3. Set up environment variables
4. Start the server: `pnpm start`

## 🔄 CI/CD

The project includes comprehensive GitHub Actions workflows:

### Workflows

1. **node.js.yml** - Main CI pipeline
   - Runs on push to `main` and `develop`
   - Executes linting, type checking, tests
   - Generates code coverage reports
   - Runs E2E tests in parallel

2. **deploy.yml** - Deployment pipeline
   - Triggered by version tags (`v*-app`, `v*-api`)
   - Deploys to Cloudflare Pages and Fly.io
   - Includes smoke tests

3. **code-quality.yml** - Code quality checks
   - ESLint and Prettier validation
   - Bundle size analysis
   - License compliance checking
   - Dependency review

4. **performance.yml** - Performance monitoring
   - Lighthouse CI
   - Bundle analysis
   - Weekly scheduled runs

5. **codeql.yml** - Security scanning
   - CodeQL analysis
   - Vulnerability detection

### Setting Up CI/CD

1. Fork the repository
2. Add required secrets in GitHub Settings > Secrets:
   - `CODECOV_TOKEN`
   - `CLOUDFLARE_API_TOKEN`
   - `CLOUDFLARE_ACCOUNT_ID`
   - `FLY_API_TOKEN`
   - `SENTRY_AUTH_TOKEN` (optional)

3. Push to `main` or create a PR to trigger workflows

## 📦 Project Structure

```
stellar-chain-explorer/
├── .github/              # CI/CD workflows and templates
│   ├── workflows/        # GitHub Actions workflows
│   │   ├── node.js.yml         # Main CI pipeline
│   │   ├── deploy.yml          # Deployment pipeline
│   │   ├── code-quality.yml    # Code quality checks
│   │   ├── performance.yml     # Performance monitoring
│   │   └── codeql.yml          # Security scanning
│   ├── ISSUE_TEMPLATE/   # Issue templates
│   └── pull_request_template.md
├── api/                  # Backend API service
├── app/                  # Frontend React application
│   ├── components/       # Reusable components
│   ├── routes/           # Remix routes
│   ├── lib/              # Utilities and helpers
│   ├── languages/        # i18n translations
│   ├── test-utils.tsx    # Testing utilities
│   └── styles/           # CSS styles
├── e2e/                  # E2E tests (Playwright)
├── public/               # Static assets
├── scripts/              # Build scripts
├── .env.example          # Environment variables template
├── jest.config.ts        # Jest configuration
├── playwright.config.ts  # Playwright configuration
├── docker-compose.yml    # Docker Compose config
├── Dockerfile            # Production Dockerfile
├── Dockerfile.dev        # Development Dockerfile
├── CONTRIBUTING.md       # Contribution guidelines
├── SETUP.md             # This file
└── README.md            # Main documentation
```

## 🔐 Environment Variables

Copy `.env.example` to `.env` and configure:

### Required Variables

```env
NODE_ENV=development
HORIZON_PUBLIC=https://horizon.stellar.org
HORIZON_TEST=https://horizon-testnet.stellar.org
HORIZON_FUTURE=https://horizon-futurenet.stellar.org
```

### Optional Variables

```env
# Error tracking
SENTRY_DSN=your_sentry_dsn

# Analytics
GOOGLE_ANALYTICS_ID=your_ga_id

# Feature flags
ENABLE_ANALYTICS=false
ENABLE_BETA_FEATURES=false

# API configuration
API_BASE_URL=https://api.cosmicledger.io
API_TIMEOUT=30000
```

## 🌐 Network Configuration

### Connecting to Different Networks

By default, the explorer connects to `http://localhost:8000` for local development.

To connect to Testnet or Mainnet locally, add these to your hosts file:

```
127.0.1.1  testnet.local      # Testnet
127.0.1.1  publicnet.local    # Mainnet
127.0.1.1  futurenet.local    # Futurenet
```

Then access:
- Testnet: `http://testnet.local:3000`
- Mainnet: `http://publicnet.local:3000`
- Futurenet: `http://futurenet.local:3000`

## 🐛 Troubleshooting

### Common Issues

#### 1. Node version mismatch

```bash
# Check Node version
node --version

# Should be >= 24.0.0
# If not, upgrade Node.js
```

#### 2. pnpm not found

```bash
# Install pnpm globally
npm install -g pnpm
```

#### 3. Port 3000 already in use

```bash
# Find process using port 3000
# Windows
netstat -ano | findstr :3000

# macOS/Linux
lsof -i :3000

# Kill the process or use a different port
PORT=3001 pnpm dev
```

#### 4. Build errors

```bash
# Clear cache and reinstall
rm -rf node_modules .cache build
pnpm install
pnpm build
```

#### 5. Test failures

```bash
# Update snapshots
pnpm test -- -u

# Clear Jest cache
pnpm test -- --clearCache
```

## 📚 Additional Resources

- [Contributing Guide](CONTRIBUTING.md)
- [Main Documentation](README.md)
- [Stellar Documentation](https://developers.stellar.org/)
- [Remix Documentation](https://remix.run/docs)
- [React Documentation](https://react.dev/)

## 💬 Getting Help

- 📧 Email: support@cosmicledger.io
- 💬 [GitHub Discussions](https://github.com/CosmicLedger/stellar-chain-explorer/discussions)
- 🐛 [Issue Tracker](https://github.com/CosmicLedger/stellar-chain-explorer/issues)
- 📖 [Wiki](https://github.com/CosmicLedger/stellar-chain-explorer/wiki)

## 📝 Next Steps

1. ✅ Complete the setup above
2. ✅ Run the development server
3. ✅ Explore the codebase
4. ✅ Read [CONTRIBUTING.md](CONTRIBUTING.md)
5. ✅ Make your first contribution!

---

**Happy coding! 🎉**

Made with ❤️ by the CosmicLedger Team
