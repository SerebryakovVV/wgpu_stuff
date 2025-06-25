// first of all we create a window
// for the window we need an event loop
// event loop has the run_app function that takes a struct that implements ApplicationHandler trait
// we create an App struct that implements this trait, and pass it to the run_app
// the trait requires two methods, and has some number of optinal methods
// the window_event method on ApplicationHandler trait handles all the events of the window

// now with window created we make an Instance, it takes env variables to configure itself
// with this instance we can create a surface and an adapter
// with adapter we can create device and a queue




// triangle fan, shader masking
// instancing
// shader module
// pipeline layout
// pipeline 
// render method update

mod model;
use model::Model;
// use model;

use std::{mem, sync::Arc};
use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, Adapter, Buffer, Device, Instance, Queue, RenderPipeline, ShaderModule, Surface, TextureFormat};
use winit::{
  application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, event_loop::{
    self,
    ActiveEventLoop,
    EventLoop
  }, window::{
    Window, 
    WindowAttributes,
    WindowId
  }
};




struct Body {
  vertex_data: Vec<Vertex>,
  index_data: Vec<u16>
}

#[repr(C)]
#[derive(Pod, Clone, Copy, Zeroable)]
struct Vertex {
  position: [f32; 3],
  color: [f32; 3]
}


struct App {
  window: Option<Arc<Window>>,
  gfx_state: Option<GfxState>,
  bodies: Vec<Body>
}

impl App {
  fn new() -> Self {

    // let vertex_data = vec![
    //   Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 0.0] }, 
    //   Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 0.0] }, 
    //   Vertex { position: [ 0.5,  0.5, 0.0], color: [0.0, 0.0, 0.0] }, 
    //   Vertex { position: [-0.5,  0.5, 0.0], color: [0.0, 0.0, 0.0] }, 
    //   Vertex { position: [-1.0,  0.5, 0.0], color: [0.0, 0.0, 0.0] }, 
    // ];
    // let index_data = vec![
    //   0, 1, 2, 
    //   2, 3, 0, 
    //   3, 4, 0
    // ];




    let vertex_data = vec![
      Vertex { position: [ 0.0, 0.0, 0.0], color: [0.0, 0.0, 0.0] }, 
      Vertex { position: [ 0.10, 0.40, 0.0], color: [0.0, 0.0, 0.0] }, 
      Vertex { position: [ 0.05,  0.48, 0.0], color: [0.0, 0.0, 0.0] }, 
      Vertex { position: [ 0.0,  0.5, 0.0], color: [0.0, 0.0, 0.0] }, 
    ];
    let index_data = vec![
      0, 1, 2, 
      2, 3, 0, 
    ];


    // let a = Model::from_obj("src/teapot.obj").unwrap();
    // let mut vertex_data = Vec::new();
    // for v in a.vertices.iter() {
    //   vertex_data.push(Vertex { position: *v, color: [0.0, 0.0, 0.0] });
    // }
    // let index_data = a.indices.to_vec();





    Self {
      window: None,
      gfx_state: None,
      bodies: vec![Body {
        vertex_data,
        index_data
      }]
    }
  }

  fn render(&mut self) {
    let gfx_state = self.gfx_state.as_mut().unwrap();
    let surface_texture = gfx_state.surface.get_current_texture().unwrap();
    let texture_view = surface_texture.texture.create_view(&wgpu::wgt::TextureViewDescriptor {format: Some(gfx_state.surface_fmt) ,..Default::default()});    
    let mut encoder = gfx_state.device.create_command_encoder(&Default::default());
    let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None, 
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &texture_view,
        resolve_target: None,
        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), store: wgpu::StoreOp::Store }
      })], 
      depth_stencil_attachment: None, 
      timestamp_writes: None, 
      occlusion_query_set: None 
    });
    renderpass.set_pipeline(&gfx_state.render_pipeline);
    renderpass.set_vertex_buffer(0, gfx_state.vertex_buffer.slice(..));
    renderpass.set_index_buffer(gfx_state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    renderpass.draw_indexed(0..gfx_state.index_buffer.size() as u32 / std::mem::size_of::<u16>() as u32, 0, 0..1);

    drop(renderpass);
    gfx_state.queue.submit([encoder.finish()]);
    self.window.as_ref().unwrap().pre_present_notify();
    surface_texture.present();
  }

  fn configure_surface(&mut self) {
    let gfx_state = self.gfx_state.as_mut().unwrap();
    gfx_state.size = self.window.as_mut().unwrap().inner_size();
    let config = wgpu::SurfaceConfiguration {
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
      desired_maximum_frame_latency: 2,
      format: gfx_state.surface_fmt,
      height: gfx_state.size.height,
      width: gfx_state.size.width,
      present_mode: wgpu::PresentMode::AutoVsync,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      view_formats: vec![gfx_state.surface_fmt.add_srgb_suffix()]
    };
    gfx_state.surface.configure(&gfx_state.device, &config);
    // self.render();
  }
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    self.window = Some(
      Arc::new(
        event_loop.create_window(
          WindowAttributes::default()
        ).unwrap()
      )
    );
    self.gfx_state = Some(
      pollster::block_on(
        GfxState::setup(
          self
          .window
          .as_ref()
          .unwrap()
          .clone()
          ,
          &self.bodies
        )
      )
    );
    self.configure_surface();
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    window_id: WindowId,
    event: WindowEvent,
  ) {
    match event {
      WindowEvent::CursorEntered { .. } => println!("Cursor entered"),
      WindowEvent::Resized(size) => {
        if size.height > 0 && size.width > 0 {
          self.configure_surface();
        }
        println!("Resized to {:#?}", size);
      },
      WindowEvent::CloseRequested => {
        println!("Close requested");
        event_loop.exit()
      },
      WindowEvent::RedrawRequested => self.render(),
      _ => {
        // println!("Rest of the events");
      }
    }
  }
}


