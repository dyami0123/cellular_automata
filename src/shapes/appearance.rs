use ggez::graphics::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Appearance {
    pub color: Color,
}
