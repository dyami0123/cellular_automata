use crate::shapes::appearance::Appearance;
use crate::shapes as shapes;
use ggez:: GameError;
use ggez::graphics::{self, Color};
use ggez::Context;
use ggez::glam::*;
use crate::shapes::shape::Shape;
use crate::rendering::renderer::Renderer;
use std::fmt::Result as GenericResult;

pub struct GGEZShapeRenderer<'a> {
    pub canvas :  & 'a mut  graphics::Canvas,
    pub ctx : &'a Context,
}
impl GGEZShapeRenderer<'_> {

    pub fn draw_shape(
        mut self,
        shape: &dyn shapes::shape::Shape,
    ) -> GenericResult {
        match shape.shape_type() {
            shapes::shape_types::ShapeType::SinglePoint => {
                if let Some(point_shape) = shape.as_any().downcast_ref::<shapes::single_point_shape::SinglePointShape>() {
                    return self.draw_point(point_shape.points()[0], point_shape.size(), point_shape.appearance());
                } else {
                    // Handle the case where downcasting fails
                    return Err(std::fmt::Error);
                }
            }
            shapes::shape_types::ShapeType::SingleLine => {
                if let Some(line_shape) = shape.as_any().downcast_ref::<shapes::single_line_shape::SingleLineShape>() {
                    return self.draw_line(line_shape.points()[0], line_shape.points()[1], line_shape.size(), line_shape.appearance());
                } else {
                    // Handle the case where downcasting fails
                    return Err(std::fmt::Error);
                }
            }
            shapes::shape_types::ShapeType::Rectangle => {
                if let Some(rectangle_shape) = shape.as_any().downcast_ref::<shapes::rectangle_shape::RectangleShape>() {
                    return self.draw_rectangle(rectangle_shape.points()[0], rectangle_shape.points()[1], rectangle_shape.size(), rectangle_shape.appearance());
                } else {
                    // Handle the case where downcasting fails
                    return Err(std::fmt::Error);
                }
            }
            shapes::shape_types::ShapeType::Triangle => {
                if let Some(triangle_shape) = shape.as_any().downcast_ref::<shapes::triangle_shape::TriangleShape>() {
                    return self.draw_triangle(triangle_shape.points()[0], triangle_shape.points()[1], triangle_shape.points()[2], triangle_shape.size(), triangle_shape.appearance());
                } else {
                    // Handle the case where downcasting fails
                    return Err(std::fmt::Error);
                }
            }
        }
    }
    

    fn unwrap_and_draw(
        mut self,
        shape_game_result: Result<graphics::Mesh, GameError>,
    ) -> GenericResult {

        if let Ok(line) = shape_game_result {
            self.canvas.draw(&line, Vec2::new(0.0, 0.0));
            return Ok(());
        }
        else {
            return Err(std::fmt::Error);
        }
    }

}

impl Renderer for GGEZShapeRenderer<'_> {

    fn draw_point(
            mut self,
            start: (f32, f32),
            width: f32,
            appearance: Appearance
        ) -> GenericResult {
        let circle_game_result = graphics::Mesh::new_circle(
            self.ctx,
            graphics::DrawMode::fill(),
            Vec2::new(start.0, start.1),
            width,
            2.0,
            Color::from(appearance.color),
        );

        return self.unwrap_and_draw(circle_game_result);
    }

    fn draw_line(
        mut self,
        start: (f32, f32),
        end: (f32, f32),
        size: f32,
        appearance: Appearance,
        
    ) -> GenericResult {
        let line_game_result = graphics::Mesh::new_line(
            self.ctx,
            &[Vec2::new(start.0, start.1), Vec2::new(end.0, end.1)],
            size,
            Color::from(appearance.color),
        );

        return self.unwrap_and_draw(line_game_result);
    }

    fn draw_circle(
            mut self,
            start: (f32, f32),
            radius: f32,
            width: f32,
            appearance: Appearance
        ) -> GenericResult {
        let circle_game_result = graphics::Mesh::new_circle(
            self.ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            radius,
            width,
            Color::from(appearance.color),
        );

        return self.unwrap_and_draw(circle_game_result);
    }


    fn draw_rectangle(
        mut self,
        start: (f32, f32),
        end: (f32, f32),
        width: f32,
        appearance: Appearance
    ) -> GenericResult {
        //FIXME: edge case if start is greater than end

        let rectangle_game_result = graphics::Mesh::new_rectangle(
            self.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                start.0,
                start.1,
                end.0 - start.0,
                end.1 - start.1,
            ),
            Color::from(appearance.color),
        );

        return self.unwrap_and_draw(rectangle_game_result);
    }

    fn draw_triangle(
            mut self,
            a: (f32, f32),
            b: (f32, f32),
            c: (f32, f32),
            width: f32,
            appearance: Appearance
        ) -> GenericResult {
        let triangle_game_result = graphics::Mesh::new_polygon(
            self.ctx,
            graphics::DrawMode::fill(),
            &[
                Vec2::new(a.0, a.1),
                Vec2::new(b.0, b.1),
                Vec2::new(c.0, c.1),
            ],
            Color::from(appearance.color),
        );

        return self.unwrap_and_draw(triangle_game_result);
    }

}