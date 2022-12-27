use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct King {}

impl Display for King {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "K")
    }
}
