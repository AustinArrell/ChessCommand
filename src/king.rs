use crate::piece::Piece;
use crate::util::Pos;



pub struct King{
    pub is_white: bool,
    pub identity: char,
}

impl King{
    pub fn new(is_white:bool, identity: char) -> Self
    {
        return King{is_white,identity};
    }
}

impl Piece for King{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        //Distance formula to find the distance between 2 points. Kings move distance should be no more than 1
        //d=sqrt((x2​−x1​)^2+(y2​−y1​)^2)
        let distance_moved = (((pos2.file - pos1.file).pow(2) as f32) + ((pos2.rank - pos1.rank).pow(2)as f32)).sqrt();

        if distance_moved as i8 == 1
        {return true;}
        return false;
    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }
}



