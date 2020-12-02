use crate::piece::Piece;
use crate::util::Pos;

pub struct Air {}

impl Air
{
    pub fn new() -> Self
    {
        return Air {};
    }
}

impl Piece for Air
{
    fn verify_move(&self, pos1:&Pos, pos2:&Pos) -> bool
    {false}
    fn get_identity(&self) -> char
    {' '}
    fn is_white(&self) -> bool
    {false}
}
