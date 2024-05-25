

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ShapeType {
    SinglePoint,
    SingleLine,
    Triangle,
    Rectangle,
}
// curl -X POST 127.0.0.1:3000 \
// -H "Content-Type: application/json" \
// -d '{
//     "shape_id": "SinglePoint",
//     "points": [
//         [0.0, 0.0]
//     ],
//     "size": 10.0,
//     "appearance": {
//         "color": {
//             "r": 1.0,
//             "g": 0.0,
//             "b": 0.0,
//             "a": 1.0
//         }
//     }
// }'