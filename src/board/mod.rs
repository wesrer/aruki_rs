use crate::{pieces::{Piece, pieces::Pieces}, player::Player};

pub struct Board {
    live_board: Vec<Vec<Option<Piece>>>,
    captured: Vec<Pieces>
}

// impl Board {
//     fn new() -> Self {
//         Self {
//             rows: Vec::with_capacity(12),
//             cols: Vec::with_capacity(12)
//         }
//     }
// }

// impl Default for Self {

// }

// impl Display for Board {

// }

impl Board {
    pub fn within_board(pos: &(u8, u8)) -> bool {
        pos.0 <= 12 && pos.0 >= 1 && pos.1 <= 12 && pos.1 >= 1
    }

    pub fn moves(pos: &(u8, u8)) -> Vec<(u8, u8)> {
        todo!()
    }
}

pub trait GameState {
    fn player_color(&self, pos: (u8, u8)) -> Option<Player>;
    fn player_piece(&self, pos: (u8, u8)) -> Option<Pieces>;
}

impl GameState for Board {
    fn player_color(&self, (row, col): (u8, u8)) -> Option<Player> {
        let piece = self.live_board[row as usize][col as usize].as_ref()?;
        Some(piece.player)
    }

    fn player_piece(&self, (row, col): (u8, u8)) -> Option<Pieces> {
        let piece = self.live_board[row as usize][col as usize].as_ref()?;
        Some(piece.piece_type)
    }
} 


// impl Display for Board {
    
// }