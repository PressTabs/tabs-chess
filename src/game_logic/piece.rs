use super::board::Board;
use super::color::Color;
use super::error::ChessError;
use super::piece_type::PieceType;
use super::position::Position;
use super::square_state::SquareState;

//This allows for easy promotion logic on pawns, and also it doesn't matter anyways.
//Position here is technically duplicated data from board, but whatever. In total it's 128 bytes.
#[derive(Clone, Copy)]
pub struct Piece {
    pub role: PieceType,
    pub color: Color,
    pub position: Position,
}

impl Piece {
    //Consider returning whether a legal move is availible.
    pub fn try_move(&mut self, board: &mut Board, pos: Position) -> Result<(), ChessError> {
        let state = board.get(pos)?;

        todo!()
    }
}
