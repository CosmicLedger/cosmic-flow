# Soroban Smart Contracts

This directory contains Soroban smart contracts for the Stellar Chain Explorer project, providing advanced DeFi and governance capabilities on the Stellar network.

## 📦 Contracts Overview

### 1. Token Contract (`contracts/token/`)

A full-featured fungible token implementation following ERC-20 standards, adapted for Soroban.

**Features:**
- Minting and burning tokens
- Transfer and transferFrom with allowances
- Balance queries
- Total supply tracking
- Approval mechanism for delegated transfers

**Key Functions:**
```rust
pub fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32)
pub fn mint(env: Env, to: Address, amount: i128)
pub fn burn(env: Env, from: Address, amount: i128)
pub fn transfer(env: Env, from: Address, to: Address, amount: i128)
pub fn approve(env: Env, from: Address, spender: Address, amount: i128)
pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128)
pub fn balance(env: Env, account: Address) -> i128
pub fn total_supply(env: Env) -> i128
```

**Use Cases:**
- Custom project tokens
- Reward tokens for liquidity providers
- Governance tokens

### 2. NFT Contract (`contracts/nft/`)

A non-fungible token implementation with metadata support.

**Features:**
- Minting unique tokens with metadata
- Token ownership tracking
- Transfer functionality
- Burn mechanism
- Token metadata (URI support)
- Approval for individual tokens

**Key Functions:**
```rust
pub fn initialize(env: Env, admin: Address, name: String, symbol: String)
pub fn mint(env: Env, to: Address, token_id: u64, uri: String)
pub fn burn(env: Env, from: Address, token_id: u64)
pub fn transfer(env: Env, from: Address, to: Address, token_id: u64)
pub fn approve(env: Env, owner: Address, spender: Address, token_id: u64)
pub fn owner_of(env: Env, token_id: u64) -> Address
pub fn token_uri(env: Env, token_id: u64) -> String
pub fn balance_of(env: Env, owner: Address) -> u64
pub fn total_supply(env: Env) -> u64
```

**Use Cases:**
- Digital collectibles
- Asset tokenization
- Proof of ownership
- Achievement badges

### 3. DeFi Pool Contract (`contracts/defi-pool/`)

An automated market maker (AMM) liquidity pool for decentralized token swaps.

**Features:**
- Add/remove liquidity with LP token rewards
- Constant product AMM (x * y = k)
- Token swaps in both directions
- Configurable trading fees
- Slippage protection
- Price quote functions

**Key Functions:**
```rust
pub fn initialize(env: Env, admin: Address, token_a: Address, token_b: Address, pool_token: Address, fee_rate: u32)
pub fn add_liquidity(env: Env, sender: Address, amount_a: i128, amount_b: i128, min_shares: i128) -> i128
pub fn remove_liquidity(env: Env, sender: Address, shares: i128, min_amount_a: i128, min_amount_b: i128) -> (i128, i128)
pub fn swap_a_to_b(env: Env, sender: Address, amount_in: i128, min_amount_out: i128) -> i128
pub fn swap_b_to_a(env: Env, sender: Address, amount_in: i128, min_amount_out: i128) -> i128
pub fn get_pool_info(env: Env) -> PoolInfo
pub fn get_quote_a_to_b(env: Env, amount_in: i128) -> i128
pub fn get_quote_b_to_a(env: Env, amount_in: i128) -> i128
```

**Fee Structure:**
- Default: 0.3% (30 basis points)
- Configurable during initialization
- Fees accrue to liquidity providers

**Use Cases:**
- Decentralized token exchanges
- Providing liquidity for trading pairs
- Price discovery
- Arbitrage opportunities

### 4. Governance Contract (`contracts/governance/`)

A DAO governance system with token-weighted voting.

**Features:**
- Proposal creation and management
- Token-weighted voting power
- Quorum and threshold requirements
- Time-locked voting periods
- Proposal execution
- Vote tracking and transparency

**Key Functions:**
```rust
pub fn initialize(env: Env, admin: Address, gov_token: Address, voting_period: u32, quorum: u32, threshold: u32)
pub fn create_proposal(env: Env, proposer: Address, title: String, description: String, target: Address, call_data: Vec<u8>) -> u64
pub fn cast_vote(env: Env, voter: Address, proposal_id: u64, support: bool)
pub fn finalize_proposal(env: Env, proposal_id: u64) -> ProposalStatus
pub fn execute_proposal(env: Env, executor: Address, proposal_id: u64)
pub fn cancel_proposal(env: Env, caller: Address, proposal_id: u64)
pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal
pub fn get_vote(env: Env, proposal_id: u64, voter: Address) -> Option<Vote>
```

