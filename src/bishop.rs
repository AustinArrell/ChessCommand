use crate::piece::Piece;
use crate::util::Pos;

pub struct Bishop{
    pub is_white: bool,
    pub identity: char,
}

impl Bishop{
    pub fn new(is_white:bool, identity: char) -> Self
    {
        return Bishop{is_white,identity};
    }
}

impl Piece for Bishop{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        //The bishops move is valid if the number of moved spaces on the x is equal to the number of moved spaces on the y 
        let file_dif = (pos2.file - pos1.file).abs();
        let rank_dif = (pos2.rank - pos1.rank).abs();
        
        if file_dif == rank_dif
        {return true;}
        return false;
    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }
}