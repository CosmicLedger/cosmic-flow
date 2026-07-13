#!/bin/bash

# Soroban Smart Contracts Test Script
# This script runs all tests for the smart contracts

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Soroban Smart Contracts Testing${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Run all tests
echo -e "${YELLOW}Running unit tests...${NC}"
cargo test --all-features -- --nocapture

echo ""
echo -e "${YELLOW}Running clippy...${NC}"
cargo clippy --all-targets --all-features -- -D warnings

echo ""
echo -e "${YELLOW}Checking format...${NC}"
cargo fmt -- --check

echo ""
echo -e "${GREEN}All tests passed!${NC}"
