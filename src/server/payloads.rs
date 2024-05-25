use crate::shapes::appearance::Appearance;
use crate::shapes::shape_types::ShapeType;
use serde::{Deserialize, Serialize};
use std::vec::IntoIter;

pub trait Payload {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShapePayload {
    pub shape_id: ShapeType,
    pub points: Vec<(f32, f32)>,
    pub size: f32,
    pub appearance: Appearance,
}

impl Payload for ShapePayload {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultipleShapesPayload {
    pub shapes: Vec<ShapePayload>,
}

// Implement IntoIterator for MultipleShapesPayload
impl IntoIterator for MultipleShapesPayload {
    type Item = ShapePayload;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.shapes.into_iter()
    }
}

impl Payload for MultipleShapesPayload {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClearPayload {
    pub clear: bool,
}

impl Payload for ClearPayload {}
