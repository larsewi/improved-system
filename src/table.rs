use std::collections::HashMap;

mod proto {
    pub mod entry {
        include!(concat!(env!("OUT_DIR"), "/entry.rs"));
    }
    pub mod table {
        include!(concat!(env!("OUT_DIR"), "/table.rs"));
    }
    pub mod state {
        include!(concat!(env!("OUT_DIR"), "/state.rs"));
    }
}

pub use proto::entry::Entry as Row;
pub use proto::state::State;
pub use proto::table::Table;

/// Builds a map from primary key to subsidiary value for all rows in a table.
pub fn table_to_map(table: &Table) -> HashMap<&Vec<String>, &Vec<String>> {
    table
        .rows
        .iter()
        .map(|row| (&row.key, &row.value))
        .collect()
}
