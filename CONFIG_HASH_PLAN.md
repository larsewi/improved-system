# Plan: Per-Table Config Hashes + Agent-Side Schema Change Detection

## Context

Two related problems:

1. **Agent-hub mismatch**: Agent and hub must have identical table configs for correct SQL generation, but there's no validation. A mismatch silently produces wrong SQL.
2. **Agent-side config change**: If the config changes between block creations, the block chain becomes invalid — deltas computed under the old schema are incompatible.

This plan addresses both with per-table field hashes in the wire format and a persisted schema hash for change detection.

## Part A: Per-Table Field Hashes in Wire Format

### What Gets Hashed (Per Table)

Only the fields that affect SQL generation:
- Field names, sql_types, primary_key flags, null sentinels (order-independent — fields sorted by name)

**Not hashed**: `source`, `header`, `compression`, `truncate`, `work_dir`, `injected_fields`

### Changes

**A1. Add `field_hash` to protos**
- `proto/delta.proto`: Add `string field_hash = 6;` to `Delta`
- `proto/table.proto`: Add `string field_hash = 3;` to `Table`

**A2. Add `TableConfig::field_hash()` — `src/config.rs`**

Compute SHA-1 over a single table's fields:
- Sort fields alphabetically by name
- For each field, serialize: `name`, `sql_type`, `primary_key`, `null`
- Return hex hash via `utils::compute_hash()`

**A3. Set `field_hash` when building Delta and Table protos**
- In the `From<delta::Delta>` impl for proto Delta (likely `src/delta.rs`): set `field_hash` — needs access to config, so this may need to be wired through
- In the `From<state::State>` impl for proto State/Table: same

**A4. Validate on hub side — `src/sql.rs`**

In `delta_to_sql()` and `state_to_sql()`, after `TableSchema::resolve()`:
- If `field_hash` is non-empty, compute the hub's hash for that table and compare
- On mismatch, return an error naming the table and both hashes
- If empty (old agent), skip validation — backwards compatible

## Part B: Agent-Side Schema Change Detection

### Mechanism

Persist a combined schema hash to a `SCHEMA` file in the work directory (alongside HEAD, STATE, REPORTED). At block creation time, detect changes and reset the chain.

**B1. Add `Config::schema_hash()` — `src/config.rs`**

Compute a single SHA-1 over all tables:
- Sort table names alphabetically
- For each table, include its `field_hash()` output
- Return hex hash via `utils::compute_hash()`

**B2. Add `src/schema.rs` module** (following `head.rs` / `reported.rs` pattern)

- `load(work_dir)` → `Result<Option<String>>`
- `store(work_dir, hash)` → `Result<()>`

Persists the schema hash to a `SCHEMA` file using `storage::store/load`.

**B3. Check and reset in `Block::create()` — `src/block.rs`**

At the top of `Block::create()`, before computing state:
1. Compute `config.schema_hash()`
2. Load persisted schema hash via `schema::load()`
3. If persisted hash exists AND differs from current:
   - Log a warning about config change
   - Reset HEAD to genesis (`head::store(work_dir, GENESIS_HASH)`)
   - Delete STATE file
4. Save current schema hash via `schema::store()`
5. Continue with normal block creation — HEAD = genesis means fresh chain, and truncation at the end cleans up orphaned blocks

## Files Modified

| File | Change |
|---|---|
| `proto/delta.proto` | Add `field_hash` to `Delta` |
| `proto/table.proto` | Add `field_hash` to `Table` |
| `src/config.rs` | Add `TableConfig::field_hash()` and `Config::schema_hash()` |
| `src/schema.rs` | New module: `load()` / `store()` for persisted schema hash |
| `src/block.rs` | Schema change detection at top of `Block::create()` |
| `src/delta.rs` | Set `field_hash` when converting to proto Delta |
| `src/state.rs` | Set `field_hash` when converting to proto Table |
| `src/sql.rs` | Validate `field_hash` in `delta_to_sql()` and `state_to_sql()` |
| `src/main.rs` or `src/lib.rs` | Register `schema` module |

## Tests

- Unit test in `config.rs`: verify `field_hash()` is deterministic and changes when config changes
- Unit test in `sql.rs`: verify SQL conversion rejects tables with mismatched field hash
- Test in `block.rs`: verify chain reset when schema changes

## Verification

```
cargo build && cargo test
cargo fmt
cargo clippy
```
