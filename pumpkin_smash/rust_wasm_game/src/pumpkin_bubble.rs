use serde::{Deserialize, Serialize};
use rand::Rng;
use std::ops::Range;

#[derive(Serialize, Deserialize, Clone)]
pub struct PumpkinBubble {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: String,
}

impl PumpkinBubble {
    pub fn new_random(x_range: Range<f64>, y_range: Range<f64>, radius_range: Range<f64>) -> PumpkinBubble {
        let mut rng = rand::thread_rng();
        let bubble = PumpkinBubble {
            x: rng.gen_range(x_range),
            y: rng.gen_range(y_range),
            radius: rng.gen_range(radius_range),
            color: "#FFA500".to_string(),
        };
        println!("New bubble created at ({}, {}) with radius {}", bubble.x, bubble.y, bubble.radius);
        bubble
    }

    pub fn is_hit(&self, point_x: f64, point_y: f64) -> bool {
        let dx = point_x - self.x;
        let dy = point_y - self.y;
        let hit = (dx * dx + dy * dy) <= self.radius * self.radius;
        if hit {
            println!("Bubble at ({}, {}) with radius {} was hit", self.x, self.y, self.radius);
        }
        hit
    }
}
