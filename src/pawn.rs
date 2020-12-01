use crate::piece::Piece;
use crate::util::Pos;

pub struct Pawn{
    pub is_white: bool,
    pub identity: char,
}

impl Pawn{
    pub fn new(is_white:bool, identity: char) -> Self
    {
        return Pawn{is_white,identity};
    }
}

impl Piece for Pawn{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        return true;
    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }
}