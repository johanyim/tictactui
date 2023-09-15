use tictactui::Game;
use std::io;
mod ui;

use ui::UI;

fn main() -> Result<(), io::Error>{
    let mut g = Game::new();
    g.show();
    play(&mut g, 0, 0); 
    play(&mut g, 0, 1); 
    play(&mut g, 0, 2); 
    play(&mut g, 1, 0); 
    play(&mut g, 1, 1); 
    play(&mut g, 1, 2); 
    play(&mut g, 2, 0); 
    play(&mut g, 2, 1); 
    play(&mut g, 2, 2);
    
    g = Game::new();
    let mut user_in: UI = UI::start(g);
    //ui::draw();
    user_in.clear();
    user_in.draw_board();
    user_in.position_cursor();
    user_in.show_cursor();
    user_in.game_loop();
    return Ok(())
}

fn play(g : &mut Game, row: usize, col: usize) {
    g.play(row, col);
    println!("====================================");
    g.show_history();
    g.show();
}






