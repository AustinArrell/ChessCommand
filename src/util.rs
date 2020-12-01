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

}