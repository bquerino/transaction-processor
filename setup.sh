#!/bin/bash

# Transaction Processor Setup Script
# This script helps set up and run the ledger application

set -e

echo "Transaction Processor - Setup Script"
echo "====================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored messages
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is not installed. Please install from https://rustup.rs/"
    exit 1
fi
print_success "Rust/Cargo found"

if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed. Please install Docker."
    exit 1
fi
print_success "Docker found"

if ! command -v diesel &> /dev/null; then
    print_warning "Diesel CLI not found. Installing..."
    cargo install diesel_cli --no-default-features --features postgres
    print_success "Diesel CLI installed"
else
    print_success "Diesel CLI found"
fi

# Start PostgreSQL
echo ""
echo "Starting PostgreSQL..."
cd infra
docker compose up -d
print_success "PostgreSQL started"

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
sleep 5

# Check PostgreSQL health
if docker compose exec -T postgres pg_isready -U ledger_user -d ledger_db > /dev/null 2>&1; then
    print_success "PostgreSQL is ready"
else
    print_error "PostgreSQL is not ready. Please check docker compose logs."
    exit 1
fi

# Set up .env file
cd ../app
if [ ! -f .env ]; then
    echo "Creating .env file..."
    cp .env.example .env
    print_success ".env file created"
else
    print_warning ".env file already exists, skipping..."
fi

# Run migrations
echo ""
echo "Running database migrations..."
diesel migration run
print_success "Migrations completed"

# Build the application
echo ""
echo "Building the application..."
cargo build --release
print_success "Application built successfully"

echo ""
echo "====================================="
print_success "Setup completed successfully!"
echo ""
echo "To run the application:"
echo "  cd app"
echo "  cargo run"
echo ""
echo "To stop PostgreSQL:"
echo "  cd infra"
echo "  docker compose down"
echo ""
