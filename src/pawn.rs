use crate::piece::Piece;
use crate::util::Pos;

pub struct Pawn{
    pub is_white: bool,
    pub identity: char,
}

impl Pawn{
    pub fn new(is_white:bool) -> Self
    {
        return Pawn{is_white,identity:'P'};
    }
}

impl Piece for Pawn{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        if self.is_white == true{
            return pos2.file == pos1.file+1 && pos2.rank == pos1.rank;
        }
        return pos2.file == pos1.file-1 && pos2.rank == pos1.rank;

    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }
}