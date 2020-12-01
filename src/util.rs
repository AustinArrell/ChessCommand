pub struct Pos{
    pub rank:i32,
    pub file:i32,

}

impl Pos
{
    pub fn new(rank:i32,file:i32)-> Self
    {
        return Pos{rank,file};
    }
    pub fn from(square:&String) -> Self
    {
        let rank = square.chars().nth(0).unwrap();
        let file = square.chars().nth(1).unwrap();

        if rank < 'a' || rank > 'h' || file < '1' || file > '8'
        {
            return Pos {rank:-1,file:-1};
        }

        return Pos {
            rank : (rank as i32) - 97,                      // 97 is the integer representation of 'a'
            file : (file.to_digit(10).unwrap() as i32) -1   // Minus one because arrays start at 0
        };
    }
    pub fn valid(&self) -> bool
    {
        return self.rank > 0 || self.file > 0;
    }
}