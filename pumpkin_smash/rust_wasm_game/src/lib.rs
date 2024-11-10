```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod pumpkin_bubble;
use pumpkin_bubble::PumpkinBubble;

#[wasm_bindgen]
pub struct PumpkinSmashGame {
    bubbles: HashMap<u32, PumpkinBubble>, // HashMap to store bubbles with unique IDs
    harvest_coins: u32, // Total number of harvest coins earned by the player
    next_id: u32, // Counter to assign unique IDs to each bubble
}

#[wasm_bindgen]
impl PumpkinSmashGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PumpkinSmashGame {
        console_log!("Creating a new PumpkinSmashGame");
        PumpkinSmashGame {
            bubbles: HashMap::new(),
            harvest_coins: 0,
            next_id: 0,
        }
    }

    // Function to spawn initial bubbles in the game
    pub fn spawn_bubbles(&mut self) {
        console_log!("Spawning bubbles");
        for _ in 0..20 {
            let id = self.next_id;
            self.next_id += 1;
            self.bubbles.insert(id, PumpkinBubble::new_random()); // Insert a new randomly generated bubble
            console_log!("Spawned bubble with ID: {}", id);
        }
    }

    // Handle tap events to check if a bubble is hit
    pub fn on_tap(&mut self, x: f64, y: f64) -> String {
        console_log!("Handling tap at coordinates: x={}, y={}", x, y);
        let mut hit_bubble_ids = vec![];
        for (id, bubble) in &self.bubbles {
            if bubble.is_hit(x, y) { // Check if the bubble was hit
                hit_bubble_ids.push(*id);
                console_log!("Bubble with ID: {} was hit", id);
            }
        }

        for id in hit_bubble_ids {
            self.bubbles.remove(&id); // Remove the hit bubble from the HashMap
            console_log!("Removed bubble with ID: {}", id);
            self.reward_harvest_coins(); // Reward the player with coins
        }

        self.serialize_bubbles() // Return the updated list of bubbles in JSON format
    }

    // Reward the user with HRVST coins when they pop a bubble
    fn reward_harvest_coins(&mut self) {
        self.harvest_coins += 10; // Increase the user's HRVST coin balance by 10
        console_log!("Rewarded 10 HRVST coins. Total: {}", self.harvest_coins);
    }

    // Get the current harvest coins balance
    pub fn get_harvest_coins(&self) -> u32 {
        console_log!("Getting harvest coins: {}", self.harvest_coins);
        self.harvest_coins
    }

    // Serialize bubbles for JavaScript interop
    fn serialize_bubbles(&self) -> String {
        let serialized = serde_json::to_string(&self.bubbles).unwrap_or_else(|_| "{}".to_string()); // Convert bubbles HashMap to JSON string
        console_log!("Serialized bubbles: {}", serialized);
        serialized
    }
}
```
