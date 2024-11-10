#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use crate::PumpkinSmashGame;

#[wasm_bindgen]
pub fn init_game() -> PumpkinSmashGame {
    console_log!("Initializing game");
    let mut game = PumpkinSmashGame::new();
    game.spawn_bubbles(); // Spawn the initial bubbles
    game
}
