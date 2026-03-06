# Plan: Per-Table Config Hashes + Agent-Side Schema Change Detection

**Depends on**: [MIXED_PAYLOAD_PLAN.md](MIXED_PAYLOAD_PLAN.md) (mixed payloads + per-table consolidation)

## Context

Two related problems:

1. **Agent-hub mismatch**: Agent and hub must have identical table configs for correct SQL generation, but there's no validation. A mismatch silently produces wrong SQL.
2. **Agent-side config change**: If the config changes between block creations, the block chain becomes invalid — deltas computed under the old schema are incompatible.

This plan addresses both with per-table field hashes on the Patch wire format and per-table STATE comparison for change detection. Per-table consolidation from the mixed payload plan ensures that a config change on one table doesn't force a full state snapshot for all tables.

## Part A: Per-Table Field Hashes on Patch

### What Gets Hashed (Per Table)

Only the fields that affect SQL generation:
- Field names, sql_types, primary_key flags, null sentinels (order-independent — fields sorted by name)

**Not hashed**: `source`, `header`, `compression`, `truncate`, `work_dir`, `injected_fields`

### Changes

**A1. Add `field_hashes` to Patch proto — `proto/patch.proto`**

```protobuf
message Patch {
  // ... existing fields from MIXED_PAYLOAD_PLAN ...
  // Per-table config hash for agent-hub validation (key = table name).
  map<string, string> field_hashes = 7;
}
```

**A2. Add `TableConfig::field_hash()` — `src/config.rs`**

Compute SHA-1 over a single table's fields:
- Sort fields alphabetically by name
- For each field, serialize: `name`, `sql_type`, `primary_key`, `null`
- Return hex hash via `utils::compute_hash()`

**A3. Set `field_hashes` in `Patch::create()` — `src/patch.rs`**

After building the patch, populate `field_hashes` from config for every table present in either `deltas` or `states`. Config is already available in `Patch::create()`.

**A4. Validate on hub side — `src/sql.rs`**

In `patch_to_sql()`, before generating any SQL:
- For each table in the payload, look up `patch.field_hashes[table_name]`
- If present, compute the hub's hash for that table via `TableConfig::field_hash()` and compare
- On mismatch, return an error naming the table and both hashes
- If absent (old agent), skip validation — backwards compatible

## Part B: Agent-Side Schema Change Detection

### Mechanism

Use the existing STATE file for change detection. The STATE proto's `Table` message already stores `repeated string fields` — the column names in PK-first order. When loading previous state in `Block::create()`, compare each table's stored fields against the current config. Tables that don't match are dropped from the previous state, causing their deltas to be all-inserts.

Combined with per-table consolidation from the mixed payload plan, this gives full per-table granularity: when `Patch::create()` consolidates across the config change boundary, the changed table's merge fails (column mismatch) and falls back to full state for that table only. Unchanged tables consolidate normally and keep their incremental deltas.

### Changes

**B1. Compare STATE fields against config in `Block::create()` — `src/block.rs`**

After loading previous state, before computing deltas:

```rust
if let Some(ref mut previous) = previous_state {
    for (name, table_config) in &config.tables {
        if let Some(table) = previous.tables.get(name) {
            let expected_fields = table_config.ordered_field_names();
            if table.fields != expected_fields {
                log::warn!(
                    "Table '{}': field layout changed, discarding previous state",
                    name
                );
                previous.tables.remove(name);
            }
        }
    }
}
```

**B2. Add `TableConfig::ordered_field_names()` — `src/config.rs`**

Returns field names in PK-first order (matching how `Table.fields` is stored in STATE):
- Primary key fields first (in declaration order)
- Subsidiary fields second (in declaration order)

This mirrors the ordering that `Table::load()` already uses when building the in-memory table.

### What about sql_type or null sentinel changes (same field names)?

These don't affect delta computation — values are stored as strings regardless of type. Block creation and delta computation work correctly. The type mismatch only matters during SQL generation, which Part A handles: the hub validates `field_hashes` before generating SQL and catches the mismatch.

### Behavior by scenario

| Scenario | What happens |
|---|---|
| **Table unchanged** | STATE fields match config → normal incremental delta |
| **Field added/removed** | Field count differs → table dropped from previous state → all-inserts delta. Cross-boundary merge fails → that table uses full state in patch. Other tables unaffected. |
| **Field renamed** | Field names differ → same as above |
| **Primary key changed** | PK-first ordering differs → same as above |
| **Field order changed** | Subsidiary ordering differs → same as above |
| **sql_type changed** | Field names unchanged → STATE compatible → normal delta. Hub catches mismatch via Part A |
| **null sentinel changed** | Same as sql_type — transparent to delta computation, caught by Part A |
| **New table in config** | No entry in previous STATE → all-inserts delta (already works) |
| **Table removed from config** | Not in `State::compute()` output → no delta produced (already works) |

## Files Modified

| File | Change |
|---|---|
| `proto/patch.proto` | Add `map<string, string> field_hashes = 7;` |
| `src/config.rs` | Add `TableConfig::field_hash()` and `TableConfig::ordered_field_names()` |
| `src/block.rs` | Compare STATE fields against config, drop mismatched tables |
| `src/patch.rs` | Populate `field_hashes` from config in `Patch::create()` |
| `src/sql.rs` | Validate `field_hashes` in `patch_to_sql()` |

## Tests

- Unit test in `config.rs`: verify `field_hash()` is deterministic and changes when config changes
- Unit test in `config.rs`: verify `ordered_field_names()` returns PK-first ordering
- Unit test in `sql.rs`: verify `patch_to_sql()` rejects patches with mismatched field hash
- Test in `block.rs`: verify mismatched tables are dropped from previous state (delta becomes all-inserts)
- Acceptance test: verify config change produces mixed patch (state for changed table, deltas for unchanged)

## Verification

```
cargo build && cargo test
cargo fmt
cargo clippy
```
