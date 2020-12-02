use std::io;
use std::io::Write;

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

pub fn prompt(message: &str) -> String
    {
        //Shows a message to the user and prompts them for input. returns said input
        let mut input = String::new();
        print!("{}",message);

        // ensure message is printed before we pause for user input
        io::stdout().flush().expect("Console write error");
        io::stdin().read_line(&mut input).expect("Could not read line!");
        return input;
    }

pub mod cons
{
    pub struct Color
    {
        pub r:u8,
        pub g:u8,
        pub b:u8,
    }
    impl Color
    {
        pub fn new(r:u8,g:u8,b:u8) -> Self
        {
            Color {r,g,b}
        }
    }

    pub fn set_bg(col:&Color)
    {
        print!("\x1b[48;2;{};{};{}m", col.r, col.g, col.b);
    }

    pub fn set_fg(col:&Color)
    {
        print!("\x1b[38;2;{};{};{}m", col.r, col.g, col.b);
    }

    pub fn reset()
    {
        print!("\x1b[0m");
    }

    pub fn do_char(bg:&Color, fg:&Color ,c:char)
    {
        set_bg(bg);
        set_fg(fg);
        print!("{}",c);
        reset();
    }
}