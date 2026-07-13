# Quick Reference Guide

## 🚀 Essential Commands

### JavaScript/TypeScript (Frontend)
```bash
pnpm dev              # Start development server
pnpm build            # Build for production
pnpm start            # Start production server
```

### Rust (Indexer - Optional)
```bash
cd rust-indexer
cargo run             # Run indexer in dev mode
cargo build --release # Build optimized binary
cargo test            # Run tests
cargo fmt             # Format code
cargo clippy          # Run linter
```

### Testing
```bash
pnpm test                    # Run unit tests
pnpm test -- --coverage      # Run with coverage
pnpm test:watch             # Watch mode
pnpm test:e2e               # E2E tests
pnpm test:e2e:ui            # E2E UI mode
```

### Code Quality
```bash
pnpm lint             # Check linting
pnpm lint:fix         # Fix linting errors
pnpm prettier         # Check formatting
pnpm prettier:fix     # Fix formatting
pnpm typecheck        # Type checking
pnpm fixall           # Fix all issues
```

### Docker
```bash
docker-compose up                   # Start production
docker-compose --profile dev up     # Start development
docker build -t stellar-explorer .  # Build image
```

### Deployment
```bash
pnpm pages:deploy     # Deploy to Cloudflare Pages
git tag v1.0.0-app    # Tag for auto-deployment
```

## 📁 Key Files

| File | Purpose |
|------|---------|
| `package.json` | Project configuration and dependencies |
| `.env.example` | Environment variables template |
| `jest.config.ts` | Test configuration |
| `playwright.config.ts` | E2E test configuration |
| `Dockerfile` | Production container |
| `docker-compose.yml` | Docker services |

## 🌐 URLs (After Deployment)

| Environment | URL |
|-------------|-----|
| Production | https://explorer.cosmicledger.io |
| Testnet | https://testnet.cosmicledger.io |
| Futurenet | https://futurenet.cosmicledger.io |
| API | https://api.cosmicledger.io |
| Local | http://localhost:3000 |

## 🔑 Required Secrets (GitHub)

- `CODECOV_TOKEN`
- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`
- `FLY_API_TOKEN`

## 📊 CI/CD Workflows

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `node.js.yml` | Push to main/develop | Main CI pipeline |
| `deploy.yml` | Version tags | Deployment |
| `code-quality.yml` | Push/PR | Quality checks |
| `performance.yml` | Push/Schedule | Performance monitoring |
| `codeql.yml` | Push/Schedule | Security scanning |

## 🐛 Common Issues

### Port 3000 in use
```bash
PORT=3001 pnpm dev
```

### Clear cache
```bash
rm -rf node_modules .cache build
pnpm install
```

### Update snapshots
```bash
pnpm test -- -u
```

## 📚 Documentation

- **Setup**: [SETUP.md](SETUP.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Full Docs**: [README.md](README.md)
- **Changes**: [CHANGELOG.md](CHANGELOG.md)
- **Summary**: [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)

## 💬 Support

- Email: support@cosmicledger.io
- Issues: https://github.com/CosmicLedger/stellar-chain-explorer/issues
- Discussions: https://github.com/CosmicLedger/stellar-chain-explorer/discussions

---

**Quick Tip**: Press `Ctrl+K` (or `Cmd+K`) in most editors to search files quickly!
