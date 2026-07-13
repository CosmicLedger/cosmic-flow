# 👋 Welcome to Stellar Chain Explorer!

## 🎯 You're in the Right Place!

This is a **complete, production-ready** blockchain explorer for the Stellar network, fully rebranded as **CosmicLedger's Stellar Chain Explorer**.

## ⚡ 3-Minute Quick Start

### Step 1: Prerequisites (2 minutes)

**Check if you have Node.js and pnpm:**

```bash
node --version  # Should be >= 24.0.0
pnpm --version  # Should be >= 11.8.0
```

**Don't have them?**
- Install Node.js from [nodejs.org](https://nodejs.org/)
- Then install pnpm: `npm install -g pnpm`

### Step 2: Install & Run (1 minute)

```bash
# Install dependencies
pnpm install

# Start the development server
pnpm dev
```

**That's it!** Open [http://localhost:3000](http://localhost:3000) in your browser.

## 📚 What to Read Next?

### Choose Your Path:

#### 🚀 I want to start developing ASAP
→ Read: [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - All essential commands in one place

#### 📖 I want to understand the complete project
→ Read: [README.md](README.md) - Complete project documentation

#### 🔧 I need detailed setup instructions
→ Read: [SETUP.md](SETUP.md) - Step-by-step setup guide

#### 🤝 I want to contribute
→ Read: [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines

#### 📊 I want the executive summary
→ Read: [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - Complete overview

#### 🗺️ I want to see everything
→ Read: [INDEX.md](INDEX.md) - Complete project index

## 🎨 What Makes This Special?

### ✅ Complete Rebranding
- **Organization**: CosmicLedger  
- **New Name**: Stellar Chain Explorer
- **Professional URLs**: explorer.cosmicledger.io

### ✅ Production Ready
- Full CI/CD with 5 GitHub Actions workflows
- Automated testing and deployment
- Security scanning and monitoring
- Docker containerization

### ✅ Developer Friendly
- TypeScript for type safety
- Hot reload in development
- Comprehensive test suite
- Modern tooling (Vite, pnpm, SWC)

### ✅ Well Documented
- 8+ documentation files
- Code examples everywhere
- Issue and PR templates
- Architecture overview

## 🧪 Test It Works

```bash
# Run tests
pnpm test

# Check code quality
pnpm lint

# Build for production
pnpm build
```

All green? **You're good to go!** 🎉

## 🆘 Having Issues?

### Common Problems & Solutions

#### ❌ "pnpm: command not found"
```bash
npm install -g pnpm
```

#### ❌ "Port 3000 is already in use"
```bash
PORT=3001 pnpm dev
```

#### ❌ "Module not found"
```bash
rm -rf node_modules
pnpm install
```

#### ❌ Still stuck?
- Check [SETUP.md](SETUP.md) troubleshooting section
- Open an [issue](https://github.com/CosmicLedger/stellar-chain-explorer/issues)
- Email: support@cosmicledger.io

## 🎯 Quick Commands Cheat Sheet

```bash
# Development
pnpm dev              # Start dev server
pnpm build            # Build for production
pnpm start            # Start production server

# Testing
pnpm test             # Run unit tests
pnpm test:e2e         # Run E2E tests
pnpm test -- --coverage   # With coverage

# Code Quality
pnpm lint             # Check linting
pnpm lint:fix         # Fix linting errors
pnpm prettier:fix     # Fix formatting
pnpm typecheck        # Type checking
pnpm fixall           # Fix everything

# Docker
docker-compose up     # Start with Docker
```

## 📁 Project Structure (Simplified)

```
stellar-chain-explorer/
├── app/              # Frontend React app
├── api/              # Backend API
├── e2e/              # E2E tests
├── .github/          # CI/CD workflows
├── public/           # Static files
└── build/            # Built files (generated)
```

## 🚀 Deployment

### Automatic (Recommended)
Push a version tag and GitHub Actions handles everything:

```bash
git tag v1.0.0-app
git push origin v1.0.0-app
```

### Manual
```bash
pnpm build
pnpm pages:deploy
```

More details in [README.md](README.md#deployment)

## 🌟 What Can You Do?

### For Developers
- ✅ Explore Stellar blockchain data
- ✅ View transactions and ledgers
- ✅ Search accounts and assets
- ✅ Monitor network activity
- ✅ Analyze operations and effects

### For Contributors
- ✅ Fix bugs and improve features
- ✅ Add new functionality
- ✅ Improve documentation
- ✅ Enhance performance
- ✅ Write tests

### For Maintainers
- ✅ Configure deployments
- ✅ Monitor application health
- ✅ Review and merge PRs
- ✅ Create releases
- ✅ Manage issues

## 📊 Current Status

| Aspect | Status |
|--------|--------|
| Tests | ✅ Passing |
| CI/CD | ✅ Configured |
| Documentation | ✅ Complete |
| Docker | ✅ Ready |
| Security | ✅ Scanned |
| Production | ✅ Ready |

## 🔗 Important Links

### Documentation
- **Complete Docs**: [README.md](README.md)
- **Setup Guide**: [SETUP.md](SETUP.md)
- **Quick Reference**: [QUICK_REFERENCE.md](QUICK_REFERENCE.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)

### Online
- **Repository**: https://github.com/CosmicLedger/stellar-chain-explorer
- **Issues**: https://github.com/CosmicLedger/stellar-chain-explorer/issues
- **Live Site**: https://explorer.cosmicledger.io (after deployment)

### Support
- **Email**: support@cosmicledger.io
- **Security**: security@cosmicledger.io

## 🎓 Learning Path

### Day 1: Get Started
1. ✅ Run `pnpm dev` and explore
2. ✅ Read [QUICK_REFERENCE.md](QUICK_REFERENCE.md)
3. ✅ Make a small change and see it work
4. ✅ Run the tests

### Day 2: Understand the Project
1. ✅ Read [README.md](README.md)
2. ✅ Explore the codebase
3. ✅ Read [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)
4. ✅ Check out the CI/CD workflows

### Day 3: Contribute
1. ✅ Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. ✅ Find an issue to work on
3. ✅ Create a branch and make changes
4. ✅ Submit your first PR

## 💡 Pro Tips

1. **Bookmark** [QUICK_REFERENCE.md](QUICK_REFERENCE.md) for daily use
2. **Star** the repository to stay updated
3. **Watch** for notifications on new issues
4. **Join** discussions to connect with the community
5. **Share** feedback to help improve the project

## 🎉 Ready to Start?

**You now have everything you need!**

1. ✅ Project is installed
2. ✅ Documentation is comprehensive
3. ✅ CI/CD is configured
4. ✅ Tests are ready
5. ✅ You know where to get help

**Choose your next step from the "What to Read Next" section above!**

---

## ❤️ Thank You!

Thank you for using **Stellar Chain Explorer** by CosmicLedger!

We're excited to have you here. If you have any questions or feedback, don't hesitate to reach out.

**Happy coding!** 🚀

---

**Quick Links:**
- [README.md](README.md) | [SETUP.md](SETUP.md) | [QUICK_REFERENCE.md](QUICK_REFERENCE.md)
- [CONTRIBUTING.md](CONTRIBUTING.md) | [INDEX.md](INDEX.md) | [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)

---

Made with ❤️ by the CosmicLedger Team
