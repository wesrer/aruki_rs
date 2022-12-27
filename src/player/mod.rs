use std::fmt::{Display, Formatter};

pub enum Player {
    White,
    Black,
}

impl TryFrom<u8> for Player {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::White),
            1 => Ok(Self::Black),
            _ => Err("conversion only valued for 0(white) and 1(black)"),
        }
    }
}

impl From<bool> for Player {
    fn from(value: bool) -> Self {
        if value {
            Self::White
        } else {
            Self::Black
        }
    }
}

impl TryFrom<char> for Player {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' | 'W' => Ok(Self::White),
            'b' | 'B' => Ok(Self::Black),
            _ => Err("invalid character"),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            Self::White => 'W',
            Self::Black => 'B',
        };
        write!(f, "{}", letter)
    }
}
