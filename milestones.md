# Project Milestones: Rust Directory Scanner SDK

## Milestone Overview

| Milestone | Duration | Goal | Deliverables |
|-----------|----------|------|--------------|
| **M1: Core Foundation** | Weeks 1-3 | Basic working SDK with core functionality | Library crate, basic CLI, minimal WASM |
| **M2: Advanced Features** | Weeks 4-6 | Full-featured implementation | Complete CLI, config system, all mappers |
| **M3: Production Ready** | Weeks 7-8 | Performance & polish | Optimized, cached, fully tested |
| **M4: Extended Capabilities** | Weeks 9-10 | Advanced features | Content sniffing, Git integration |

---

## M1: Core Foundation (Weeks 1-3)

**Goal:** Establish basic architecture and core functionality

### F1.1: Project Structure & Dependencies
**Tasks:**
- [ ] Create workspace Cargo.toml with crates/core, crates/cli, crates/wasm (2h)
- [ ] Set up core dependencies: walkdir, serde, thiserror (1h) 
- [ ] Configure CLI dependencies: clap 4.x (1h)
- [ ] Set up WASM dependencies: wasm-bindgen, serde-wasm-bindgen (2h)
- [ ] Create basic .gitignore and project README (1h)

### F1.2: Core Data Structures
**Tasks:**
- [ ] Implement `FileEntry` struct with serialization (2h)
- [ ] Implement `DirectoryNode` struct (1h)
- [ ] Implement `ScanResult` and `ScanStats` structs (1h)
- [ ] Create `ScanOptions` configuration struct (2h)
- [ ] Implement `ScanError` enum with thiserror (1h)

### F1.3: Basic Directory Scanning
**Tasks:**
- [ ] Implement recursive directory traversal with walkdir (4h)
- [ ] Add file metadata collection (size, mtime, permissions) (3h)
- [ ] Create basic ignore patterns (.git, node_modules, target) (2h)
- [ ] Implement path normalization and sanitization (2h)
- [ ] Add basic error handling and resource limits (2h)

### F1.4: Mapper Trait System
**Tasks:**
- [ ] Define `Mapper` trait interface (1h)
- [ ] Implement `GenericMapper` with basic patterns (3h)
- [ ] Create mapper registry system (2h)
- [ ] Implement tag application logic (2h)

### F1.5: Basic CLI
**Tasks:**
- [ ] Create main.rs with clap argument parsing (3h)
- [ ] Implement basic scan command with JSON output (2h)
- [ ] Add --profile flag with generic profile (2h)
- [ ] Add basic error handling and exit codes (1h)

### F1.6: Minimal WASM Bindings
**Tasks:**
- [ ] Set up wasm-pack build configuration (2h)
- [ ] Create basic scan function export (3h)
- [ ] Implement JS/TS type bindings (2h)
- [ ] Add basic error handling for WASM (1h)

**M1 Acceptance Criteria:**
- [ ] `projscan . --profile generic --json` outputs valid JSON
- [ ] Library can scan 1000+ files without crashing
- [ ] WASM module can be imported in Node.js
- [ ] Basic file classification works (docs, scripts, examples)

---

## M2: Advanced Features (Weeks 4-6)

**Goal:** Complete feature implementation per specification

### F2.1: Language-Specific Mappers
**Tasks:**
- [ ] Implement `NodeJsMapper` with all patterns from spec (4h)
- [ ] Implement `PythonMapper` with Python-specific rules (3h)  
- [ ] Implement `RustLangMapper` with Cargo conventions (3h)
- [ ] Create `PolyglotMapper` combining multiple profiles (2h)
- [ ] Add comprehensive mapper unit tests (4h)

### F2.2: Advanced CLI Features
**Tasks:**
- [ ] Add all remaining CLI flags (ignore, max-depth, etc.) (4h)
- [ ] Implement --stats flag with detailed timing (2h)
- [ ] Add --ndjson output format (2h)
- [ ] Implement --progress bar with indicatif (3h)
- [ ] Add --pretty and --out file options (2h)
- [ ] Implement --fail-on-limit behavior (1h)

