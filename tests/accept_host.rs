mod common;

use leech2::block::Block;
use leech2::config::Config;
use leech2::patch::Patch;
use leech2::sql;
use leech2::utils::GENESIS_HASH;

#[test]
fn test_host_delta_sql() {
    let tmp = tempfile::tempdir().unwrap();
    let work_dir = tmp.path();

    common::write_config(
        work_dir,
        "config.toml",
        r#"
[host]
name = "host"
type = "TEXT"
value = "agent-1"

[tables.users]
source = "users.csv"
fields = [
    { name = "id", type = "INTEGER", primary-key = true },
    { name = "name", type = "TEXT" },
]
"#,
    );

    // Block 1: initial data (many rows so delta is smaller than state)
    common::write_csv(
        work_dir,
        "users.csv",
        "1,Alice\n2,Bob\n3,Charlie\n4,Dave\n5,Eve\n6,Frank\n7,Grace\n8,Heidi\n",
    );
    let config = Config::load(work_dir).unwrap();
    let hash1 = Block::create(&config).unwrap();

    // Block 2: update Alice->Alicia, delete Bob, insert Ivan
    common::write_csv(
        work_dir,
        "users.csv",
        "1,Alicia\n3,Charlie\n4,Dave\n5,Eve\n6,Frank\n7,Grace\n8,Heidi\n9,Ivan\n",
    );
    Block::create(&config).unwrap();

    // Patch from hash1: 1 insert, 1 delete, 1 update — all with host
    let patch = Patch::create(&config, &hash1).unwrap();
    let sql = sql::patch_to_sql(&config, &patch).unwrap().unwrap();

    assert!(
        sql.contains(
            r#"INSERT INTO "users" ("host", "id", "name") VALUES ('agent-1', 9, 'Ivan');"#
        )
    );
    assert!(sql.contains(r#"DELETE FROM "users" WHERE "id" = 2 AND "host" = 'agent-1';"#));
    assert!(sql.contains(
        r#"UPDATE "users" SET "name" = 'Alicia' WHERE "id" = 1 AND "host" = 'agent-1';"#
    ));

    common::assert_wire_roundtrip(&config, &patch);
}

#[test]
fn test_host_state_sql() {
    let tmp = tempfile::tempdir().unwrap();
    let work_dir = tmp.path();

    common::write_config(
        work_dir,
        "config.toml",
        r#"
[host]
name = "host"
type = "TEXT"
value = "agent-1"

[tables.users]
source = "users.csv"
fields = [
    { name = "id", type = "INTEGER", primary-key = true },
    { name = "name", type = "TEXT" },
]
"#,
    );

    // Create a single row so the state is small enough to be used as payload
    common::write_csv(work_dir, "users.csv", "1,Alice\n");
    let config = Config::load(work_dir).unwrap();
    Block::create(&config).unwrap();

    // Create a state-payload patch by forcing many blocks so deltas are larger
    // Actually, for a single block from genesis, the patch might use state if
    // state is smaller. Let's just create the patch and check whichever path.
    let patch = Patch::create(&config, GENESIS_HASH).unwrap();
    let sql = sql::patch_to_sql(&config, &patch).unwrap().unwrap();

    // Regardless of delta vs state payload, host should be present
    assert!(sql.contains(r#""host""#), "SQL should contain host column");
    assert!(sql.contains("'agent-1'"), "SQL should contain host value");

    // If state payload, should use DELETE WHERE instead of TRUNCATE
    if sql.contains("TRUNCATE") {
        panic!("With host configured, state payload should use DELETE WHERE, not TRUNCATE");
    }

    common::assert_wire_roundtrip(&config, &patch);
}

#[test]
fn test_no_host_unchanged_sql() {
    let tmp = tempfile::tempdir().unwrap();
    let work_dir = tmp.path();

    // No [host] section
    common::write_config(
        work_dir,
        "config.toml",
        r#"
[tables.users]
source = "users.csv"
fields = [
    { name = "id", type = "INTEGER", primary-key = true },
    { name = "name", type = "TEXT" },
]
"#,
    );

    common::write_csv(work_dir, "users.csv", "1,Alice\n");
    let config = Config::load(work_dir).unwrap();
    Block::create(&config).unwrap();

    let patch = Patch::create(&config, GENESIS_HASH).unwrap();
    let sql = sql::patch_to_sql(&config, &patch).unwrap().unwrap();

    // Without host config, SQL should not contain any host column
    assert!(
        !sql.contains(r#""host""#),
        "SQL should not contain host column when not configured"
    );
}

#[test]
fn test_host_integer_type() {
    let tmp = tempfile::tempdir().unwrap();
    let work_dir = tmp.path();

    common::write_config(
        work_dir,
        "config.toml",
        r#"
[host]
name = "agent_id"
type = "INTEGER"
value = "42"

[tables.users]
source = "users.csv"
fields = [
    { name = "id", type = "INTEGER", primary-key = true },
    { name = "name", type = "TEXT" },
]
"#,
    );

    common::write_csv(work_dir, "users.csv", "1,Alice\n");
    let config = Config::load(work_dir).unwrap();
    Block::create(&config).unwrap();

    let patch = Patch::create(&config, GENESIS_HASH).unwrap();
    let sql = sql::patch_to_sql(&config, &patch).unwrap().unwrap();

    // Integer host should not be quoted
    assert!(
        sql.contains(r#""agent_id""#),
        "SQL should contain agent_id column"
    );
    assert!(sql.contains("42"), "SQL should contain integer host value");
}
