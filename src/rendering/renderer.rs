use std::fmt::Result;
use crate::shapes::appearance::Appearance;



pub trait Renderer {

    fn draw_point(
        self,
        start: (f32, f32),
        width: f32,
        appearance: Appearance
    ) -> Result;

    fn draw_circle(
        self,
        start: (f32, f32),
        radius: f32,
        width: f32,
        appearance: Appearance
    ) -> Result;

    fn draw_line(
        self,
        start: (f32, f32),
        end: (f32, f32),
        width: f32,
        appearance: Appearance
    ) -> Result;

    fn draw_rectangle(
        self,
        start: (f32, f32),
        end: (f32, f32),
        width: f32,
        appearance: Appearance
    ) -> Result;

    fn draw_triangle(
        self,
        a: (f32, f32),
        b: (f32, f32),
        c: (f32, f32),
        width: f32,
        appearance: Appearance
    ) -> Result;

}