**Governance Parameters:**
- **Voting Period**: Duration in ledgers (e.g., 100 ledgers ≈ 8.3 minutes)
- **Quorum**: Minimum participation percentage (e.g., 40%)
- **Threshold**: Minimum approval percentage (e.g., 50%)

**Proposal Lifecycle:**
1. **Created** → Proposal submitted by token holder
2. **Active** → Voting period open
3. **Passed/Failed** → Finalized based on votes
4. **Executed** → Passed proposals are executed
5. **Canceled** → Optional cancellation by proposer/admin

**Use Cases:**
- Protocol parameter changes
- Treasury management
- Feature activation/deactivation
- Community-driven decisions

## 🚀 Quick Start

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/)
- **Soroban CLI**: Install Soroban tools
  ```bash
  cargo install --locked soroban-cli
  ```
- **Stellar Account**: For testnet/mainnet deployment

### Installation

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Verify installation
soroban --version
```

### Build Contracts

```bash
cd smart-contracts

# Build all contracts
cargo build --release --target wasm32-unknown-unknown

# Build specific contract
cd contracts/token
cargo build --release --target wasm32-unknown-unknown
```

### Run Tests

```bash
# Test all contracts
cargo test

# Test specific contract
cd contracts/token
cargo test

# Test with output
cargo test -- --nocapture
```

### Deploy to Testnet

```bash
# Configure Soroban for testnet
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate identity
soroban config identity generate deployer

# Get testnet XLM
soroban config identity fund deployer --network testnet

# Deploy token contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/token.wasm \
  --network testnet \
  --source deployer

# Initialize contract (replace CONTRACT_ID with deployed address)
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  --source deployer \
  -- initialize \
  --admin GXXXXXX \
  --name "My Token" \
  --symbol "MTK" \
  --decimals 7
```

## 🧪 Testing

### Unit Tests

Each contract includes comprehensive unit tests:

```bash
# Run all tests
cargo test

# Run tests with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Integration Tests

```bash
# Run integration tests
cd smart-contracts
cargo test --test integration_tests
```

### Local Testing with Stellar CLI

```bash
# Start local Stellar network
docker run --rm -it \
  -p 8000:8000 \
  --name stellar \
  stellar/quickstart:latest \
  --standalone \
  --enable-soroban-rpc

# Deploy and test contracts locally
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/token.wasm
```

## 📖 Development Guide

### Contract Development Workflow

1. **Write Contract Logic** (`src/lib.rs`)
   - Define data structures with `#[contracttype]`
   - Implement contract with `#[contract]` and `#[contractimpl]`
   - Use Soroban SDK types (Address, Env, etc.)

2. **Add Tests** (`src/lib.rs` or `tests/`)
   - Use `#[cfg(test)]` module
   - Test with `soroban_sdk::testutils`

3. **Build Contract**
   ```bash
   cargo build --release --target wasm32-unknown-unknown
   ```

4. **Optimize (Optional)**
   ```bash
   soroban contract optimize \
     --wasm target/wasm32-unknown-unknown/release/token.wasm
   ```

5. **Deploy**
   ```bash
   soroban contract deploy --wasm optimized.wasm --network testnet
   ```

### Best Practices

**Security:**
- Always validate inputs
- Use `require_auth()` for access control
- Check for integer overflow/underflow
- Implement reentrancy guards for critical functions
- Audit contracts before mainnet deployment

**Efficiency:**
- Minimize storage operations
- Use efficient data structures
- Optimize WASM size with `opt-level = "z"`
- Batch operations when possible

**Testing:**
- Achieve >80% code coverage
- Test edge cases and error conditions
- Use fuzzing for critical functions
- Test gas consumption

**Documentation:**
- Document all public functions
- Include usage examples
- Document security considerations
- Keep README updated

## 🔧 Contract Interaction

### Using Stellar SDK (JavaScript)

```javascript
import { Contract, SorobanRpc } from '@stellar/stellar-sdk';

// Connect to contract
const server = new SorobanRpc.Server('https://soroban-testnet.stellar.org');
const contract = new Contract(contractId);

// Call contract function
const result = await contract.call(
  'transfer',
  address('from_address'),
  address('to_address'),
  i128(1000)
);
```

