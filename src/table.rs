use std::collections::HashMap;

pub use crate::entry::Entry;
pub use crate::proto::state::State;
pub use crate::proto::table::Table;

/// Builds a map from primary key to subsidiary value for all rows in a table.
pub fn table_to_map(table: &Table) -> HashMap<&Vec<String>, &Vec<String>> {
    table
        .rows
        .iter()
        .map(|row| (&row.key, &row.value))
        .collect()
}
