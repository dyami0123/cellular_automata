
use crate::shapes::appearance::Appearance;
use crate::shapes::shape_types::ShapeType;
use std::any::Any;

pub trait Shape:  Send + Sync + Any{
    fn points(&self) -> Vec<(f32, f32)>;
    fn size(&self) -> f32;
    fn appearance(&self) -> Appearance;
    fn shape_type(&self) -> ShapeType;
    fn as_any(&self) -> &dyn Any ;
 
}
