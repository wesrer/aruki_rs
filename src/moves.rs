use crate::{pieces::pieces::Pieces, player_piece::PlayerPiece};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub u8, pub u8);

pub enum Moves {
    Move {
        piece: PlayerPiece,
        starting: Position,
        ending: Position,
        captured: Option<Pieces>,
    },
    Promotion {
        piece: PlayerPiece,
        starting: Position,
        ending: Position,
    },
    Evolution {
        piece: PlayerPiece,
        starting: Position,
        ally_piece: Pieces,
        ending_piece: Pieces,
    },
}

// impl Moves {
//     fn to_move(starting: Position, ending:Position, board: Board) -> Option<Self> {
//         let piece = board.player_piece(starting)?;

//         piece.is_valid_move();

//         Self::Move {
//             player: board.player_color(starting),
//             starting,
//             ending
//         }
//     }

//     // TODO: refactor into Error
//     fn to_capture(starting: Position, ending: Position, board: Board) -> Option<Self> {
//         if board.player_piece(ending)? == piece {
//             if board.player_color(starting) != board.player_color(ending) {
//                 Self::Capture { starting, ending, captured: board.player_piece(ending).}
//             }
//         }

//         None
//     }
// }
