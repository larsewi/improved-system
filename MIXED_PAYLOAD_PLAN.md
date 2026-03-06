# Plan: Mixed Patch Payloads + Per-Table Consolidation

## Context

The Patch proto currently uses a `oneof payload` — either all-deltas or all-state for every table. This forces a full state snapshot for all tables even when only one table needs it. Removing this constraint enables per-table payload selection and is a prerequisite for [CONFIG_HASH_PLAN.md](CONFIG_HASH_PLAN.md).

## Proto Change — `proto/patch.proto`

Replace the `oneof payload` with two coexisting fields. A table appears in either `deltas` or `states`, not both.

```protobuf
message Patch {
  string head = 1;
  google.protobuf.Timestamp created = 2;
  repeated injected.Field injected_fields = 3;
  uint32 num_blocks = 4;
  // Tables with incremental changes (each Delta carries its own table_name).
  repeated delta.Delta deltas = 5;
  // Tables requiring a full state snapshot (each Table carries its own table_name).
  repeated table.Table states = 6;
}
```

The `Deltas` wrapper message in `delta.proto` is no longer needed and can be removed.

## Per-Table Consolidation — `src/patch.rs`

Currently `consolidate()` merges entire blocks via `Block::merge()`. If any single table's merge fails, the whole consolidation fails and everything falls back to full state. Refactor to consolidate per table:

1. **Walk the chain once**, collecting all blocks from HEAD back to `last_known` (oldest first).
2. **Group deltas by table**: for each table name that appears in any block, extract its delta from each block (a table may be absent in blocks where it had no changes).
3. **Merge each table independently**: merge that table's delta chain using `Delta::merge()`. If the merge fails for a table (e.g. column mismatch), fall back to full state for that table only.
4. **Per-table size comparison**: for each successfully merged table, compare the consolidated delta's encoded size against the table's state from the STATE file. Use whichever is smaller.
5. **Build the Patch** with `deltas` (tables where the delta won) and `states` (tables where the state won, plus tables whose merge failed).

`Block::merge()` is no longer needed after this refactor and can be removed.

## Full State Fallback

When the reference block can't be resolved (REPORTED missing, genesis, broken chain), all tables use full state — same as today, just populating `states` instead of `Payload::State`.

## SQL Generation — `src/sql.rs`

Refactor `patch_to_sql()` to handle both fields:

```rust
pub fn patch_to_sql(config: &Config, patch: &Patch) -> Result<Option<String>> {
    if patch.deltas.is_empty() && patch.states.is_empty() {
        return Ok(None);
    }

    // ... resolve injected fields ...

    let mut sql = String::from("BEGIN;\n");
    for delta in &patch.deltas {
        delta_to_sql(config, delta, &injected_fields, &mut sql)?;
    }
    for table in &patch.states {
        state_table_to_sql(config, table, &injected_fields, &mut sql)?;
    }
    sql.push_str("COMMIT;\n");
    Ok(Some(sql))
}
```

Extract the per-table logic from the current `state_to_sql()` into a `state_table_to_sql()` that handles a single table (using `table.table_name`).

## Display — `src/patch.rs`

Update the `Display` impl to show mixed payloads:

```
Patch:
  Head: 9c4d2e8f...
  Blocks: 3
  Deltas (1):
    'employees' [id, name, dept]
      Inserts (1): ...
  States (1):
    'departments' [dept_id, dept_name]
      (5 entries)
```

## Files Modified

| File | Change |
|---|---|
| `proto/patch.proto` | Replace `oneof payload` with `repeated delta.Delta deltas` + `repeated table.Table states` |
| `proto/delta.proto` | Remove `Deltas` wrapper message |
| `src/block.rs` | Remove `Block::merge()` |
| `src/patch.rs` | Per-table consolidation, per-table size comparison, mixed payload construction, updated Display |
| `src/sql.rs` | Handle mixed payloads, extract `state_table_to_sql()` |

## Tests

- Unit test in `sql.rs`: verify `patch_to_sql()` handles mixed deltas + states
- Test in `patch.rs`: verify per-table consolidation produces mixed payload when one table's merge fails
- Test in `patch.rs`: verify per-table size comparison (small table uses state, large table uses delta)
- Update existing acceptance tests to new Patch structure

## Verification

```
cargo build && cargo test
cargo fmt
cargo clippy
```
