use crate::{moves::Position, pieces::pieces::Pieces, player::Player, player_piece::PlayerPiece};

use super::Board;

pub trait BoardSetup {
    fn empty() -> Self;
    fn starting_pos() -> Self;
    fn with_pieces<const K: usize, const L: usize, const M: usize, const N: usize>(
        white_board: [(Pieces, Position); K],
        white_captured: [Pieces; L],
        black_board: [(Pieces, Position); M],
        black_captured: [Pieces; N],
    ) -> Self;
}

impl BoardSetup for Board {
    fn empty() -> Self {
        Self {
            live_board: [[None; 12]; 12],
            white_captured: vec![],
            black_captured: vec![],
        }
    }

    fn with_pieces<const K: usize, const L: usize, const M: usize, const N: usize>(
        white_board: [(Pieces, Position); K],
        white_captured: [Pieces; L],
        black_board: [(Pieces, Position); M],
        black_captured: [Pieces; N],
    ) -> Self {
        let mut board = Self::empty();

        for (piece, Position(row, col)) in white_board {
            let piece = PlayerPiece::new('w'.try_into().unwrap(), piece);
            board.live_board[row as usize][col as usize] = Some(piece);
        }

        for (piece, Position(row, col)) in black_board {
            let piece = PlayerPiece::new('b'.try_into().unwrap(), piece);
            board.live_board[row as usize][col as usize] = Some(piece);
        }

        board.white_captured = white_captured.into();
        board.black_captured = black_captured.into();

        board
    }

    fn starting_pos() -> Self {
        let mut board = Self::empty();

        let w_king = PlayerPiece::new(Player::White, Pieces::king());
        let b_king = PlayerPiece::new(Player::Black, Pieces::king());

        let w_rook = PlayerPiece::new(Player::White, Pieces::rook());
        let w_rook2 = w_rook.clone();
        let b_rook = PlayerPiece::new(Player::Black, Pieces::rook());
        let b_rook2 = b_rook.clone();

        let w_arrow = PlayerPiece::new(Player::White, Pieces::arrow());
        let w_arrow2 = w_arrow.clone();
        let b_arrow = PlayerPiece::new(Player::Black, Pieces::arrow());
        let b_arrow2 = b_arrow.clone();

        let w_lance= PlayerPiece::new(Player::White, Pieces::lance());
        let w_lance2 = w_lance.clone();
        let b_lance = PlayerPiece::new(Player::Black, Pieces::lance());
        let b_lance2= b_lance.clone();

        let w_pike= PlayerPiece::new(Player::White, Pieces::pike());
        let w_pike2= w_pike.clone();
        let b_pike = PlayerPiece::new(Player::Black, Pieces::pike());
        let b_pike2= b_lance.clone();

        let w_golden_dragon = PlayerPiece::new(Player::White, Pieces::golden_dragon());
        let b_golden_dragon = PlayerPiece::new(Player::Black, Pieces::golden_dragon());

        let w_silver_dragon = PlayerPiece::new(Player::White, Pieces::silver_dragon());
        let b_silver_dragon = PlayerPiece::new(Player::Black, Pieces::silver_dragon());
       
        let w_jester = PlayerPiece::new(Player::White, Pieces::jester());
        let b_jester = PlayerPiece::new(Player::Black, Pieces::jester());
        
        board.live_board[0][0] = Some(w_rook);
        board.live_board[0][1] = Some(w_arrow);
        board.live_board[0][2] = Some(w_lance);
        board.live_board[0][3] = Some(w_pike);
        board.live_board[0][4] = Some(w_golden_dragon);
        board.live_board[0][5] = Some(w_king);
        board.live_board[0][6] = Some(w_jester);
        board.live_board[0][7] = Some(w_silver_dragon);
        board.live_board[0][8] = Some(w_pike2);
        board.live_board[0][9] = Some(w_lance2);
        board.live_board[0][10] = Some(w_arrow2);
        board.live_board[0][11] = Some(w_rook2);

        board.live_board[11][0] = Some(b_rook);
        board.live_board[11][1] = Some(b_arrow);
        board.live_board[11][2] = Some(b_lance);
        board.live_board[11][3] = Some(b_pike);
        board.live_board[11][4] = Some(b_golden_dragon);
        board.live_board[11][5] = Some(b_king);
        board.live_board[11][6] = Some(b_jester);
        board.live_board[11][7] = Some(b_silver_dragon);
        board.live_board[11][8] = Some(b_pike2);
        board.live_board[11][9] = Some(b_lance2);
        board.live_board[11][10] = Some(b_arrow2);
        board.live_board[11][11] = Some(b_rook2);

        for c in 0..12 {
            let w_pawn = PlayerPiece::new(Player::White, Pieces::pawn());
            let b_pawn = PlayerPiece::new(Player::Black, Pieces::pawn());

            board.live_board[1][c] = Some(w_pawn);
            board.live_board[10][c] = Some(b_pawn);
        }

        board
    }
    }
