# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

**Prerequisite:** `protobuf-compiler` must be installed (`sudo apt install protobuf-compiler` / `brew install protobuf`).

- **Build the Rust library:** `cargo build`
- **Run Rust tests:** `cargo test`
- **Run a single test:** `cargo test <test_name>` (e.g. `cargo test test_merge_rule5`)
- **Build the C test harness:** `make` (builds `tests/leech2` and links against `target/debug/libleech2.so`)
- **Clean C artifacts:** `make clean`

The CI pipeline (`.github/workflows/rust.yml`) runs `cargo build` and `cargo test` on the master branch.

## Architecture

leech2 is a Rust `cdylib` that exposes a C-compatible API for tracking changes to CSV-backed database tables. It implements a git-like content-addressable block chain for change history.

### C FFI Layer (`src/lib.rs`, `include/leech2.h`)

The public API exposed to C callers:
- `lch_init(work_dir)` — initialize from a work directory containing `config.toml` and CSV sources
- `lch_block_create()` — snapshot current CSV state, compute deltas from previous state, create a new block
- `lch_patch_create(last_known_hash)` — build a patch (consolidated deltas or full state) from `last_known` to HEAD
- `lch_patch_to_sql(buf, len)` — convert a protobuf-encoded patch to SQL (TODO: stub)

### Core Data Model

- **Config** (`src/config.rs`) — TOML-based config defining tables, their CSV source files, field names, and primary keys. Stored in a global `OnceLock`. Work directory path set at init time.
- **Table** (`src/table.rs`) — In-memory representation of a CSV table. Records stored as `HashMap<Vec<String>, Vec<String>>` (primary key → subsidiary columns). Fields are reordered so primary key columns come first.
- **State** (`src/state.rs`) — Snapshot of all tables at a point in time. Serialized to protobuf and persisted as `STATE` file in the work directory.
- **Delta** (`src/delta.rs`) — Diff between two states for a single table: inserts, deletes, and updates. Contains the merge logic implementing 15 rules (see `DELTA_MERGING_RULES.md`).
- **Block** (`src/block.rs`) — A content-addressable unit containing a timestamp, parent hash, and a list of deltas. Blocks form a linked chain. SHA-1 hashed and stored by hash in the work directory.
- **Patch** (`src/patch.rs`) — Consolidates multiple blocks from HEAD back to a `last_known` hash by merging deltas. Chooses between sending consolidated deltas or full state based on encoded size.
- **Head** (`src/head.rs`) — Reads/writes the `HEAD` file tracking the current block hash.
- **Storage** (`src/storage.rs`) — File I/O with `fs2` file locking (exclusive for writes, shared for reads). All files stored in the work directory.

### Protobuf

Proto definitions are in `proto/`. Code is generated at build time via `prost-build` (`build.rs`) into `OUT_DIR` and included via `src/proto.rs`. Domain types in `src/` have `From` impls to convert to/from their proto counterparts.

**Note:** leech2 has not been released yet, so there are no backwards-compatibility constraints on the proto specs. Reusing or renumbering wire fields is perfectly fine.

### Work Directory Layout

The work directory (passed to `lch_init`) contains:
- `config.toml` — table definitions
- CSV source files referenced by config
- `HEAD` — current block hash (text)
- `STATE` — protobuf-encoded state snapshot
- `<sha1_hash>` — protobuf-encoded block files

### Delta Merging Rules

The 15 merge rules in `src/delta.rs` are fully specified in `DELTA_MERGING_RULES.md`. When modifying merge logic, refer to that document and ensure all rule tests pass.
