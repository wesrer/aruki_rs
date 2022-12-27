use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, PartialEq)]
pub struct Minister {}

impl Display for Minister {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "M")
    }
}
