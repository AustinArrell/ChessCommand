use std::io;
use std::io::Write;
use crate::util::Pos;

pub struct Game
{
    running:bool,
    white_turn: bool,
    is_server: bool,
    //board: Board

}

impl Game
{
    pub fn new(white_turn: bool, is_server: bool)-> Self
    {
        return Game{running:true,white_turn,is_server};
    }

    pub fn run(&self)
    {
        while(self.running)
        {
            let start_pos = Pos::from(&Game::prompt("Enter starting square: "));
            if !start_pos.valid()
            { continue;}

            let (piece,dest_string) = Game::parse_move(Game::prompt("Enter move:" ));
            let dest_pos = Pos::from(&dest_string);
            if !dest_pos.valid()
            { continue; }
        }
    }

    fn prompt(message: &str) -> String
    {
        //Shows a message to the user and prompts them for input. returns said input
        let mut input = String::new();
        print!("{}",message);

        // ensure message is printed before we pause for user input
        io::stdout().flush().expect("Console write error");
        io::stdin().read_line(&mut input).expect("Could not read line!");
        return input;
    }

    fn parse_move(input: String) -> (char, String) 
    {
        //Takes a string and parses the move data from it
        let piece = Game::get_piece(&input);
        let destination = Game::get_destination(&input);
        return (piece,destination);
    }

    fn get_piece(input: &String) -> char
    {
        //Parse the piece data from user input
        if input.chars().next().unwrap().is_uppercase()
        {return input.chars().next().unwrap() }
        else
        {return 'P' };

    }

    fn get_destination(input: &String) -> String
    {
        //Parse the destination data from user input
        if input.chars().next().unwrap().is_uppercase()
        { return input[1..].to_string() }
        else
        { return input.to_string() };
    }

}