### F2.3: Configuration System
**Tasks:**
- [ ] Design YAML/JSON config schema (2h)
- [ ] Implement config file parsing with serde (3h)
- [ ] Add custom mapper rules from config (4h)
- [ ] Implement config precedence (CLI > file > defaults) (2h)
- [ ] Add config validation and error reporting (2h)

### F2.4: Ignore File Processing
**Tasks:**
- [ ] Integrate `ignore` crate for .gitignore support (3h)
- [ ] Add support for .ignore files (1h)
- [ ] Implement custom glob patterns (2h)
- [ ] Add --no-gitignore flag support (1h)
- [ ] Test ignore behavior across different scenarios (2h)

### F2.5: Complete WASM API
**Tasks:**
- [ ] Implement all ScanOptions in WASM bindings (3h)
- [ ] Add custom mapper callback support (4h)
- [ ] Generate complete TypeScript definitions (2h)
- [ ] Add ESM/CJS dual exports (2h)
- [ ] Create npm package structure (2h)

### F2.6: Enhanced Error Handling
**Tasks:**
- [ ] Implement detailed error codes (E_IO, E_LIMIT, etc.) (2h)
- [ ] Add partial results on error (--allow-partial) (3h)
- [ ] Improve error messages with context (2h)
- [ ] Add error recovery strategies (2h)

**M2 Acceptance Criteria:**
- [ ] All built-in mappers correctly classify files
- [ ] Config file system works end-to-end
- [ ] CLI supports all documented flags
- [ ] WASM API matches TypeScript definitions
- [ ] Graceful error handling with meaningful messages

---

## M3: Production Ready (Weeks 7-8)

**Goal:** Performance optimization and production quality

### F3.1: Performance Optimization
**Tasks:**
- [ ] Profile scanning performance with criterion (3h)
- [ ] Implement parallel directory traversal with rayon (4h)
- [ ] Optimize memory usage for large directory trees (3h)
- [ ] Add benchmark suite against target metrics (2h)
- [ ] Optimize hot paths based on profiling (4h)

### F3.2: Caching System
**Tasks:**
- [ ] Design cache key strategy (mtime + path hash) (2h)
- [ ] Implement filesystem cache with sled or similar (4h)
- [ ] Add cache invalidation logic (2h)
- [ ] Add --no-cache and --cache-dir CLI options (2h)
- [ ] Test cache correctness across scenarios (3h)

### F3.3: Hashing Support
**Tasks:**
- [ ] Integrate blake3 crate (1h)
- [ ] Implement chunked file hashing (3h)
- [ ] Add hash caching and deduplication (2h)
- [ ] Add progress reporting for hash operations (2h)
- [ ] Add cancellation support (2h)

### F3.4: Comprehensive Testing
**Tasks:**
- [ ] Create fixture repositories for integration tests (4h)
- [ ] Implement golden file testing for JSON output (3h)
- [ ] Add CLI snapshot testing (2h)
- [ ] Create WASM integration tests with Vitest (3h)
- [ ] Add property-based testing for edge cases (3h)
- [ ] Achieve 90%+ code coverage (4h)

### F3.5: Documentation & Polish
**Tasks:**
- [ ] Write comprehensive API documentation (4h)
- [ ] Create usage examples and recipes (3h)
- [ ] Polish CLI help text and error messages (2h)
- [ ] Add logging with tracing crate (2h)
- [ ] Create performance benchmarking guide (2h)

**M3 Acceptance Criteria:**
- [ ] Scans 50k files in <5 seconds on typical hardware
- [ ] Cache reduces rescan time by 80%+
- [ ] Test coverage >90% with comprehensive edge cases
- [ ] Documentation covers all public APIs
- [ ] Performance meets specification targets

---

## M4: Extended Capabilities (Weeks 9-10)

**Goal:** Advanced features for future roadmap

### F4.1: Content Sniffing
**Tasks:**
- [ ] Implement shebang detection (2h)
- [ ] Add file magic number detection (3h)
- [ ] Create language detection heuristics (4h)
- [ ] Add pluggable content sniffer trait (2h)
- [ ] Integrate with existing mapper system (2h)

