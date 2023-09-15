
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Read, Write, stdin,stdout, Stdout};
use crate::Game;
use termion::{color::{Fg, Red, Blue}, style::Reset};

                        //  x y
pub enum Direction {
    Up,Left,Down,Right,
}

pub struct UI {
    //origin to draw the board 
    origin_x: u16,
    origin_y: u16,
    //current square selected
    p_x: i16, 
    p_y: i16,
    //literal cursor position on terminal
    cursor: (u16, u16),
    stdout: RawTerminal<Stdout>,
    g: Game,
}

pub const OFFSET_X: u16 = 5;
pub const OFFSET_Y: u16 = 3;

impl UI{
    pub fn start(g: Game) -> Self {
        
        //define a screen 
        //let mut stdout = stdout().lock().into_raw_mode().unwrap(); 
        //You can use lock to ensure threads don't interfere with each other
        let stdout = stdout().into_raw_mode().unwrap();
        
        let (w,h) = termion::terminal_size().unwrap();
        

        let cursor = (1, 1);
        return Self{
            origin_x: w/2, 
            origin_y: h/2,
            p_x: 1,
            p_y: 1,
            cursor,
            stdout,
            g,
        }
    }
    


    pub fn clear(&mut self) {
        write!(self.stdout, "{}{}",
        termion::cursor::Hide,
        termion::clear::All).unwrap();
        self.stdout.flush().unwrap();
    }
    
    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{} {} ", 
               termion::cursor::Goto(self.cursor.0-1,self.cursor.1),
               termion::cursor::Goto(self.cursor.0+1,self.cursor.1)).ok();
        self.stdout.flush().unwrap(); 
    }
    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}>{}<", 
               termion::cursor::Goto(self.cursor.0-1,self.cursor.1),
               termion::cursor::Goto(self.cursor.0+1,self.cursor.1)).ok();
        self.stdout.flush().unwrap(); 
    }
    pub fn position_cursor(&mut self) {
        self.cursor.0 = ((self.origin_x as i16) + self.p_x*4 + 1 - (OFFSET_X as i16)) as u16;
        self.cursor.1 = ((self.origin_y as i16) + self.p_y*2 + 1 - (OFFSET_Y as i16)) as u16;
    }
    pub fn move_cursor(&mut self,d: Direction) -> () {
        //delete the current cursor
        self.hide_cursor();

        match d {
            Direction::Up => {self.p_y -= 1},
            Direction::Left => {self.p_x -= 1},
            Direction::Down => {self.p_y += 1},
            Direction::Right => {self.p_x += 1},
        }
        //stay within boundaries
        self.p_x = self.p_x.clamp(0,2);
        self.p_y = self.p_y.clamp(0,2);
        


        return ();
    }
    pub fn game_loop(&mut self) {
        
        let mut bytes = stdin().bytes();
        loop {
            
           //show new cursor
            self.position_cursor();
            self.show_cursor();
            self.print_text();
            self.stdout.flush().unwrap();
            //get the next byte
            let b = bytes.next().unwrap().unwrap();

            //depending on the byte read ...
            let _ = match b {
                //quit
                b'q' => return,

                //write something to the buffer
                b'w' | b'k' => self.move_cursor(Direction::Up), 
                b'a' | b'h' => self.move_cursor(Direction::Left),
                b's' | b'j' => self.move_cursor(Direction::Down),
                b'd' | b'l' => self.move_cursor(Direction::Right), 

                b' ' | b'\r'=> self.g.play(self.p_y as usize, self.p_x as usize),
                _ => (),

            };
            //show board 
            self.draw_board();
            

        } 
    }


    pub fn print_text(&mut self) {
        use tictactui::State::*;
        use tictactui::Symbol::*;
        match self.g.current_board.state {
            Unfinished => match self.g.next_player {
               X => self.centered_print(format!("It's {}X{}'s turn", Fg(Blue),Reset)),
               O => self.centered_print(format!("It's {}O{}'s turn", Fg(Red),Reset)),

            }, 
            Draw => self.centered_print("Game is a draw".to_string()),
            Winner(s) => match s {
                X => self.centered_print(format!("The winner is {}X{}",Fg(Blue),Reset)),
                O => self.centered_print(format!("The winner is {}O{}",Fg(Red) ,Reset)),
            },
        }
    }

    pub fn centered_print(&mut self, text: String) {
        
        //calculate start of text at middle
        let text_offset = 7;

        write!(self.stdout, "{}                                                      ",
               termion::cursor::Goto(0,self.origin_y + 7 - OFFSET_Y)
              ).unwrap();
        self.stdout.flush().unwrap();
        
        write!(self.stdout, "{}{}",
               termion::cursor::Goto(self.origin_x - text_offset, self.origin_y + 7 - OFFSET_Y),
               text
               ).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn draw_board(&mut self){
        //write will put the following into a buffer
        write!(self.stdout, 
               //"{}{}\
               //{} {} ┃ {} ┃ {} 
               //{}━━━╋━━━╋━━━
               //{} {} ┃ {} ┃ {} 
               //{}━━━╋━━━╋━━━
               //{} {} ┃ {} ┃ {} \
               //", 
               "\
               {} {} ┃ {} ┃ {} 
               {}━━━╋━━━╋━━━
               {} {} ┃ {} ┃ {} 
               {}━━━╋━━━╋━━━
               {} {} ┃ {} ┃ {} \
               ", 
               termion::cursor::Goto(self.origin_x - OFFSET_X, self.origin_y + 1 - OFFSET_Y),
               &self.g.at(0,0),
               &self.g.at(0,1),
               &self.g.at(0,2),
               termion::cursor::Goto(self.origin_x - OFFSET_X,self.origin_y + 2 - OFFSET_Y),
               termion::cursor::Goto(self.origin_x - OFFSET_X, self.origin_y + 3 - OFFSET_Y),
               &self.g.at(1,0),
               &self.g.at(1,1),
               &self.g.at(1,2),
               termion::cursor::Goto(self.origin_x - OFFSET_X,self.origin_y + 4 - OFFSET_Y),
               termion::cursor::Goto(self.origin_x - OFFSET_X,self.origin_y + 5 - OFFSET_Y),
               &self.g.at(2,0),
               &self.g.at(2,1),
               &self.g.at(2,2),



               ).unwrap();

        self.stdout.flush().unwrap();    

    }

}





//     ┃   ┃ 
//  ━━━╋━━━╋━━━
//     ┃   ┃ 
//  ━━━╋━━━╋━━━
//     ┃   ┃
