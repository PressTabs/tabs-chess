use std::ops::{Index, IndexMut};
use super::{bit_square::BitSquare, board::Board, error::ChessError, position::Position};


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitBoard {
    //We could potentially flatten this, very easily mind you, for a performance boost. Consider it, perhaps.
    pub data: [[BitSquare; 8]; 8]
}

impl Index<Position> for BitBoard {
    type Output = BitSquare;

    fn index(&self, index: Position) -> &Self::Output {
        &self.data[index.rank as usize][index.file as usize]
    }
}

impl IndexMut<Position> for BitBoard {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.data[index.rank as usize][index.file as usize]
    }
}

//There is some overhead checking if these positions are valid. Use indexing if the pos is known to be safe.
impl BitBoard {
    pub fn get(&self, pos: Position) -> Result<&BitSquare, ChessError> {
        match pos.is_valid() {
            true => Ok(&self[pos]),
            false => Err(ChessError::IllegalMove),
        }
    }

    pub fn set(&mut self, bit_square: BitSquare, pos: Position) -> Result<(), ChessError> {
        match pos.is_valid() {
            true => {
                self[pos] = bit_square;
                Ok(())
            }
            false => Err(ChessError::InvalidPosition),
        }
    }

    //Doesn't actually put anything on the board. Perfect because we're initializing purely from a Board. TECHNICALLY SINCE Board::new() is const,
    //We could just make a new Board and then convert it to a BitBoard. This conversion should be optimized to nothing (only performed once) by the compiler.
    pub const fn new() -> Self {
        BitBoard { data: [[BitSquare::new(0); 8]; 8] }
    }
}

impl From<Board> for BitBoard {
    fn from(value: Board) -> Self {
        let mut board = BitBoard::new();
        for (rank, row) in value.data.iter().enumerate() {
            for (file, square_state) in row.iter().enumerate() {
                board[Position::new(rank as u8, file as u8)] = BitSquare::from(*square_state);
            }
        }

        //MUST IMPLEMENT CURRENT_TURN FLAG.
        todo!();
        board
    }
}