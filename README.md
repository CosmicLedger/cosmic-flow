# Stellar Chain Explorer

[![Node.js CI](https://github.com/CosmicLedger/stellar-chain-explorer/actions/workflows/node.js.yml/badge.svg)](https://github.com/CosmicLedger/stellar-chain-explorer/actions/workflows/node.js.yml)
[![Run e2e tests against production](https://github.com/CosmicLedger/stellar-chain-explorer/actions/workflows/playwright-production.yml/badge.svg)](https://github.com/CosmicLedger/stellar-chain-explorer/actions/workflows/playwright-production.yml)
[![codecov](https://codecov.io/gh/CosmicLedger/stellar-chain-explorer/branch/main/graph/badge.svg)](https://codecov.io/gh/CosmicLedger/stellar-chain-explorer)

An advanced blockchain explorer for [Stellar](https://stellar.org) with comprehensive monitoring and analytics capabilities.

Deployments:

- Public: <https://explorer.cosmicledger.io>
- Test: <https://testnet.cosmicledger.io>
- Futurenet: <https://futurenet.cosmicledger.io>
- Local: <http://localhost:3000>

## Resources

### Lists

| Resource           | URI                                               |
| ------------------ | ------------------------------------------------- |
| Operations         | [/operations](https://steexp.com/operations)      |
| Transactions       | [/txs](https://steexp.com/txs)                    |
| Ledgers            | [/ledgers](https://steexp.com/ledgers)            |
| Payments           | [/payments](https://steexp.com/payments)          |
| Trades             | [/trades](https://steexp.com/trades)              |
| Effects            | [/effects](https://steexp.com/effects)            |
| Liquidity Pools    | [/effects](https://steexp.com/pools)              |
| Claimable Balances | [/effects](https://steexp.com/claimable-balances) |

### Directory

| Resource  | URI                                        |
| --------- | ------------------------------------------ |
| Assets    | [/assets](https://steexp.com/assets)       |
| Anchors   | [/anchors](https://steexp.com/anchors)     |
| Exchanges | [/exchanges](https://steexp.com/exchanges) |

### Accounts

| Resource               | URI                                                                                                                                                                                |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| by Federated address   | [/account/stellar\*fed.network](https://steexp.com/account/stellar*fed.network)                                                                                                    |
| by Public address      | [/account/GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX](https://steexp.com/account/GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX)                           |
| by Multiplexed address | [/account/MDZ464OWNGEL4X2DE6JPLEARO2WJ4AGCBN3XM7E4ZSLPHRBV6AZB6AAAAAAAAAAAAGW4M](https://steexp.com/account/MDZ464OWNGEL4X2DE6JPLEARO2WJ4AGCBN3XM7E4ZSLPHRBV6AZB6AAAAAAAAAAAAGW4M) |

#### Tabs

| Resource         | URI                                                                                                       |
| ---------------- | --------------------------------------------------------------------------------------------------------- |
| Balances Tab     | [/account/stellar\*fed.network#balances](https://steexp.com/account/stellar*fed.network#balances)         |
| Payments Tab     | [/account/stellar\*fed.network#payments](https://steexp.com/account/stellar*fed.network#payments)         |
| Offers Tab       | [/account/stellar\*fed.network#offers](https://steexp.com/account/stellar*fed.network#offers)             |
| Trades Tab       | [/account/stellar\*fed.network#trades](https://steexp.com/account/stellar*fed.network#trades)             |
| Effects Tab      | [/account/stellar\*fed.network#effects](https://steexp.com/account/stellar*fed.network#effects)           |
| Operations Tab   | [/account/stellar\*fed.network#operations](https://steexp.com/account/stellar*fed.network#operations)     |
| Transactions Tab | [/account/stellar\*fed.network#transactions](https://steexp.com/account/stellar*fed.network#transactions) |
| Signing Tab      | [/account/stellar\*fed.network#signing](https://steexp.com/account/stellar*fed.network#signing)           |
| Flags Tab        | [/account/stellar\*fed.network#flags](https://steexp.com/account/stellar*fed.network#flags)               |
| Data Tab         | [/account/stellar\*fed.network#data](https://steexp.com/account/stellar*fed.network#data)                 |

### Search

| Resource              | URI                                                                                                                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Federated address     | [/search/steexp\*fed.network](https://steexp.com/search/steexp*fed.network)                                                                                                      |
| Public address        | [/search/GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX](https://steexp.com/search/GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX)                           |
| Multiplexed address   | [/search/MDZ464OWNGEL4X2DE6JPLEARO2WJ4AGCBN3XM7E4ZSLPHRBV6AZB6AAAAAAAAAAAAGW4M](https://steexp.com/search/MDZ464OWNGEL4X2DE6JPLEARO2WJ4AGCBN3XM7E4ZSLPHRBV6AZB6AAAAAAAAAAAAGW4M) |
| Ledger                | [/search/10000000](https://steexp.com/search/10000000)                                                                                                                           |
| Transaction           | [/search/26a568681712a44a515b2c90dcfaadb3ed2c40dc60254638407937bee4767071](https://steexp.com/search/26a568681712a44a515b2c90dcfaadb3ed2c40dc60254638407937bee4767071)           |
| Asset Code            | [/search/NGN](https://steexp.com/search/NGN)                                                                                                                                     |
| Anchor Name           | [/search/ripplefox](https://steexp.com/search/ripplefox)                                                                                                                         |
| Anchor Name (Partial) | [/search/fox](https://steexp.com/search/fox)                                                                                                                                     |

### Misc

| Resource    | URI                                                                                                                                                            |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Transaction | [/tx/26a568681712a44a515b2c90dcfaadb3ed2c40dc60254638407937bee4767071](https://steexp.com/tx/26a568681712a44a515b2c90dcfaadb3ed2c40dc60254638407937bee4767071) |
| Ledger      | [/ledger/10000000](https://steexp.com/ledger/10000000)                                                                                                         |
| Anchor      | [/anchor/apay.io](https://steexp.com/anchor/apay.io)                                                                                                           |
| Asset       | [/asset/NGN](https://steexp.com/asset/NGN)                                                                                                                     |

## Exploring Private / Local Development Networks<a name="private-networks"></a>

steexp will connect to a local horizon instance at <http://localhost:8000> by default. If your running a local private network for development this is quite handy for browsing your changes to the ledger.

Alternatively you can run locally connecting to the testnet or public network horizon instances. To do this define these aliases to localhost:

```sh
127.0.1.1  testnet.local     # for steexp use testnet horizon
127.0.1.1  publicnet.local   # for steexp use mainnet horizon
127.0.1.1  futurenet.local   # for steexp use future horizon
```

> Use 127.0.0.1 instead of 127.0.1.1 when the above doesn't work.

Navigate to <http://testnet.local:3000>, <http://futurenet.local:3000> or <http://publicnet.local:3000> to select the network your interesting in exploring.

## Development

See the section [Exploring Private / Local Development Networks](#private-networks) for connecting to different backend networks. By default steexp will look for a local instance of horizon.

### Prerequisites

- Node.js >= 24.0.0
- pnpm >= 11.8.0

### Quick Start

```sh
# Install dependencies
pnpm install

# Start development server
pnpm dev

# Open http://localhost:3000 in your browser
```

### Available Scripts

```sh
# Development
pnpm dev              # Start development server
pnpm build            # Build for production
pnpm start            # Start production server

# Quality Checks
pnpm lint             # Run ESLint
pnpm lint:fix         # Fix ESLint errors
pnpm prettier         # Check code formatting
pnpm prettier:fix     # Fix formatting issues
pnpm typecheck        # Run TypeScript type checking
pnpm fixall           # Fix all linting and formatting issues

# Testing
pnpm test             # Run unit tests
pnpm test:watch       # Run tests in watch mode
pnpm test:coverage    # Run tests with coverage
pnpm test:e2e         # Run E2E tests
pnpm test:e2e:ui      # Run E2E tests in UI mode
pnpm test:e2e:report  # Show E2E test report

# Deployment
pnpm pages:dev        # Preview with Wrangler
pnpm pages:deploy     # Deploy to Cloudflare Pages
```

### Docker Development

```sh
# Build and run with Docker
docker-compose up

# Development mode
docker-compose --profile dev up

# Build only
docker build -t stellar-chain-explorer .
```

### Environment Variables

Create a `.env` file in the root directory:

```env
# Horizon API endpoints
HORIZON_PUBLIC=https://horizon.stellar.org
HORIZON_TEST=https://horizon-testnet.stellar.org
HORIZON_FUTURE=https://horizon-futurenet.stellar.org

# Sentry (optional)
SENTRY_DSN=your_sentry_dsn

# Analytics (optional)
GOOGLE_ANALYTICS_ID=your_ga_id
```

## Test

### Unit Testing

```sh
# Run all tests
pnpm test

# Run tests with coverage
pnpm test -- --coverage

# Run tests in watch mode
pnpm test:watch

# Run specific test file
pnpm test path/to/test.test.ts
```

or

You can run test with Jest Preview with the following command. You can see the result at <http://locahost:3336> for test using React Testing Library along with Jest Preview.

```sh
pnpm test:preview
```

or

You can run test and Jest Preview separately as follows (this is basically same as the previous one).

```sh
pnpm jest-preview

pnpm test
or
pnpm test:watch
```

### Coverage Requirements

- Minimum 50% overall coverage
- All new features must include tests
- Critical paths should have >80% coverage

### E2E Testing

```sh
# Run E2E tests
pnpm test:e2e

# Run with UI mode
pnpm test:e2e:ui

# Show test report
pnpm test:e2e:report

# Run specific test
pnpm test:e2e tests/homepage.spec.ts

# Debug mode
pnpm test:e2e --debug
```

### E2E Test

```sh
pnpm test:e2e
```

Run the test in UI mode:

```sh
pnpm test:e2e:ui
```

Show HTML test reports:

```sh
pnpm test:e2e:report
```

### Build

```sh
pnpm i && pnpm build
```

## Languages

Use the language selector in the top right corner to change the language.

Translation files are here:
<https://github.com/CosmicLedger/stellar-chain-explorer/tree/main/app/languages>

Submit pull requests with new languages or language fixes there.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on:

- Code of conduct
- Development workflow
- Testing requirements
- Code style guidelines
- Pull request process

### Quick Contribution Guide

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`pnpm test`)
5. Commit your changes (`git commit -m 'feat: add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Architecture

### Tech Stack

**Frontend & Web Application:**
- **Framework:** [Remix](https://remix.run/) - Full-stack web framework
- **UI Library:** [React](https://react.io/) - Component-based UI
- **Styling:** [Bootstrap](https://getbootstrap.com/) - Responsive CSS framework
- **Blockchain SDK:** [Stellar SDK](https://github.com/stellar/js-stellar-sdk) - Stellar network interaction
- **Type Safety:** [TypeScript](https://www.typescriptlang.org/) - Static type checking
- **Testing:** [Jest](https://jestjs.io/) + [Playwright](https://playwright.dev/) - Unit and E2E testing
- **Build Tool:** [Vite](https://vitejs.dev/) - Fast build tooling
- **Package Manager:** [pnpm](https://pnpm.io/) - Efficient dependency management

**Backend Indexer (Rust):**
- **Language:** [Rust](https://www.rust-lang.org/) - High-performance systems language
- **Web Framework:** [Actix-web](https://actix.rs/) - Fast async web framework
- **Database:** [SQLx](https://github.com/launchbadge/sqlx) - Async PostgreSQL driver
- **Async Runtime:** [Tokio](https://tokio.rs/) - Asynchronous runtime
- **Caching:** [Redis](https://redis.io/) - In-memory data store

### Project Structure

```
stellar-chain-explorer/
├── .github/              # CI/CD workflows and templates
│   ├── workflows/        # GitHub Actions
│   └── ISSUE_TEMPLATE/   # Issue templates
├── api/                  # Backend API service (Node.js)
├── rust-indexer/         # High-performance Rust indexer
│   ├── src/              # Rust source code
│   ├── migrations/       # Database migrations
│   ├── Cargo.toml        # Rust dependencies
│   └── Dockerfile        # Rust service container
├── app/                  # Frontend application
│   ├── components/       # Reusable React components
│   ├── routes/           # Remix route modules
│   ├── lib/              # Utility functions and helpers
│   ├── languages/        # Internationalization files
│   └── styles/           # CSS and styling
├── e2e/                  # End-to-end tests
├── public/               # Static assets
├── scripts/              # Build and utility scripts
└── build/                # Build output (generated)
```

## Deployment

### Cloudflare Pages (Web App)

```sh
# Deploy to production
pnpm pages:deploy

# Environment variables needed:
# - CLOUDFLARE_API_TOKEN
# - CLOUDFLARE_ACCOUNT_ID
```

### Fly.io (API)

```sh
# Deploy API to Fly.io
cd api
flyctl deploy

# Environment variables needed:
# - FLY_API_TOKEN
```

### Docker Deployment

```sh
# Build image
docker build -t cosmicledger/stellar-chain-explorer .

# Run container
docker run -p 3000:3000 cosmicledger/stellar-chain-explorer

# Or use docker-compose
docker-compose up -d
```

## Monitoring and Observability

### Error Tracking

The application uses [Sentry](https://sentry.io/) for error tracking. Configure with:

```env
SENTRY_DSN=your_sentry_dsn
SENTRY_ENVIRONMENT=production
```

### Performance Monitoring

- Lighthouse CI for performance metrics
- Bundle size analysis in CI/CD
- E2E test performance tracking

### Health Checks

- Web: `GET /health` (if implemented)
- API: `GET /` returns 200 or 404

## Security

### Reporting Security Issues

Please report security vulnerabilities to: security@cosmicledger.io

Do not open public issues for security problems.

### Security Practices

- Regular dependency audits (`pnpm audit`)
- CodeQL scanning in CI/CD
- Trivy vulnerability scanning
- No secrets in code or git history
- Environment variable management

## Performance

### Optimization Techniques

- Code splitting with Remix
- Asset optimization with Vite
- Lazy loading of heavy components
- Efficient bundle size management
- CDN distribution via Cloudflare

### Performance Targets

- Lighthouse Score: >90
- First Contentful Paint: <1.5s
- Time to Interactive: <3.5s
- Bundle Size: <500KB (gzipped)

## Browser Support

- Chrome (last 2 versions)
- Firefox (last 2 versions)
- Safari (last 2 versions)
- Edge (last 2 versions)

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original [StellarExplorer](https://github.com/chatch/stellarexplorer) by Chris Hatch
- [Stellar Development Foundation](https://stellar.org/)
- All [contributors](https://github.com/CosmicLedger/stellar-chain-explorer/graphs/contributors)

## Support

- 📧 Email: support@cosmicledger.io
- 💬 Discussions: [GitHub Discussions](https://github.com/CosmicLedger/stellar-chain-explorer/discussions)
- 🐛 Bug Reports: [GitHub Issues](https://github.com/CosmicLedger/stellar-chain-explorer/issues)
- 📖 Documentation: [Wiki](https://github.com/CosmicLedger/stellar-chain-explorer/wiki)

## Roadmap

See our [project roadmap](https://github.com/CosmicLedger/stellar-chain-explorer/projects) for upcoming features and improvements.

---

Made with ❤️ by the CosmicLedger Team

## Scripts

### Fetch and restructure centralized exchanges data

```sh
pnpm exec ts-node scripts/restructure-centralized-exchanges-json.ts
```