### Using Soroban CLI

```bash
# Query balance
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  -- balance \
  --account GXXXXXX

# Transfer tokens
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  --source sender \
  -- transfer \
  --from GXXXXXX \
  --to GXXXXXX \
  --amount 1000
```

## 🔗 Integration with Frontend

The frontend application integrates with these contracts via:

**Location**: `app/lib/contracts/`

**Components**:
- `app/lib/contracts/token.ts` - Token contract client
- `app/lib/contracts/nft.ts` - NFT contract client
- `app/lib/contracts/defi-pool.ts` - DeFi pool client
- `app/lib/contracts/governance.ts` - Governance client
- `app/components/wallet/` - Wallet connection (Freighter)
- `app/routes/contracts/` - Contract interaction pages

**Example Usage**:
```typescript
import { TokenContract } from '~/lib/contracts/token';

// Initialize contract client
const tokenContract = new TokenContract(contractId, server);

// Call contract method
const balance = await tokenContract.balance(userAddress);
```

## 🔗 Integration with Backend API

The Node.js backend provides REST API endpoints for contract interaction:

**Location**: `api/routes/contracts.js`

**Endpoints**:
- `GET /api/contracts/:id` - Get contract info
- `POST /api/contracts/:id/invoke` - Invoke contract function
- `GET /api/contracts/:id/events` - Get contract events
- `GET /api/tokens` - List deployed tokens
- `GET /api/pools` - List liquidity pools
- `GET /api/governance/proposals` - List proposals

## 📊 Gas Optimization

### Contract Size

Minimize WASM size for lower deployment costs:

```toml
[profile.release]
opt-level = "z"           # Optimize for size
lto = true                # Link-time optimization
codegen-units = 1         # Single codegen unit
strip = "symbols"         # Strip debug symbols
```

### Storage Optimization

- Use compact data structures
- Implement storage cleanup
- Use temporary storage for ephemeral data
- Batch read/write operations

## 🔐 Security Considerations

### Auditing

Before mainnet deployment:
1. Internal code review
2. External security audit
3. Testnet deployment and monitoring
4. Bug bounty program
5. Gradual mainnet rollout

### Common Vulnerabilities

- **Reentrancy**: Prevented by Soroban's design
- **Integer Overflow**: Use checked arithmetic
- **Access Control**: Always use `require_auth()`
- **Front-running**: Consider time-locks and commit-reveal
- **Oracle Manipulation**: Use decentralized price feeds

## 📝 Contract Addresses

### Testnet

| Contract | Address | Status |
|----------|---------|--------|
| Token    | TBD     | Not deployed |
| NFT      | TBD     | Not deployed |
| DeFi Pool| TBD     | Not deployed |
| Governance| TBD    | Not deployed |

### Mainnet

| Contract | Address | Status |
|----------|---------|--------|
| Token    | TBD     | Not deployed |
| NFT      | TBD     | Not deployed |
| DeFi Pool| TBD     | Not deployed |
| Governance| TBD    | Not deployed |

*Update these after deployment*

## 🤝 Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for:
- Development setup
- Coding standards
- Testing requirements
- Pull request process

## 📚 Resources

**Soroban Documentation:**
- [Soroban Docs](https://soroban.stellar.org/docs)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Soroban SDK](https://docs.rs/soroban-sdk/)

**Stellar Resources:**
- [Stellar Docs](https://developers.stellar.org/)
- [Stellar Discord](https://discord.gg/stellardev)
- [Stellar Stack Exchange](https://stellar.stackexchange.com/)

**Security:**
- [Soroban Security Best Practices](https://soroban.stellar.org/docs/learn/security)
- [Smart Contract Security Checklist](https://github.com/stellar/soroban-docs/blob/main/docs/security-checklist.md)

## 📄 License

Apache-2.0 License - see [LICENSE](../LICENSE) for details.

## 🆘 Support

- 📧 Email: contracts@cosmicledger.io
- 💬 Discord: [CosmicLedger Community](https://discord.gg/cosmicledger)
- 🐛 Issues: [GitHub Issues](https://github.com/CosmicLedger/stellar-chain-explorer/issues)

---

Made with ❤️ by the CosmicLedger Team
