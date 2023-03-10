use crate::{
    board::{Board, BoardState},
    pieces::pieces::Pieces,
    player_piece::PlayerPiece,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub u8, pub u8);

pub type MoveConfig<'a> = (Position, Position, &'a Board);

pub enum Move {
    MovePiece {
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

impl<'a> TryFrom<MoveConfig<'a>> for Move {
    type Error = &'static str;

    fn try_from((starting, ending, board): MoveConfig) -> Result<Self, Self::Error> {
        if let None = board.player_color(starting) {
            return Err("Invalid starting position");
        }

        if board.player_color(starting) == board.player_color(ending) {
            return Err("Invalid move");
        }

        // NOTE: These unwraps are fine because these cases have been already handled upstream
        let piece = PlayerPiece::get(starting, board).unwrap();

        let captured = match board.player_color(ending) {
            Some(_) => None,
            None => {
                let captured = board.player_piece(ending).unwrap();
                Some(captured)
            }
        };

        Ok(Self::MovePiece {
            piece,
            starting,
            ending,
            captured,
        })
    }
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
