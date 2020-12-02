use crate::util::Pos;

static PIECE_SYMBOLS :[(char,char);6] =
[
    ('K','♔'),
    ('Q','♕'),
    ('R','♖'),
    ('B','♗'),
    ('N','♘'),
    ('P','♙'),
];

pub trait Piece{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool;
    fn get_identity(&self) -> char;
    fn is_white(&self) -> bool;

    fn get_symbol(&self) -> char
    {
        match self.get_identity()
        {
            'K' => '♚',
            'Q' => '♛',
            'R' => '♜',
            'B' => '♝',
            'N' => '♞',
            'P' => '♙',
             _  => ' ',
        }
    }
}