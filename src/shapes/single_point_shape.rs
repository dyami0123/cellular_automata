
use crate::shapes::shape::Shape;
use crate::shapes::appearance::Appearance;
use serde::{Serialize, Deserialize};
use crate::shapes::shape_types::ShapeType;
use std::any::Any;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SinglePointShape {
    pub point: (f32, f32),
    pub size: f32,
    pub appearance: Appearance,
}

impl Shape for SinglePointShape {
    fn points(&self) -> Vec<(f32, f32)> {
        vec![self.point]
    }

    fn size(&self) -> f32 {
        self.size
    }

    fn appearance(&self) -> Appearance {
        self.appearance.clone()
    }

    fn shape_type(&self) -> ShapeType {
        ShapeType::SinglePoint
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
