use crate::piece::Piece;
use crate::util::Pos;

pub struct Rook{
    pub is_white: bool,
    pub identity: char,
}

impl Rook{
    pub fn new(is_white:bool, identity: char) -> Self
    {
        return Rook{is_white,identity};
    }
}

impl Piece for Rook{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        println!("Trying to move from ({},{}) to ({},{})...",pos1.file,pos1.rank,pos2.file,pos2.rank);
        
        //The rook is simple. If either the file or rank stay the same, the move is valid
        if (pos1.file == pos2.file || pos1.rank == pos2.rank)
        {return true;}

        return false;
    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }
}