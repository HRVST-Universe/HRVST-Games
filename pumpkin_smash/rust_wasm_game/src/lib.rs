use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod pumpkin_bubble;
use pumpkin_bubble::PumpkinBubble;
use console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct PumpkinSmashGame {
    bubbles: HashMap<u32, PumpkinBubble>,
    harvest_coins: u32,
    next_id: u32,
    num_bubbles: u32,
}

#[wasm_bindgen]
impl PumpkinSmashGame {
    #[wasm_bindgen(constructor)]
    pub fn new(num_bubbles: u32) -> PumpkinSmashGame {
        // Initialize a new game with the given number of bubbles
        console_error_panic_hook::set_once();
        PumpkinSmashGame {
            bubbles: HashMap::new(),
            harvest_coins: 0,
            next_id: 0,
            num_bubbles,
        }
    }

    #[wasm_bindgen]
    pub fn spawn_bubbles(&mut self) {
        // Spawn the specified number of bubbles with random properties
        for _ in 0..self.num_bubbles {
            let id = self.next_id;
            self.next_id += 1;
            self.bubbles.insert(id, PumpkinBubble::new_random(0.0..300.0, 0.0..400.0, 20.0..35.0));
        }
    }

    #[wasm_bindgen]
    pub fn on_tap(&mut self, x: f64, y: f64) -> String {
        // Find and collect all bubbles that were hit
        let mut hit_bubble_ids = vec![];
        for (id, bubble) in &self.bubbles {
            if bubble.is_hit(x, y) {
                hit_bubble_ids.push(*id);
            }
        }

        // Remove the hit bubbles and reward the player
        for id in hit_bubble_ids {
            self.bubbles.remove(&id);
            self.reward_harvest_coins();
        }

        // Serialize the remaining bubbles to return as a JSON string
        serde_json::to_string(&self.bubbles).unwrap_or_else(|e| {
            alert(&format!("Error serializing bubbles: {}", e));
            "{}".to_string()
        })
    }

    #[wasm_bindgen]
    pub fn reward_harvest_coins(&mut self) {
        // Increase the harvest coin count by a fixed amount
        self.harvest_coins += 10;
    }

    #[wasm_bindgen]
    pub fn get_harvest_coins(&self) -> u32 {
        // Return the current number of harvest coins
        self.harvest_coins
    }
}
