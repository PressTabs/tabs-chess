use std::ops::{Index, IndexMut};

use super::{bit_board::BitBoard, color::Color, error::ChessError, piece::Piece, position::Position, square_state::SquareState, piece_type::PieceType};

/*
 * By convention let [0][0] mark a1 and [7][7] mark h8. 
 * The format is [Rank][File], but I believe that the Position struct is a
 * zero-cost abstraction that handles this for us.
 */
#[derive(Clone, Copy)]
pub struct Board {
    pub data: [[SquareState; 8]; 8],
    pub curr_turn: Color,
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

        //Must implement CURRENT_TURN Flag.
        todo!();
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

    //Genuinely barbaric function.
    pub const fn new() -> Self {
        let mut board = Board { data: [[SquareState::Empty; 8]; 8], curr_turn: Color::White };

        //Genuinely absolutely disgusting that I can't write this as a for loop. 
        //This frustrates me so much that I refuse to write comments for the remainder of this function.
        let mut color_idx: u8 = 0;
        while color_idx < 2 {
            let color = if color_idx == 0 { Color::White } else { Color::Black };

            let mut rank: u8 = 0;
            while rank < 2 {
                let mut file: u8 = 0;
                while file < 8 {
                    //I'm just going to hard code the position function. Who cares. Technically ineffecient, but wtvr man
                    let pos = Position::new(match color {
                        Color::White => rank,
                        Color::Black => 7 - rank
                    }, file);

                    use PieceType::*;
                    let piece = Piece { role: match rank {
                        1 => Pawn(false),
                        0 => match file {
                            0 | 7 => Rook(false),
                            1 | 6 => Knight,
                            2 | 5 => Bishop,
                            3 => Queen,
                            4 => King(false),
                            _ => panic!()
                        },
                        _ => panic!()
                    }, color, position: pos };

                    //Can't just index it apparently because const idx is not a thing. Amazing.
                    board.data[pos.rank as usize][pos.file as usize] = SquareState::Occupied(piece);

                    file += 1;
                }

                rank += 1;
            }

            color_idx += 1;
        }

        board
        }

    pub const fn mirror_position(pos: Position) -> Position {
        Position::new(7 - pos.rank, pos.file)
    }

    pub fn get_legal_moves(&self) -> Vec<Board> {

        todo!()
    }
}