use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct PumpkinBubble {
    pub x: f64, // X-coordinate of the bubble
    pub y: f64, // Y-coordinate of the bubble
    pub radius: f64, // Radius of the bubble
    pub color: String, // Color of the bubble in a hex string format
}

impl PumpkinBubble {
    // Create a new bubble with random properties
    pub fn new_random() -> PumpkinBubble {
        let mut rng = rand::thread_rng();
        let bubble = PumpkinBubble {
            x: rng.gen_range(0.0..300.0), // Random x-coordinate within a range
            y: rng.gen_range(0.0..400.0), // Random y-coordinate within a range
            radius: 20.0 + rng.gen_range(0.0..15.0), // Random radius between 20 and 35
            color: "#FFA500".to_string(), // Set pumpkin color as orange in hex
        };
        console_log!("Created new bubble: x={}, y={}, radius={}", bubble.x, bubble.y, bubble.radius);
        bubble
    }

    // Check if a given point is within the bubble (used for detecting hits)
    pub fn is_hit(&self, point_x: f64, point_y: f64) -> bool {
        let hit = ((point_x - self.x).powi(2) + (point_y - self.y).powi(2)) <= self.radius.powi(2); // Calculate if the point is within the radius of the bubble
        if hit {
            console_log!("Bubble hit detected at: x={}, y={}", point_x, point_y);
        }
        hit
    }
}
