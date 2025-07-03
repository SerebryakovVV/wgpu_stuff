use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Pod, Clone, Copy, Zeroable, Debug)]
pub struct Vertex {
  pub position: [f32; 3],
  pub color: [f32; 3]
}

#[derive(Debug)]
pub struct Model {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>
}


impl Model {
  pub fn from_obj(path: &str) -> anyhow::Result<Self> {
    let model = obj::Obj::load(path)?;
    let mut indices = Vec::new();
    for object in model.data.objects.iter() {
      for group in object.groups.iter() {
        for poly in group.polys.iter() {
          for index_tuple in poly.0.iter() {
            indices.push(index_tuple.0 as u16);
          }
        }
      }
    }
    let mut vertices = Vec::new();
    for v in model.data.position {
      vertices.push(Vertex {
        position: v,
        color: [0.0, 0.0, 0.0]
      });
    }
    Ok(Self {
      vertices,
      indices
    })
  }
}



#[test]
fn test_model_loading() {
  let a = Model::from_obj("models/teapot.obj");
  println!("{:#?}", a.unwrap());
}