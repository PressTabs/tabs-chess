use std::ops::Add;

//Zero-indexed. Could use just one u8 if seeking effeciency. Am not. Needs conversion method to standard notation.
//Needs to impl From<(u8, u8)>, TryFrom<String>.
#[derive(Copy, Clone)]
pub struct Position {
    pub rank: u8,
    pub file: u8,
}

impl Position {
    pub fn is_valid(&self) -> bool {
        //Unsigned, so we don't need to check for negatives.
        self.rank < 8 && self.file < 8
    }

    pub fn new(rank: u8, file: u8) -> Self {
        Position { rank, file }
    }
}

impl From<(u8, u8)> for Position {
    fn from(value: (u8, u8)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.rank + rhs.rank, self.file + rhs.file)
    }
}
