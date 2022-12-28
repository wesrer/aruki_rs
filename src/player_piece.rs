use crate::{
    board::{Board, GameState},
    moves::Position,
    pieces::pieces::Pieces,
    player::Player,
};
#[derive(Clone, Copy)]
pub struct PlayerPiece {
    pub player: Player,
    pub piece_type: Pieces,
}

impl PlayerPiece {
    pub fn new(player: Player, piece_type: Pieces) -> Self {
        Self { player, piece_type }
    }

    pub fn get(pos: Position, board: Board) -> Option<Self> {
        let piece_type = board.player_piece(pos)?;
        let player = board.player_color(pos)?;

        Some(Self::new(player, piece_type))
    }
}