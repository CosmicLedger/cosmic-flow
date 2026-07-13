# Contributing to Stellar Chain Explorer

Thank you for your interest in contributing to Stellar Chain Explorer! This document provides guidelines and instructions for contributing to this project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Style](#code-style)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Node.js >= 24.0.0
- pnpm >= 11.8.0

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/stellar-chain-explorer.git
   cd stellar-chain-explorer
   ```

3. Install dependencies:
   ```bash
   pnpm install
   ```

4. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Running the Development Server

```bash
pnpm dev
```

The application will be available at `http://localhost:3000`.

### Building for Production

```bash
pnpm build
```

### Running Tests

```bash
# Run unit tests
pnpm test

# Run tests in watch mode
pnpm test:watch

# Run tests with coverage
pnpm test -- --coverage

# Run E2E tests
pnpm test:e2e

# Run E2E tests in UI mode
pnpm test:e2e:ui
```

## Testing

### Unit Tests

- Write unit tests for all new features and bug fixes
- Place test files next to the files they test with `.test.ts` or `.spec.ts` extension
- Aim for at least 80% code coverage for new code
- Use React Testing Library for component tests

Example:
```typescript
import { render, screen } from '@testing-library/react'
import { MyComponent } from './MyComponent'

describe('MyComponent', () => {
  it('renders correctly', () => {
    render(<MyComponent />)
    expect(screen.getByText('Hello')).toBeInTheDocument()
  })
})
```

### E2E Tests

- Write E2E tests for critical user flows
- Place E2E tests in the `e2e/` directory
- Use Playwright for E2E testing

Example:
```typescript
import { test, expect } from '@playwright/test'

test('homepage loads correctly', async ({ page }) => {
  await page.goto('/')
  await expect(page).toHaveTitle(/Stellar Chain Explorer/)
})
```

## Code Style

We use ESLint and Prettier to maintain code quality and consistency.

### Linting

```bash
# Check for linting errors
pnpm lint

# Fix linting errors automatically
pnpm lint:fix
```

### Formatting

```bash
# Check code formatting
pnpm prettier

# Fix formatting issues
pnpm prettier:fix
```

### Type Checking

```bash
# Run TypeScript type checking
pnpm typecheck
```

## Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes

### Examples

```
feat(ledger): add pagination support

fix(account): resolve balance display issue

docs(readme): update installation instructions

test(transaction): add unit tests for transaction parsing
```

## Pull Request Process

1. **Update your branch** with the latest changes from `main`:
   ```bash
   git fetch origin
   git rebase origin/main
   ```

2. **Run all checks locally**:
   ```bash
   pnpm lint
   pnpm typecheck
   pnpm test -- --coverage
   pnpm build
   ```

3. **Push your changes**:
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create a Pull Request**:
   - Go to the repository on GitHub
   - Click "New Pull Request"
   - Select your feature branch
   - Fill out the PR template with:
     - Description of changes
     - Related issues
     - Testing performed
     - Screenshots (if applicable)

5. **Address Review Comments**:
   - Make requested changes
   - Push additional commits
   - Request re-review when ready

6. **Merge**:
   - Once approved, a maintainer will merge your PR
   - Your branch will be deleted automatically

## Code Review Guidelines

### For Contributors

- Keep PRs focused and reasonably sized
- Write clear descriptions
- Add tests for new functionality
- Update documentation as needed
- Respond to feedback promptly

### For Reviewers

- Be constructive and respectful
- Focus on code quality and correctness
- Check for test coverage
- Verify documentation is updated
- Test changes locally when needed

## Project Structure

```
stellar-chain-explorer/
├── .github/          # GitHub Actions workflows
├── api/              # Backend API
├── app/              # Frontend application
│   ├── components/   # React components
│   ├── routes/       # Remix routes
│   ├── lib/          # Utility functions
│   └── styles/       # CSS styles
├── e2e/              # End-to-end tests
├── public/           # Static assets
└── scripts/          # Build and utility scripts
```

## Getting Help

- Check existing [issues](https://github.com/CosmicLedger/stellar-chain-explorer/issues)
- Join our community discussions
- Read the [documentation](https://github.com/CosmicLedger/stellar-chain-explorer/wiki)

## License

By contributing, you agree that your contributions will be licensed under the Apache-2.0 License.
