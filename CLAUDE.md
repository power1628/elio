# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Mojito is a graph database system written in Rust that processes Cypher query language (Neo4j's query language). It implements a full compiler/execution pipeline for graph queries.

## Build Commands

```bash
make build              # Build all binaries
make build-release      # Release build
```

## Test Commands

```bash
make test               # Run all tests with full backtrace
make unit-test          # Run unit and doc tests
make logic-test         # Run logic tests (src/logictest/tests/*.slt)
make planner-test       # Run planner tests (src/plannertest/tests/*.yml)

# Rewrite test outputs (when expected outputs change)
make rewrite-logic-test
make rewrite-planner-test
```

## Lint and Format

```bash
make fmt                # Format code
make fmt-check          # Check formatting
make clippy             # Run clippy with auto-fix
make clippy-check       # Check clippy warnings
```

## Architecture

Query processing pipeline:

```
Cypher Query → Parser → Binder → Planner → Executor → Storage (RocksDB)
```

### Workspace Crates

| Crate | Purpose |
|-------|---------|
| `mojito_parser` | PEG-based Cypher parser, produces AST |
| `mojito_cypher` | Query binding and planning (binder, planner, plan nodes, expressions) |
| `mojito_exec` | Physical execution engine with async task management |
| `mojito_storage` | RocksDB-backed graph storage (nodes, relationships, properties) |
| `mojito_expr` | Expression evaluation, function implementations (uses proc macros) |
| `mojito_catalog` | Function registry and schema metadata |
| `mojito_common` | Shared types, data types, value representations |
| `mojito_core` | Top-level database engine, session handling |
| `cmd` | CLI binary (`mojito`) |

### Test Frameworks

- **Logic tests** (`src/logictest/tests/`): `.slt` files using sqllogictest format for end-to-end query testing
- **Planner tests** (`src/plannertest/tests/`): `.yml` files for testing query plan generation

## Toolchain

Uses Rust nightly (nightly-2025-11-01) for thiserror backtrace feature.
