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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_entry_to_tuple() {
        let entry = Entry {
            key: vec!["k".to_string()],
            value: vec!["v".to_string()],
        };
        let (key, value): (Vec<String>, Vec<String>) = entry.into();
        assert_eq!(key, vec!["k"]);
        assert_eq!(value, vec!["v"]);
    }

    #[test]
    fn test_from_tuple_to_entry() {
        let entry: Entry = (vec!["k".to_string()], vec!["v".to_string()]).into();
        assert_eq!(entry.key, vec!["k"]);
        assert_eq!(entry.value, vec!["v"]);
    }

    #[test]
    fn test_display() {
        let entry = Entry {
            key: vec!["k1".to_string(), "k2".to_string()],
            value: vec!["v1".to_string()],
        };
        assert_eq!(format!("{}", entry), r#"["k1", "k2"] -> ["v1"]"#);
    }
}
