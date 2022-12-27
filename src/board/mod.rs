use crate::{moves::Position, pieces::pieces::Pieces, player::Player, player_piece::PlayerPiece};

pub type Square = Option<PlayerPiece>;

pub struct Board {
    live_board: Vec<Vec<Square>>,
    captured: Vec<Pieces>,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            live_board: vec![vec![None; 12]; 12],
            captured: vec![],
        }
    }

    pub fn within_board(pos: &Position) -> bool {
        pos.0 <= 12 && pos.0 >= 1 && pos.1 <= 12 && pos.1 >= 1
    }

    pub fn moves(pos: &(u8, u8)) -> Vec<(u8, u8)> {
        todo!()
    }
}

pub trait GameState {
    fn player_color(&self, pos: Position) -> Option<Player>;
    fn player_piece(&self, pos: Position) -> Option<Pieces>;
}

impl GameState for Board {
    fn player_color(&self, pos: Position) -> Option<Player> {
        let piece = self.live_board[pos.0 as usize][pos.1 as usize].as_ref()?;
        Some(piece.player)
    }

    fn player_piece(&self, pos: Position) -> Option<Pieces> {
        let piece = self.live_board[pos.0 as usize][pos.1 as usize].as_ref()?;
        Some(piece.piece_type)
    }
}

// impl Display for Board {

// }
