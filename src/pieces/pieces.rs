use std::fmt::Display;

use enum_display_derive::Display;

use super::king::King;

#[derive(Display)]
pub enum Pieces {
    King(King),
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
    // Minister,
    // Javelin,
    // Rook
}

impl Pieces {
    pub fn king() -> Self {
        Self::King(King {})
    }
}

impl TryFrom<&str> for Pieces {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "k" | "K" => Ok(Pieces::King(King {})),
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
