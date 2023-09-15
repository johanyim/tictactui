use rand_derive2::RandGen;
use std::fmt;
use termion::{color::{Fg, Red, Blue}, style::Reset};

mod ui {
    pub struct UI;
}
pub const DEFAULT:[[Option<Symbol>;3];3] =[   
    [None,None,None],
    [None,None,None],
    [None,None,None]];

#[derive(RandGen)]
#[derive(Copy, Clone)]
pub enum Symbol {
    X,
    O,
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::X => write!(f, "X"),
            Symbol::O => write!(f, "O"),

        }
    }

}


#[derive(Copy, Clone)]
pub enum State {
    Winner(Symbol),
    Draw,
    Unfinished,
}

pub struct Game {
    pub history: Vec<(Board, Symbol)>,
    pub current_board: Board,
    pub next_player: Symbol,
}

impl Game {
    pub fn new() -> Game {
        let s : Symbol = Symbol::generate_random();
        Game {
            history: vec![],
            current_board: Board{squares: DEFAULT, state: State::Unfinished},
            next_player: s,
        }
    }
    
    pub fn show(&self) {
        self.current_board.show()
    }

    pub fn play(&mut self, row:usize, col:usize) {
        let previous = (self.current_board.clone(), self.next_player); 
        match self.current_board.put(self.next_player, row, col) {
            Err(_) => return,
            Ok(_) => (),
        }
        //add to history
        self.history.push(previous);
        //switch player
        self.next_player = match self.next_player {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
        };
        //update current state
        self.current_board.update_state();

    }

    pub fn revert(&mut self){
        let (board, symbol) = self.history.pop().unwrap();
        self.current_board = board;
        self.next_player = symbol;
    }
    
    //string for the symbol at square row, col
    pub fn at(&self, row: usize, col: usize) -> String {
        match self.current_board.squares[row][col] {
            Some(Symbol::X) => return format!("{}X{}", Fg(Blue), Reset),
            Some(Symbol::O) => return format!("{}O{}", Fg(Red), Reset),
            None            => return String::from(" "),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    pub squares: [[Option<Symbol>;3];3],
    pub state: State,
}
impl Board {
    fn show(&self) {
        for row in &self.squares{
            for square in row{
                match square {
                    Some(Symbol::O) => print!("O"),
                    Some(Symbol::X) => print!("X"),
                    None            => print!("."),
                };
            }
            println!("");
        }
        println!("");
    }

    fn put(&mut self, s: Symbol, row: usize, col: usize) -> Result<(), String>{
        //invalid index 
        if row > self.squares.len() || col > self.squares[0].len() {
            return Err(String::from("Outside array index"));
        }
        //if occupied
        match self.squares[row][col] {
            Some(_) => {
                return Err(String::from("Occupied array index"));}
            None => (),
        }
        //game concluded
        match self.state {
            State::Unfinished => (),
            _ => {
                return Err(String::from("No valid moves"))
            }

        }
        self.squares[row][col] = Some(s); 
        return Ok(()) 
    }

    fn update_state(&mut self){
        use Symbol::{X,O};
        use State::*;

            //Check Draw
            if let [
                [Some(_),Some(_),Some(_)],
                [Some(_),Some(_),Some(_)],
                [Some(_),Some(_),Some(_)]
            ] = self.squares {
                self.state = Draw;
            }

            // Check rows
            for row in self.squares {
                match row {
                    [Some(X), Some(X), Some(X)] => self.state = Winner(X),
                    [Some(O), Some(O), Some(O)] => self.state = Winner(O),
                    _ => (),
                }
            }    

            // Check columns
            for col in 0..3 {
                match [self.squares[0][col],self.squares[1][col],self.squares[2][col]] {
                    [Some(X), Some(X), Some(X)] => self.state = Winner(X),
                    [Some(O), Some(O), Some(O)] => self.state = Winner(O),
                    _ => (),
                }
            }
            
            // Check diagonals
            match [self.squares[0][0], self.squares[1][1], self.squares[2][2]] {
                    [Some(X), Some(X), Some(X)] => self.state = Winner(X),
                    [Some(O), Some(O), Some(O)] => self.state = Winner(O),
                    _ => (),
            }
            match [self.squares[0][2], self.squares[1][1], self.squares[2][0]] {
                    [Some(X), Some(X), Some(X)] => self.state = Winner(X),
                    [Some(O), Some(O), Some(O)] => self.state = Winner(O),
                    _ => (),
            }
                

        //Unfinished otherwise
        //Printing
       // println!("{}", match self.state{
       //    State::Winner(Symbol::X) => "Winner is X",
       //    State::Winner(Symbol::O) => "Winner is O",
       //    State::Draw => "Draw",
       //    State::Unfinished => "Unfinished",
       // });
    }
}


impl Game {
    pub fn show_history(&self) {
        for r in 0..3{    
            for (b,_) in self.history.iter() {
                for square in b.squares[r as usize]{
                    match square {
                        Some(Symbol::O) => print!("O"),
                        Some(Symbol::X) => print!("X"),
                        None            => print!("."),
                    };
                }print!("|");
            }println!("");
        }
        
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enum_match() {
        
    }
}
