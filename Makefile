.PHONY: help setup start stop clean build run test migrate migrate-redo db-reset

help: ## Display this help message
	@echo "Transaction Processor - Available Commands"
	@echo "=========================================="
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Complete setup (PostgreSQL + migrations + build)
	@echo "Running setup..."
	@./setup.sh

start: ## Start PostgreSQL database
	@echo "Starting PostgreSQL..."
	@cd infra && docker compose up -d
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 5
	@echo "✓ PostgreSQL is running"

stop: ## Stop PostgreSQL database
	@echo "Stopping PostgreSQL..."
	@cd infra && docker compose down
	@echo "✓ PostgreSQL stopped"

clean: ## Stop and remove all data
	@echo "Cleaning up..."
	@cd infra && docker compose down -v
	@echo "✓ All data removed"

build: ## Build the application
	@echo "Building application..."
	@cd app && cargo build
	@echo "✓ Build complete"

build-release: ## Build the application in release mode
	@echo "Building application (release mode)..."
	@cd app && cargo build --release
	@echo "✓ Release build complete"

run: ## Run the application
	@cd app && cargo run

test: ## Run tests
	@cd app && cargo test

fmt: ## Format code
	@cd app && cargo fmt

lint: ## Run clippy linter
	@cd app && cargo clippy

migrate: ## Run database migrations
	@cd app && diesel migration run

migrate-redo: ## Revert and rerun the last migration
	@cd app && diesel migration redo

db-reset: ## Reset database (down + up + migrate)
	@echo "Resetting database..."
	@cd infra && docker compose down -v
	@cd infra && docker compose up -d
	@sleep 5
	@cd app && diesel migration run
	@echo "✓ Database reset complete"

logs: ## Show PostgreSQL logs
	@cd infra && docker compose logs -f postgres

status: ## Show PostgreSQL status
	@cd infra && docker compose ps
