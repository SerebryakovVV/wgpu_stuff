use bytemuck::{Pod, Zeroable};



#[derive(Debug)]
pub struct Model {
  vertices:Vec<[f32; 3]>,
  indices: Vec<u16>
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
    Ok(Self {
      vertices: model.data.position,
      indices
    })
  }
}



#[repr(C)]
#[derive(Pod, Clone, Copy, Zeroable)]
struct Vertex {
  position: [f32; 3],
  color: [f32; 3]
}



#[test]
fn test_model_loading() {
  let a = Model::from_obj("src/test.obj");
  println!("{:#?}", a.unwrap());
}