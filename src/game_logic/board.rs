use std::ops::{Index, IndexMut};

use super::{bit_board::BitBoard, error::ChessError, position::Position, square_state::SquareState};

//Yay. All of these enums, structs, and functions should probably be put into other files.
//Need to make Board indexable by Position.
#[derive(Clone, Copy)]
pub struct Board {
    pub data: [[SquareState; 8]; 8],
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.data[index.rank as usize][index.file as usize]
    }
}

//Is this a good idea?
impl Index<Position> for Board {
    type Output = SquareState;

    fn index(&self, index: Position) -> &Self::Output {
        &self.data[index.rank as usize][index.file as usize]
    }
}

impl From<BitBoard> for Board {
    fn from(value: BitBoard) -> Self {
        let mut board = Board::new();
        for (rank, row) in value.data.iter().enumerate() {
            for (file, bit_square) in row.iter().enumerate() {
                let pos = Position::new(rank as u8, file as u8);
                board[pos] = SquareState::from_bit_square(*bit_square, pos);
            }
        }
        board
    }
}

impl Board {
    pub fn get(&self, pos: Position) -> Result<&SquareState, ChessError> {
        match pos.is_valid() {
            true => Ok(&self[pos]),
            false => Err(ChessError::IllegalMove),
        }
    }

    pub fn set(&mut self, square_state: SquareState, pos: Position) -> Result<(), ChessError> {
        match pos.is_valid() {
            true => {
                self[pos] = square_state;
                Ok(())
            }
            false => Err(ChessError::InvalidPosition),
        }
    }

    pub fn new() -> Self {
        Board { data: [[SquareState::Empty; 8]; 8] }
    }
}
