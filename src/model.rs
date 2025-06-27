use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Pod, Clone, Copy, Zeroable)]
pub struct Vertex {
  pub position: [f32; 3],
  pub color: [f32; 3]
}

pub struct Model {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>
}