struct GfxState {
  instance: Instance,
  surface: Surface<'static>,
  adapter: Adapter,
  device: Device,
  queue: Queue,
  surface_fmt: TextureFormat,
  size: PhysicalSize<u32>,
  vertex_buffer: Buffer,
  index_buffer: Buffer,
  shader: ShaderModule,
  render_pipeline: RenderPipeline
}

impl GfxState {
  async fn setup(window: Arc<Window>, bodies: &Vec<Body>) -> Self {
		let instance = Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
		let surface = instance.create_surface(window.clone()).unwrap();
    let size = window.inner_size();
		let adapter = instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				compatible_surface: Some(&surface),
				..Default::default()
			}
		).await.unwrap();
		let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor{
        required_features: wgpu::Features::POLYGON_MODE_LINE,                 // changed
        ..Default::default()
      }
    ).await.unwrap();
    let surface_fmt = surface.get_capabilities(&adapter).formats[0];

    // creating the index and vertex buffers here, probably

    

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex buffer"),
      usage: wgpu::BufferUsages::VERTEX,
      contents: bytemuck::cast_slice(&bodies[0].vertex_data)
    });    

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Index buffer"),
      usage: wgpu::BufferUsages::INDEX,
      contents: bytemuck::cast_slice(&bodies[0].index_data)
    });
    let shader = device.create_shader_module(wgpu::include_wgsl!("3d_model_render_shader.wgsl"));
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
      label: Some("pipeline layout label"), 
      bind_group_layouts: &[], 
      push_constant_ranges: &[] 
    });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
      label: Some("Render pipeline"), 
      layout: Some(&pipeline_layout), 
      vertex: wgpu::VertexState { 
        module: &shader, 
        entry_point: Some("vs_main"), 
        compilation_options: Default::default(), 
        buffers: &[
          wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
              wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3
              },
              wgpu::VertexAttribute {
                offset: 12,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
              },
            ]
          }
        ]
      }, 
      primitive: wgpu::PrimitiveState { 
        topology: wgpu::PrimitiveTopology::TriangleList, 
        front_face: wgpu::FrontFace::Ccw, 
        cull_mode: Some(wgpu::Face::Back), 
        polygon_mode: wgpu::PolygonMode::Line,                                                                 // changed
        ..Default::default()
      }, 
      depth_stencil: None, 
      multisample: wgpu::MultisampleState::default(), 
      fragment: Some(wgpu::FragmentState { 
        module: &shader, 
        entry_point: Some("fs_main"), 
        compilation_options: Default::default(), 
        targets: &[Some(wgpu::ColorTargetState { 
          format: surface_fmt, 
          blend: Some(wgpu::BlendState::REPLACE), 
          write_mask: wgpu::ColorWrites::ALL 
        })]
      }), 
      multiview: None, 
      cache: None
    });
    Self {
      instance,
      surface,
      size,
      adapter,
      device,
      queue,
      surface_fmt,
      vertex_buffer,
      index_buffer,
      shader,
      render_pipeline
    }
  }
}



fn main() {
  env_logger::init();
  let mut app = App::new();
  let event_loop = EventLoop::new().unwrap();
  event_loop.run_app(&mut app);
}
