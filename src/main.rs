mod tetromino;
mod board;
mod input;
mod game;

use macroquad::prelude::*;
use game::Game;

const COLS: usize = 10;
const ROWS: usize = 20;
const CELL_SIZE: f32 = 30.0;

#[macroquad::main("Rust Tetris")]
async fn main() {
    let mut game = Game::new();
    game.run().await;
}
