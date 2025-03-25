
//See PieceBoard for details.
//This struct only exists to implement Ord on to get .sort() functionality.

//According to our data convention, this actually doesn't need any extra logic to Ord properly. Nice!
//Because our first three bits are the rank, and the next three are the file. This should sort without regard to role, only to position, just fine.
//We just need to be careful to only sort in the correct chunk sizes.

//Upon further consideration, to avoid packing issues I will be using BitPiece purely as an alias for u8. This should optimize my memory use.
//I feel like if I had just used C and then wrote a binding I would have faster code (as I am not using unsafe blocks).

pub type BitPiece = u8;

/*
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct BitPiece {
    pub data: u8
}
*/
