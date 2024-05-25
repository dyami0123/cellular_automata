

use crate::shapes::shape::Shape;
use crate::shapes::appearance::Appearance;
use serde::{Serialize, Deserialize};
use crate::shapes::shape_types::ShapeType;
use std::any::Any;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriangleShape {
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub c: (f32, f32),
    pub size: f32,
    pub appearance: Appearance,
}

impl Shape for TriangleShape {
    fn points(&self) -> Vec<(f32, f32)> {
        vec![self.a, self.b, self.c]
    }

    fn size(&self) -> f32 {
        self.size
    }

    fn appearance(&self) -> Appearance {
        self.appearance.clone()
    }
    
    fn shape_type(&self) -> ShapeType {
        ShapeType::Triangle
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

