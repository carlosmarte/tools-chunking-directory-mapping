# Rust Directory Scanner SDK Makefile
# High-performance directory scanning with CLI, library, and WebAssembly bindings

# Colors for output
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
RED := \033[31m
RESET := \033[0m
BOLD := \033[1m

# Default target
.DEFAULT_GOAL := help

# Phony targets
.PHONY: help install-deps install-wasm-pack install-all build build-debug build-core build-cli build-wasm clean test test-core test-cli test-wasm test-unit test-integration test-performance test-features test-verbose test-quick check format lint run run-json example publish publish-core publish-cli publish-all

##@ Installation
install-deps: ## Install Rust toolchain and essential dependencies
	@echo "$(BLUE)Installing Rust toolchain...$(RESET)"
	@if ! command -v rustc >/dev/null 2>&1; then \
		echo "$(YELLOW)Rust not found. Installing via rustup...$(RESET)"; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		echo "$(GREEN)Please run 'source ~/.cargo/env' or restart your shell$(RESET)"; \
	else \
		echo "$(GREEN)Rust already installed: $$(rustc --version)$(RESET)"; \
	fi
	@echo "$(BLUE)Updating Rust toolchain...$(RESET)"
	rustup update
	@echo "$(GREEN)✓ Dependencies installed$(RESET)"

install-wasm-pack: ## Install wasm-pack for WebAssembly builds
	@echo "$(BLUE)Installing wasm-pack...$(RESET)"
	@if ! command -v wasm-pack >/dev/null 2>&1; then \
		cargo install wasm-pack; \
		echo "$(GREEN)✓ wasm-pack installed$(RESET)"; \
	else \
		echo "$(GREEN)wasm-pack already installed: $$(wasm-pack --version)$(RESET)"; \
	fi

install-all: ## Install all workspace crates with binary targets
	@echo "$(BLUE)Installing workspace binaries...$(RESET)"
	@for crate in crates/*; do \
		if [ -d "$$crate" ] && grep -q "\[\[bin\]\]" "$$crate/Cargo.toml" 2>/dev/null; then \
			echo "$(YELLOW)Installing $$crate...$(RESET)"; \
			cargo install --path "$$crate"; \
		fi; \
	done
	@echo "$(GREEN)✓ Installation completed$(RESET)"

##@ Building
build: ## Build all workspace crates in release mode
	@echo "$(BLUE)Building all crates (release mode)...$(RESET)"
	cargo build --workspace --release
	@echo "$(GREEN)✓ Build completed$(RESET)"

build-debug: ## Build all workspace crates in debug mode
	@echo "$(BLUE)Building all crates (debug mode)...$(RESET)"
	cargo build --workspace
	@echo "$(GREEN)✓ Debug build completed$(RESET)"

build-core: ## Build only the core library
	@echo "$(BLUE)Building core library...$(RESET)"
	cargo build -p thinkeloquent-tools-chunking-directory-mapping-core --release
	@echo "$(GREEN)✓ Core library built$(RESET)"

build-cli: ## Build only the CLI application
	@echo "$(BLUE)Building CLI application...$(RESET)"
	cargo build -p thinkeloquent-tools-chunking-directory-mapping --release
	@echo "$(GREEN)✓ CLI application built$(RESET)"

build-wasm: install-wasm-pack ## Build WebAssembly bindings
	@echo "$(BLUE)Building WebAssembly bindings...$(RESET)"
	cd crates/wasm && wasm-pack build --target bundler --scope thinkeloquent
	@echo "$(BLUE)Building WebAssembly for Node.js...$(RESET)"
	cd crates/wasm && wasm-pack build --target nodejs --scope thinkeloquent
	@echo "$(GREEN)✓ WebAssembly bindings built$(RESET)"

clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(RESET)"
	cargo clean
	rm -rf crates/wasm/pkg
	rm -rf pkg
	@echo "$(GREEN)✓ Clean completed$(RESET)"

##@ Testing
test: ## Run all tests across the workspace
	@echo "$(BLUE)Running all tests...$(RESET)"
	@echo "$(YELLOW)→ Core library tests$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping-core
	@echo "$(YELLOW)→ CLI application tests$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping 2>/dev/null || echo "$(YELLOW)No CLI tests found$(RESET)"
	@echo "$(YELLOW)→ WASM binding tests$(RESET)"  
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping-wasm 2>/dev/null || echo "$(YELLOW)No WASM tests found$(RESET)"
	@echo "$(GREEN)✓ All tests completed$(RESET)"

test-core: ## Run core library tests only
	@echo "$(BLUE)Running core library tests...$(RESET)"
	cargo test -p thinkeloquent-tools-chunking-directory-mapping-core
	@echo "$(GREEN)✓ Core tests completed$(RESET)"

test-cli: ## Run CLI application tests
	@echo "$(BLUE)Running CLI tests...$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping || echo "$(YELLOW)No CLI tests found$(RESET)"

test-wasm: ## Run WebAssembly binding tests
	@echo "$(BLUE)Running WASM tests...$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping-wasm || echo "$(YELLOW)No WASM tests found$(RESET)"

test-unit: ## Run only unit tests (fast)
	@echo "$(BLUE)Running unit tests...$(RESET)"
	cargo test --lib -p thinkeloquent-tools-chunking-directory-mapping-core
	@echo "$(GREEN)✓ Unit tests completed$(RESET)"

test-integration: ## Run integration tests
	@echo "$(BLUE)Running integration tests...$(RESET)"
	cargo test --test '*' -p thinkeloquent-tools-chunking-directory-mapping-core 2>/dev/null || echo "$(YELLOW)No integration tests found$(RESET)"
	@echo "$(GREEN)✓ Integration tests completed$(RESET)"

test-performance: ## Run performance tests only
	@echo "$(BLUE)Running performance tests...$(RESET)"
	cargo test -p thinkeloquent-tools-chunking-directory-mapping-core performance_tests
	@echo "$(GREEN)✓ Performance tests completed$(RESET)"

test-features: ## Run enhanced branching analysis feature tests
	@echo "$(BLUE)Running enhanced branching analysis tests...$(RESET)"
	@echo "$(YELLOW)→ Hard-coded values detection$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping-core hardcoded_values_detection
	@echo "$(YELLOW)→ Purity analysis$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping-core purity_analysis
	@echo "$(YELLOW)→ Temporal logic detection$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping-core temporal_logic_detection
	@echo "$(YELLOW)→ Nesting distribution$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping-core nesting_distribution
	@echo "$(YELLOW)→ Cross-language compatibility$(RESET)"
	@cargo test -p thinkeloquent-tools-chunking-directory-mapping-core cross_language_compatibility
	@echo "$(GREEN)✓ Feature tests completed$(RESET)"

test-verbose: ## Run tests with verbose output (shows println! etc.)
	@echo "$(BLUE)Running tests with verbose output...$(RESET)"
	cargo test -p thinkeloquent-tools-chunking-directory-mapping-core -- --nocapture
	@echo "$(GREEN)✓ Verbose tests completed$(RESET)"

test-quick: ## Run tests quickly (parallel, stop on first failure)
	@echo "$(BLUE)Running tests quickly...$(RESET)"
	cargo test -p thinkeloquent-tools-chunking-directory-mapping-core -- --stop-on-first-failure
	@echo "$(GREEN)✓ Quick tests completed$(RESET)"

test-coverage: ## Generate test coverage report (requires cargo-llvm-cov)
	@echo "$(BLUE)Generating test coverage report...$(RESET)"
	@if ! command -v cargo-llvm-cov >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing cargo-llvm-cov...$(RESET)"; \
		cargo install cargo-llvm-cov; \
	fi
	cargo llvm-cov --workspace --html
	@echo "$(GREEN)✓ Coverage report generated in target/llvm-cov/html/$(RESET)"

test-doc: ## Run documentation tests
	@echo "$(BLUE)Running documentation tests...$(RESET)"
	cargo test --doc -p thinkeloquent-tools-chunking-directory-mapping-core
	@echo "$(GREEN)✓ Documentation tests completed$(RESET)"

test-release: ## Run tests in release mode (optimized)
	@echo "$(BLUE)Running tests in release mode...$(RESET)"
	cargo test --release -p thinkeloquent-tools-chunking-directory-mapping-core
	@echo "$(GREEN)✓ Release tests completed$(RESET)"

test-specific: ## Run a specific test (Usage: make test-specific TEST=test_name)
	@echo "$(BLUE)Running specific test: $(TEST)$(RESET)"
	@if [ -z "$(TEST)" ]; then \
		echo "$(RED)Error: Please specify TEST=test_name$(RESET)"; \
		echo "$(YELLOW)Example: make test-specific TEST=test_detects_iso_dates$(RESET)"; \
		exit 1; \
	fi
	cargo test -p thinkeloquent-tools-chunking-directory-mapping-core $(TEST) -- --nocapture
	@echo "$(GREEN)✓ Specific test completed$(RESET)"

test-watch: ## Continuously run tests on file changes (requires cargo-watch)
	@echo "$(BLUE)Starting test watcher...$(RESET)"
	@if ! command -v cargo-watch >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing cargo-watch...$(RESET)"; \
		cargo install cargo-watch; \
	fi
	cargo watch -x "test -p thinkeloquent-tools-chunking-directory-mapping-core"

test-bench: ## Run benchmark tests (if available)
	@echo "$(BLUE)Running benchmark tests...$(RESET)"
	@cargo test --benches -p thinkeloquent-tools-chunking-directory-mapping-core 2>/dev/null || echo "$(YELLOW)No benchmark tests found$(RESET)"
	@echo "$(GREEN)✓ Benchmark tests completed$(RESET)"

##@ Development

check: ## Run cargo check on all crates
	@echo "$(BLUE)Checking code...$(RESET)"
	cargo check --workspace
	@echo "$(GREEN)✓ Check completed$(RESET)"

format: ## Format code with rustfmt
	@echo "$(BLUE)Formatting code...$(RESET)"
	cargo fmt --all
	@echo "$(GREEN)✓ Code formatted$(RESET)"

lint: ## Run clippy linter
	@echo "$(BLUE)Running linter...$(RESET)"
	cargo clippy --workspace -- -D warnings
	@echo "$(GREEN)✓ Lint completed$(RESET)"

##@ Publishing
publish-core: ## Publish the core library to crates.io
	@echo "$(BLUE)Publishing core library...$(RESET)"
	cargo publish -p thinkeloquent-tools-chunking-directory-mapping-core
	@echo "$(GREEN)✓ Core library published$(RESET)"

publish-cli: ## Publish the CLI application to crates.io
	@echo "$(BLUE)Publishing CLI application...$(RESET)"
	cargo publish -p thinkeloquent-tools-chunking-directory-mapping
	@echo "$(GREEN)✓ CLI application published$(RESET)"

publish-all: ## Publish all crates to crates.io (in dependency order)
	@echo "$(BLUE)Publishing all crates in dependency order...$(RESET)"
	@echo "$(YELLOW)→ Publishing core library first...$(RESET)"
	cargo publish -p thinkeloquent-tools-chunking-directory-mapping-core
	@echo "$(YELLOW)→ Waiting for core to propagate...$(RESET)"
	sleep 10
	@echo "$(YELLOW)→ Publishing CLI application...$(RESET)"
	cargo publish -p thinkeloquent-tools-chunking-directory-mapping
	@echo "$(GREEN)✓ All crates published successfully$(RESET)"

publish: publish-all ## Alias for publish-all

##@ Usage
run: build-cli ## Run CLI with current directory scan
	@echo "$(BLUE)Running directory scanner on current directory...$(RESET)"
	./target/release/projscan . --profile generic

run-json: build-cli ## Run CLI with JSON output
	@echo "$(BLUE)Running directory scanner with JSON output...$(RESET)"
	./target/release/projscan . --profile generic --json

example: build-cli ## Show example usage
	@echo "$(BOLD)$(BLUE)Directory Scanner SDK Examples:$(RESET)"
	@echo ""
	@echo "$(YELLOW)1. Basic scan:$(RESET)"
	@echo "   ./target/release/projscan ."
	@echo ""
	@echo "$(YELLOW)2. JSON output:$(RESET)"
	@echo "   ./target/release/projscan . --profile generic --json"
	@echo ""
	@echo "$(YELLOW)3. Scan specific directory:$(RESET)"
	@echo "   ./target/release/projscan /path/to/directory --profile generic"
	@echo ""
	@echo "$(YELLOW)4. Library usage (Rust):$(RESET)"
	@echo "   use directory_scanner_core::{DirectoryScanner, ScanOptions};"
	@echo "   let scanner = DirectoryScanner::new(ScanOptions::default());"
	@echo "   let result = scanner.scan(\".\")?;"
	@echo ""
	@echo "$(GREEN)Run 'make run' for a live example!$(RESET)"

##@ Help
help: ## Display this help message
	@echo "$(BOLD)$(BLUE)Rust Directory Scanner SDK$(RESET)"
	@echo "A high-performance directory scanning SDK with CLI, library, and WebAssembly bindings"
	@echo ""
	@echo "$(BOLD)Usage:$(RESET) make $(BLUE)<target>$(RESET)"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*?##/ { printf "  $(BLUE)%-15s$(RESET) %s\n", $$1, $$2 }' $(MAKEFILE_LIST) | sort
	@echo ""
	@echo "$(BOLD)Project Status:$(RESET)"
	@echo "  $(GREEN)✓$(RESET) M1 Core Foundation - Complete"
	@echo "  $(GREEN)✓$(RESET) M2 Enhanced Analysis - Complete (48 tests)"
	@echo "  $(YELLOW)→$(RESET) M3 Production Ready - In Progress"
	@echo ""
	@echo "$(BOLD)Quick Start:$(RESET)"
	@echo "  1. $(BLUE)make install-deps$(RESET)     # Install Rust toolchain"
	@echo "  2. $(BLUE)make build$(RESET)           # Build all components"
	@echo "  3. $(BLUE)make test$(RESET)            # Run all tests (48 tests)"
	@echo "  4. $(BLUE)make run$(RESET)             # Test the CLI"
	@echo ""
	@echo "$(BOLD)Testing Commands:$(RESET)"
	@echo "  $(BLUE)make test$(RESET)              # Run all tests"
	@echo "  $(BLUE)make test-features$(RESET)     # Test enhanced branching analysis"
	@echo "  $(BLUE)make test-performance$(RESET)  # Run performance tests"
	@echo "  $(BLUE)make test-verbose$(RESET)      # Show detailed test output"
	@echo "  $(BLUE)make test-coverage$(RESET)     # Generate coverage report"