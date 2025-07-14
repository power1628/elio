# Overview

Storage model is based on a btree key-value storage: redb.

We map all the data into a single key-value store:

- Schema: including constraints
- Token: including label, property key, relationship types
- Node/Relationship data
- NodeId
- Index

Since redb already support transaction and it has it's own buffer pool it saves
us a lot of work. So in mojito, we no longer have our own buffer pool.

All transaction ACID guarantee and caching are supported by redb.

## API

GraphStore

- open tx
- read/write token
- read/write id
- read/write node
- read/write relationship
- read/write schema
- read/write index
- commit/abort tx

KvStore

- open tx
- read(key, value)
- write(key, value)
- commit/abort tx

## Model

How we map graph data into a key value store?

### Schema

### Token

### Node/Relationship
