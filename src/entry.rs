pub use crate::proto::entry::Entry;

use std::fmt;

impl From<Entry> for (Vec<String>, Vec<String>) {
    fn from(entry: Entry) -> Self {
        (entry.key, entry.value)
    }
}

impl From<(Vec<String>, Vec<String>)> for Entry {
    fn from((key, value): (Vec<String>, Vec<String>)) -> Self {
        Entry { key, value }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.key, self.value)
    }
}
