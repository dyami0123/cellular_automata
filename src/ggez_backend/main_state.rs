use crate::rendering::ggez_renderer::GGEZShapeRenderer;
use crate::server::endpoint_reciever_stack::EndpointRecieverStack;
use crate::shapes;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub struct MainState {
    shapes: Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>>,
    drawn_shape_ids: Vec<i32>,
    endpoint_reciever_stack: EndpointRecieverStack,
}

impl MainState {
    pub fn new(
        shapes: Arc<Mutex<HashMap<i32, Box<dyn shapes::shape::Shape>>>>,
        endpoint_reciever_stack: EndpointRecieverStack,
    ) -> Result<MainState, ggez::GameError> {
        let drawn_shape_ids = vec![];
        Ok(MainState {
            shapes,
            endpoint_reciever_stack,
            drawn_shape_ids,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_all_shapes(ctx)
    }
}

impl MainState {
    fn draw_all_shapes(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for shape in self.shapes.lock().unwrap().values() {
            let renderer = GGEZShapeRenderer {
                ctx,
                canvas: &mut canvas,
            };
            renderer.draw_shape(Box::as_ref(shape)).unwrap_or_else(|e| {
                tracing::error!("Error drawing shape: {:?}", e);
            });
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    #[allow(dead_code)] // this function is not used in the current implementation
    fn draw_skipping_shapes(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: graphics::Canvas;

        // if we have no shapes, assume we need to clear the canvas
        if self.shapes.lock().unwrap().len() == 0 {
            canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        } else {
            canvas = graphics::Canvas::from_frame(ctx, None);
        }

        let shapes = self.shapes.lock().unwrap();

        // create a list of shapes to draw by creating a set of drawn shape ids
        // and a set of all shape ids
        // and then subtracting the drawn shape ids from the all shape ids

        let mut all_shape_ids = vec![];
        for (id, _) in shapes.iter() {
            all_shape_ids.push(*id);
        }
        let all_shape_ids_set: std::collections::HashSet<i32> =
            all_shape_ids.iter().cloned().collect();
        let drawn_shape_ids_set: std::collections::HashSet<i32> =
            self.drawn_shape_ids.iter().cloned().collect();
        let shapes_to_draw_set = all_shape_ids_set.difference(&drawn_shape_ids_set);

        for id in shapes_to_draw_set {
            let renderer = GGEZShapeRenderer {
                ctx,
                canvas: &mut canvas,
            };
            renderer
                .draw_shape(Box::as_ref(&shapes[id]))
                .unwrap_or_else(|e| {
                    tracing::error!("Error drawing shape: {:?}", e);
                });

            self.drawn_shape_ids.push(*id);
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}
