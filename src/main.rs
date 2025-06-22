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


use std::sync::Arc;
use wgpu::{Adapter, Device, Instance, Queue, Surface, TextureFormat};
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




struct App {
  window: Option<Arc<Window>>,
  gfx_state: Option<GfxState>
}

impl App {
  fn new() -> Self {
    Self {
      window: None,
      gfx_state: None
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
  size: PhysicalSize<u32>
}

impl GfxState {
  async fn setup(window: Arc<Window>) -> Self {
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
    Self {
      instance,
      surface,
      size,
      adapter,
      device,
      queue,
      surface_fmt
    }
  }
}




struct BodiesState {
  bodies: Vec<Body>
}

struct Body {}


fn main() {
  env_logger::init();
  let mut app = App::new();
  let event_loop = EventLoop::new().unwrap();
  event_loop.run_app(&mut app);
}
