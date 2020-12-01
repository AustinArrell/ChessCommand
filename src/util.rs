pub struct Pos{
    pub file:i8,
    pub rank:i8,
    
}

impl Pos
{
    pub fn new(file:i8,rank:i8)-> Self
    {
        return Pos{file,rank};
    }

}