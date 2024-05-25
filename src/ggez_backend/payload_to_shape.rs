use crate::server::payloads::ShapePayload;
use crate::shapes;

// define an error type for incorrect number of points
#[derive(Debug)]
pub enum Error {
    IncorrectNumberOfPoints,
}

pub fn payload_to_shape(payload: ShapePayload) -> Result<Box<dyn shapes::shape::Shape>, Error> {
    match payload.shape_id {
        shapes::shape_types::ShapeType::SingleLine => {
            if payload.points.len() != 2 {
                println!(
                    "Incorrect number of points for SingleLine shape: {:?}",
                    payload.points
                );
                return Err(Error::IncorrectNumberOfPoints);
            }

            let start = payload.points[0];
            let end = payload.points[1];
            let size = payload.size;
            let appearance = payload.appearance;

            let shape = shapes::single_line_shape::SingleLineShape {
                start,
                end,
                size,
                appearance,
            };
            Ok(Box::new(shape))
        }
        shapes::shape_types::ShapeType::SinglePoint => {
            if payload.points.len() != 1 {
                println!(
                    "Incorrect number of points for SinglePoint shape: {:?}",
                    payload.points
                );
                return Err(Error::IncorrectNumberOfPoints);
            }

            let point = payload.points[0];
            let size = payload.size;
            let appearance = payload.appearance;

            let shape = shapes::single_point_shape::SinglePointShape {
                point,
                size,
                appearance,
            };
            Ok(Box::new(shape))
        }
        shapes::shape_types::ShapeType::Rectangle => {
            if payload.points.len() != 2 {
                println!(
                    "Incorrect number of points for Rectangle shape: {:?}",
                    payload.points
                );
                return Err(Error::IncorrectNumberOfPoints);
            }

            let start = payload.points[0];
            let end = payload.points[1];
            let size = payload.size;
            let appearance = payload.appearance;

            let shape = shapes::rectangle_shape::RectangleShape {
                start,
                end,
                size,
                appearance,
            };
            Ok(Box::new(shape))
        }
        shapes::shape_types::ShapeType::Triangle => {
            // verify that the payload has the correct number of points without panicking
            if payload.points.len() != 3 {
                println!(
                    "Incorrect number of points for Triangle shape: {:?}",
                    payload.points
                );
                return Err(Error::IncorrectNumberOfPoints);
            }

            let a = payload.points[0];
            let b = payload.points[1];
            let c = payload.points[2];
            let size = payload.size;
            let appearance = payload.appearance;

            let shape = shapes::triangle_shape::TriangleShape {
                a,
                b,
                c,
                size,
                appearance,
            };
            return Ok(Box::new(shape));
        }
    }
}
