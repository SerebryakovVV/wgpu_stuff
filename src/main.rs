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

use std::sync::Arc;
use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, Adapter, Buffer, Device, Instance, Queue, Surface, TextureFormat};
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
  index_data: Vec<u8>
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
    let vertex_data = vec![
      Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] },
      Vertex { position: [ 1.0, -1.0,  1.0], color: [0.0, 1.0, 0.0] },
      Vertex { position: [ 1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
      Vertex { position: [-1.0,  1.0,  1.0], color: [1.0, 1.0, 0.0] },
      Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 1.0] },
      Vertex { position: [ 1.0, -1.0, -1.0], color: [0.0, 1.0, 1.0] },
      Vertex { position: [ 1.0,  1.0, -1.0], color: [1.0, 1.0, 1.0] },
      Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 0.0, 0.0] },
    ];
    let index_data = vec![
      0, 1, 2, 2, 3, 0,
      4, 5, 6, 6, 7, 4,
      0, 4, 7, 7, 3, 0,
      1, 5, 6, 6, 2, 1,
      3, 2, 6, 6, 7, 3,
      0, 1, 5, 5, 4, 0,
    ];
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
    let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None, 
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &texture_view,
        resolve_target: None,
        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::RED), store: wgpu::StoreOp::Store }
      })], 
      depth_stencil_attachment: None, 
      timestamp_writes: None, 
      occlusion_query_set: None 
    });
    drop(renderpass);
    gfx_state.queue.submit([encoder.finish()]);
    self.window.as_ref().unwrap().pre_present_notify();
    surface_texture.present();
  }

  fn configure_surface(&mut self) {
    let gfx_state = self.gfx_state.as_mut().unwrap();
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
      WindowEvent::Resized(size) => println!("Resized to {:#?}", size),
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
  index_buffer: Buffer
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
		let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor{..Default::default()}).await.unwrap();
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
      contents: &bodies[0].index_data
    });




    // let shader = device.create_shader_module();

    // let pipeline_layout = device.create_pipeline_layout(desc);

    // let render_pipeline = device.create_render_pipeline();

    
 



    Self {
      instance,
      surface,
      size,
      adapter,
      device,
      queue,
      surface_fmt,
      vertex_buffer,
      index_buffer
    }
  }
}



fn main() {
  env_logger::init();
  let mut app = App::new();
  let event_loop = EventLoop::new().unwrap();
  event_loop.run_app(&mut app);
}
