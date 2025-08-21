#!/bin/bash
# validate_builds.sh - Deterministic Build Validation Script
# This script validates that builds are truly deterministic by running multiple builds
# and comparing their outputs for consistency.
set -euo pipefail
# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'
# No Color
# Configuration
BUILD_ITERATIONS=3
TEMP_DIR="./deterministic_validation_tmp"
BUILD_OUTPUT_DIR="target/release"
HASHES_FILE="validation_hashes.txt"
echo -e "${BLUE}🔬 Starting Deterministic Build Validation${NC}"
echo "======================================================"
# Clean up function
cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up temporary files...${NC}"
    rm -rf "$TEMP_DIR" 2>/dev/null || true
    rm -f "$HASHES_FILE" 2>/dev/null || true
}
# Set up cleanup on exit
trap cleanup EXIT
# Create temporary directory
mkdir -p "$TEMP_DIR"
