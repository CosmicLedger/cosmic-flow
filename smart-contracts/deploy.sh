#!/bin/bash

# Soroban Smart Contracts Deployment Script
# This script builds, optimizes, and deploys all contracts to the specified network

set -e

# Configuration
NETWORK=${1:-testnet}
IDENTITY=${2:-deployer}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Soroban Contract Deployment${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Network: $NETWORK"
echo "Identity: $IDENTITY"
echo ""

# Check if soroban CLI is installed
if ! command -v soroban &> /dev/null; then
    echo -e "${RED}Error: soroban CLI not found${NC}"
    echo "Install with: cargo install --locked soroban-cli"
    exit 1
fi

# Check if identity exists
if ! soroban config identity ls | grep -q "$IDENTITY"; then
    echo -e "${YELLOW}Identity '$IDENTITY' not found. Creating...${NC}"
    soroban config identity generate "$IDENTITY"
    
    if [ "$NETWORK" != "mainnet" ]; then
        echo -e "${YELLOW}Funding identity on $NETWORK...${NC}"
        soroban config identity fund "$IDENTITY" --network "$NETWORK"
    fi
fi

# Build all contracts
echo -e "${GREEN}Building contracts...${NC}"
cargo build --release --target wasm32-unknown-unknown

# Create deployment directory
DEPLOY_DIR="deployments/$NETWORK"
mkdir -p "$DEPLOY_DIR"

# Array of contracts to deploy
CONTRACTS=("token" "nft" "defi-pool" "governance")

# Deploy each contract
for CONTRACT in "${CONTRACTS[@]}"; do
    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}Deploying $CONTRACT contract...${NC}"
    echo -e "${GREEN}========================================${NC}"
    
    WASM_FILE="target/wasm32-unknown-unknown/release/${CONTRACT}.wasm"
    
    if [ ! -f "$WASM_FILE" ]; then
        echo -e "${RED}Error: WASM file not found: $WASM_FILE${NC}"
        continue
    fi
    
    # Optimize contract
    echo -e "${YELLOW}Optimizing contract...${NC}"
    soroban contract optimize --wasm "$WASM_FILE"
    OPTIMIZED_WASM="${WASM_FILE%.wasm}.optimized.wasm"
    
    # Get file sizes
    ORIGINAL_SIZE=$(stat -c%s "$WASM_FILE")
    OPTIMIZED_SIZE=$(stat -c%s "$OPTIMIZED_WASM")
    REDUCTION=$((100 - (OPTIMIZED_SIZE * 100 / ORIGINAL_SIZE)))
    
    echo "Original size: $ORIGINAL_SIZE bytes"
    echo "Optimized size: $OPTIMIZED_SIZE bytes"
    echo "Size reduction: $REDUCTION%"
    
    # Deploy contract
    echo -e "${YELLOW}Deploying to $NETWORK...${NC}"
    CONTRACT_ID=$(soroban contract deploy \
        --wasm "$OPTIMIZED_WASM" \
        --network "$NETWORK" \
        --source "$IDENTITY" 2>&1 | tee /dev/tty | tail -1)
    
    if [ -z "$CONTRACT_ID" ]; then
        echo -e "${RED}Error: Failed to deploy $CONTRACT${NC}"
        continue
    fi
    
    echo -e "${GREEN}✓ Deployed successfully!${NC}"
    echo "Contract ID: $CONTRACT_ID"
    
    # Save deployment info
    DEPLOY_FILE="$DEPLOY_DIR/${CONTRACT}.json"
    cat > "$DEPLOY_FILE" << EOF
{
  "contract": "$CONTRACT",
  "network": "$NETWORK",
  "contractId": "$CONTRACT_ID",
  "deployedAt": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "deployedBy": "$IDENTITY",
  "wasmSize": $OPTIMIZED_SIZE
}
EOF
    
    echo "Deployment info saved to: $DEPLOY_FILE"
done

# Summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Deployment Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Network: $NETWORK"
echo "Contracts deployed:"
echo ""

for CONTRACT in "${CONTRACTS[@]}"; do
    DEPLOY_FILE="$DEPLOY_DIR/${CONTRACT}.json"
    if [ -f "$DEPLOY_FILE" ]; then
        CONTRACT_ID=$(jq -r '.contractId' "$DEPLOY_FILE")
        echo "  • $CONTRACT: $CONTRACT_ID"
    fi
done

echo ""
echo -e "${GREEN}All deployments completed!${NC}"
echo ""
echo "Deployment info stored in: $DEPLOY_DIR/"
echo ""
echo "Next steps:"
echo "1. Initialize contracts with proper parameters"
echo "2. Update frontend configuration with contract IDs"
echo "3. Update backend API with contract IDs"
echo "4. Test contract interactions"
echo ""
