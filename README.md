
[![CI](https://github.com/power1628/elio/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/power1628/elio/actions/workflows/ci.yml)
![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)

### elio: Embedded Graph Database

`elio` is an **embedded graph database** written in Rust. Your application opens a local database directory (default: `.db`) via library API or CLI, queries it using **openCypher** syntax, and stores data in a **schema-free** property graph model (no up-front schema/table definition required).

- 🧩 **Embedded**: no standalone server process; embed the database into your application, with data stored in a local directory (RocksDB backend).
- 🟣 **openCypher**: supports Neo4j-style pattern matching and queries such as `MATCH/CREATE/WHERE/RETURN`.
- 🧱 **Schema-free**: node/relationship properties are written on demand; labels and relationship types organize data but do not enforce a fixed schema.

### Quick Start

#### Build

```bash
make build
```

#### Run (CLI example)

The `cmd` crate provides the `elio` binary. The example program opens `--db-path` and executes a few Cypher statements:

```bash
cargo run -p cmd --bin elio -- --db-path .db
```

#### openCypher examples

```cypher
// Create nodes with properties (schema-free: no prior declaration needed)
CREATE (a:Person {name: 'Alice', age: 30})

// Create relationships
CREATE (a:Person {name: 'Alice'}), (b:Person {name: 'Bob'}), (a)-[:KNOWS]->(b)

// Query
MATCH (n) RETURN n

// Filter by label / properties
MATCH (n:Person {age: 30}) RETURN n

// Traversal (variable-length path)
MATCH (a)-[r:KNOWS*1..3]->(b) RETURN r
```


