

use crate::shapes::shape::Shape;
use crate::shapes::appearance::Appearance;
use serde::{Serialize, Deserialize};
use crate::shapes::shape_types::ShapeType;
use std::any::Any;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SingleLineShape {
    pub start: (f32, f32),
    pub end: (f32, f32),
    pub size: f32,
    pub appearance: Appearance,
}

impl Shape for SingleLineShape {
    fn points(&self) -> Vec<(f32, f32)> {
        vec![self.start, self.end]
    }

    fn size(&self) -> f32 {
        self.size
    }

    fn appearance(&self) -> Appearance {
        self.appearance.clone()
    }

    fn shape_type(&self) -> ShapeType {
        ShapeType::SingleLine
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}