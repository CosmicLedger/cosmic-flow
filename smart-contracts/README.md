# Soroban Smart Contracts

This directory contains all Soroban smart contracts for the Stellar Chain Explorer project.

## Quick Start

### Build Contracts

```bash
# Build all contracts
cargo build --release --target wasm32-unknown-unknown

# Build specific contract
cd contracts/token
cargo build --release --target wasm32-unknown-unknown
```

### Test Contracts

```bash
# Run all tests
./test-contracts.sh

# Or manually
cargo test
```

### Deploy Contracts

```bash
# Deploy to testnet
./deploy.sh testnet deployer

# Deploy to futurenet
./deploy.sh futurenet deployer

# Deploy to mainnet (requires proper configuration)
./deploy.sh mainnet deployer
```

## Contracts

- **token**: Fungible token contract (ERC-20 style)
- **nft**: Non-fungible token contract
- **defi-pool**: Automated market maker liquidity pool
- **governance**: DAO governance with voting

## Documentation

See [SMART_CONTRACTS.md](../SMART_CONTRACTS.md) for comprehensive documentation.

## License

Apache-2.0
