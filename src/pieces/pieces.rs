use std::fmt::Display;

use enum_display_derive::Display;

use super::{king::King, minister::Minister};

#[derive(Display)]
pub enum Pieces {
    King(King),
    Minister(Minister),
    // Jester,
    // Arrow
    // Lance,
    // GreaterLance,
    // GreaterRiver,
    // LesserRiver,
    // Pike,
    // GreaterPike,
    // Sword,
    // LongSword,
    // Pawn,
    // Javelin,
    // Rook
}

impl Pieces {
    pub fn king() -> Self {
        Self::King(King {})
    }

    pub fn minister() -> Self {
        Self::Minister(Minister {})
    }
}

impl TryFrom<&str> for Pieces {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "k" | "K" => Ok(Self::king()),
            "m" | "M" => Ok(Self::minister()),
            _ => Err("invalid character"),
        }
    }
}

impl TryFrom<String> for Pieces {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}
