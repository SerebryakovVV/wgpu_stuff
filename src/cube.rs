use crate::model::{Model, Vertex};

pub fn make_cube() -> Model {
  Model {
    vertices: vec![
      Vertex {position: [-1.0, -1.0, -1.0], color: [0.0, 0.0, 0.0]},
      Vertex {position: [1.0, -1.0, -1.0], color: [0.0, 0.0, 0.0]},
      Vertex {position: [1.0, 1.0, -1.0], color: [0.0, 0.0, 0.0]},
      Vertex {position: [-1.0, 1.0, -1.0], color: [0.0, 0.0, 0.0]},
      Vertex {position: [-1.0, -1.0, 1.0], color: [0.0, 0.0, 0.0]},
      Vertex {position: [1.0, -1.0, 1.0], color: [0.0, 0.0, 0.0]},
      Vertex {position: [1.0, 1.0, 1.0], color: [0.0, 0.0, 0.0]},
      Vertex {position: [-1.0, 1.0, 1.0], color: [0.0, 0.0, 0.0]},
    ],
    indices: vec![
      0, 1, 3,
      3, 1, 2,
      1, 5, 2,
      2, 5, 6,
      5, 4, 6, 
      6, 4, 7,
      4, 0, 7, 
      7, 0, 3,
      3, 2, 7, 
      7, 2, 6,
      4, 5, 0, 
      0, 5, 1
    ]
  }
}