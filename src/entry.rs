pub use crate::proto::entry::Entry;

/// Creates a new Entry from key and value vectors.
pub fn new(key: Vec<String>, value: Vec<String>) -> Entry {
    Entry { key, value }
}