### F4.2: Git Integration
**Tasks:**
- [ ] Add git2 dependency for Git awareness (1h)
- [ ] Implement .git/info/exclude support (2h)
- [ ] Add git ls-files integration (3h)
- [ ] Create Git-aware ignore processing (2h)
- [ ] Add --git-aware CLI flag (1h)

### F4.3: Streaming API
**Tasks:**
- [ ] Design streaming trait interface (2h)
- [ ] Implement callback-based scanner (3h)
- [ ] Add backpressure handling (2h)
- [ ] Create NDJSON streaming output (2h)
- [ ] Add streaming WASM bindings (3h)

### F4.4: Advanced Progress & Observability
**Tasks:**
- [ ] Enhanced progress reporting with ETA (2h)
- [ ] Add structured logging with metrics (2h)
- [ ] Implement scan cancellation (2h)
- [ ] Add memory usage monitoring (2h)
- [ ] Create debug/trace modes (1h)

**M4 Acceptance Criteria:**
- [ ] Content sniffing improves classification accuracy
- [ ] Git integration works with complex repository structures
- [ ] Streaming API handles large repositories efficiently
- [ ] Advanced observability aids in debugging and optimization

---

## Implementation Guidelines

### Development Principles
1. **Test-Driven Development:** Write tests alongside implementation
2. **Incremental Delivery:** Each milestone produces working software
3. **Performance Awareness:** Profile early and continuously
4. **Documentation First:** API docs written with implementation

### Quality Gates
- **Code Review:** All changes reviewed for correctness and style
- **Automated Testing:** CI runs full test suite on each commit
- **Performance Regression:** Benchmark suite prevents performance degradation
- **API Stability:** Public APIs frozen after M2, changes require RFC

### Technical Debt Management
- **Refactoring Sprints:** 20% of time allocated for technical debt
- **Architecture Reviews:** Regular assessment of design decisions
- **Dependency Updates:** Monthly dependency updates and security patches

---

## Risk Assessment & Mitigation

### High Risk Items
| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| WASM complexity exceeds estimates | High | Medium | Start with minimal proof-of-concept, iterate |
| Performance targets not met | High | Low | Profile early, implement parallel processing |
| Mapper accuracy insufficient | Medium | Medium | Validate against real-world repos, community feedback |

### Critical Path Dependencies
1. **M1 → M2:** Core architecture must be solid before feature expansion
2. **M2 → M3:** Complete feature set required before optimization
3. **Rust ecosystem:** Dependency on stable versions of key crates

### Success Metrics

#### M1 Success Metrics
- [ ] Core library compiles and passes basic tests
- [ ] CLI produces valid JSON output for simple cases
- [ ] WASM module loads in Node.js without errors
- [ ] Basic file classification accuracy >80%

#### M2 Success Metrics
- [ ] All CLI flags implemented and tested
- [ ] Configuration system handles complex scenarios
- [ ] All built-in mappers achieve >90% accuracy on test repos
- [ ] WASM API feature-complete with TypeScript support

#### M3 Success Metrics
- [ ] Performance targets met: 50k files in <5s
- [ ] Test coverage >90% across all modules
- [ ] Memory usage stable for large directory trees
- [ ] Documentation complete and validated

#### M4 Success Metrics
- [ ] Content sniffing improves classification by >15%
- [ ] Git integration works with monorepos
- [ ] Streaming API handles 1M+ files efficiently
- [ ] Advanced observability enables debugging

---

## Resource Planning

### Development Team
- **Lead Developer:** Architecture, core library, performance optimization
- **CLI Developer:** Command-line interface, configuration system
- **WASM Developer:** JavaScript bindings, TypeScript definitions
- **QA Engineer:** Testing strategy, CI/CD, quality gates

### Time Allocation
- **Implementation:** 60% of effort
- **Testing:** 25% of effort  
- **Documentation:** 10% of effort
- **Project Management:** 5% of effort

### Infrastructure Requirements
- **CI/CD:** GitHub Actions for automated testing and releases
- **Benchmarking:** Dedicated performance testing environment
- **Documentation:** rustdoc + mdBook for comprehensive docs
- **Package Distribution:** crates.io, npm registry, GitHub releases