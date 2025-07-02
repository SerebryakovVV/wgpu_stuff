use crate::model::{Model, Vertex};

pub fn make_cube() -> Model {
  Model {
    vertices: vec![
      Vertex {position: [-0.5, -0.5, -0.5], color: [0.0, 0.0, 0.0]},
      Vertex {position: [0.5, -0.5, -0.5], color: [100.0, 0.0, 0.0]},
      Vertex {position: [0.5, 0.5, -0.5], color: [0.0, 0.0, 0.0]},
      Vertex {position: [-0.5, 0.5, -0.5], color: [0.0, 60.0, 0.0]},
      Vertex {position: [-0.5, -0.5, 0.5], color: [0.0, 0.0, 0.0]},
      Vertex {position: [0.5, -0.5, 0.5], color: [0.0, 0.0, 70.0]},
      Vertex {position: [0.5, 0.5, 0.5], color: [0.0, 0.0, 0.0]},
      Vertex {position: [-0.5, 0.5, 0.5], color: [0.0, 0.0, 0.0]},
      // Vertex {position: [-0.5, -0.5, -0.5], color: [100.0, 100.0, 100.0]},
      // Vertex {position: [0.5, -0.5, -0.5], color: [100.0, 100.0, 100.0]},
      // Vertex {position: [0.5, 0.5, -0.5], color: [100.0, 100.0, 100.0]},
      // Vertex {position: [-0.5, 0.5, -0.5], color: [100.0, 100.0, 100.0]},
      // Vertex {position: [-0.5, -0.5, 0.5], color: [100.0, 100.0, 100.0]},
      // Vertex {position: [0.5, -0.5, 0.5], color: [100.0, 100.0, 100.0]},
      // Vertex {position: [0.5, 0.5, 0.5], color: [100.0, 100.0, 100.0]},
      // Vertex {position: [-0.5, 0.5, 0.5], color: [100.0, 100.0, 100.0]},
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

  //       0, 1, 2,  2, 3, 0,   // back face
  // 1, 5, 6,  6, 2, 1,   // right face
  // 5, 4, 7,  7, 6, 5,   // front face
  // 4, 0, 3,  3, 7, 4,   // left face
  // 3, 2, 6,  6, 7, 3,   // top face
  // 4, 5, 1,  1, 0, 4    // bottom face
    ]
  }
}


