#!/bin/bash

# CarbonChain Deployment Script
# This script deploys all three Solana programs to devnet

set -e

echo "CarbonChain Deployment Script"
echo "================================"

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "ERROR: Solana CLI not found. Please install it first."
    exit 1
fi

# Check if Anchor CLI is installed
if ! command -v anchor &> /dev/null; then
    echo "ERROR: Anchor CLI not found. Please install it first."
    exit 1
fi

# Set Solana config to devnet
echo "Setting Solana config to devnet..."
solana config set --url devnet

# Check wallet balance
echo "Checking wallet balance..."
BALANCE=$(solana balance --lamports)
if [ "$BALANCE" -lt 1000000000 ]; then
    echo "WARNING: Low balance detected. Requesting airdrop..."
    solana airdrop 2
    sleep 5
fi

# Build all programs
echo "Building Anchor programs..."
anchor build

# Deploy programs
echo "Deploying programs to devnet..."

echo "Deploying Carbon Marketplace..."
anchor deploy --program-name carbon_marketplace --provider.cluster devnet

echo "Deploying Carbon Verification..."
anchor deploy --program-name carbon_verification --provider.cluster devnet

echo "Deploying Carbon Registry..."
anchor deploy --program-name carbon_registry --provider.cluster devnet

# Get program IDs
echo "Program IDs:"
echo "Carbon Marketplace: $(solana address -k target/deploy/carbon_marketplace-keypair.json)"
echo "Carbon Verification: $(solana address -k target/deploy/carbon_verification-keypair.json)"
echo "Carbon Registry: $(solana address -k target/deploy/carbon_registry-keypair.json)"

echo ""
echo "SUCCESS: Deployment completed successfully!"
echo "Programs are now live on Solana devnet"
echo ""
echo "Next steps:"
echo "1. Update program IDs in your frontend application"
echo "2. Initialize the marketplace with admin functions"
echo "3. Set up verifiers and registry"
echo "4. Start trading carbon credits"
echo ""
echo "Building a sustainable future on Solana!"