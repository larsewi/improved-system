pub use crate::proto::update::Update;

use std::fmt;

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {:?} -> {:?}",
            self.key, self.old_value, self.new_value
        )
    }
}
