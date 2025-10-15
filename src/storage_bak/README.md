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

## Redb

Redb support create multiple tables. But before read/write a table, we needs to
open the table. When opening the table, there's an mutex lock needs to be
acquired. Which means opening table in the read path may block any other reads.

So we put all the data into one table with different key prefix.

| information       | key prefix | note             |
| ----------------- | ---------- | ---------------- |
| Schema            | 0x00       |                  |
| Label Token       | 0x01       |                  |
| RelType Token     | 0x02       |                  |
| PropertyKey Token | 0x03       |                  |
| NodeId            | 0x04       | single key value |
| RelationshipId    | 0x05       | single key value |
| Node              | 0x06       |                  |

# Capacity

- RelatioshipId: 5B
- NodeId: 8B
- PropertyKey/Label/RelType: 2B
