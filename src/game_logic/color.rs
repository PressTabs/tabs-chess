use std::ops::Not;

//These are better served as zero-sized structs
#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Self::Black,
            Color::Black => Self::White,
        }
    }
}
