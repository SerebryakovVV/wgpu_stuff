// use std::sync::Arc;
// use winit::{
//   application::ApplicationHandler,
//   event::WindowEvent,
//   event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
//   window::{Window, WindowId}
// };



// struct State {
//   window: Arc<Window>,
//   device: wgpu::Device,
//   queue: wgpu::Queue,
//   size: winit::dpi::PhysicalSize<u32>,
//   surface: wgpu::Surface<'static>,
//   surface_format: wgpu::TextureFormat
// }



// impl State {
//   async fn new(window: Arc<Window>) -> State {
//     let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
//     let adapter = instance.request_adapter(
//       &wgpu::RequestAdapterOptions::default()
//     ).await.unwrap();
//     let (device, queue) = adapter.request_device(
//       &wgpu::DeviceDescriptor::default()
//     ).await.unwrap();
//     let size = window.inner_size();
//     let surface = instance.create_surface(window.clone()).unwrap();
//     let cap = surface.get_capabilities(&adapter);
//     let surface_format = cap.formats[0];
//     let state = State {
//       window,
//       device,
//       queue,
//       size,
//       surface,
//       surface_format
//     };
//     state.configure_surface();
//     state
//   }

//   fn get_window(&self) -> &Window {
//     &self.window
//   }

//   fn configure_surface(&self) {
//     let surface_config = wgpu::SurfaceConfiguration {
//       usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//       format: self.surface_format,
//       view_formats: vec![self.surface_format.add_srgb_suffix()],
//       alpha_mode: wgpu::CompositeAlphaMode::Auto,
//       width: self.size.width,
//       height: self.size.height,
//       desired_maximum_frame_latency: 2,
//       present_mode: wgpu::PresentMode::AutoVsync
//     };
//     self.surface.configure(&self.device, &surface_config);
//   }

//   fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//     self.size = new_size;
//     println!("resizing");
//     self.configure_surface();
//   }

//   fn render(&mut self) {
//     let surface_texture = self
//                          .surface
//                          .get_current_texture()
//                          .expect("Failed to acquire next swapchain texture");
//     let texture_view 
//       = surface_texture
//        .texture
//        .create_view(
//           &wgpu::TextureViewDescriptor {
//             format: Some(self.surface_format.add_srgb_suffix()),
//             ..Default::default()
//           }
//        );

//     let mut encoder = self.device.create_command_encoder(&Default::default());
//     let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//       label: None,
//       color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//         view: &texture_view,
//         // depth_slice: None,
//         resolve_target: None,
//         ops: wgpu::Operations {
//           load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
//           store: wgpu::StoreOp::Store
//         }
//       })],
//       depth_stencil_attachment: None,
//       timestamp_writes: None,
//       occlusion_query_set: None
//     });
//     drop(renderpass);
//     self.queue.submit([encoder.finish()]);
//     self.window.pre_present_notify();
//     surface_texture.present();
//   }
// }


// #[derive(Default)]
// struct App {
//   state: Option<State>
// }

// impl ApplicationHandler for App {
//   fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//       let window = Arc::new(
//         event_loop.create_window(Window::default_attributes()).unwrap()
//       );
//       let state = pollster::block_on(State::new(window.clone()));
//       self.state = Some(state);
//       window.request_redraw();
//   }

//   fn window_event(
//           &mut self,
//           event_loop: &ActiveEventLoop,
//           window_id: WindowId,
//           event: WindowEvent,
//       ) {
//       let state = self.state.as_mut().unwrap();
//       match event {
//         WindowEvent::CloseRequested => {
//           println!("The close button was pressed.");
//           event_loop.exit();
//         },
//         WindowEvent::RedrawRequested => {
//           state.render();
//           // state.get_window().request_redraw();
//         },
//         WindowEvent::Resized(size) => {
//           state.resize(size);
//         },
//         _ => ()
//       }
//   }
// }


// pub fn run_example_window() {
//   env_logger::init();
//   let event_loop = EventLoop::new().unwrap();
//   event_loop.set_control_flow(ControlFlow::Poll);
//   let mut app = App::default();
//   event_loop.run_app(&mut app).unwrap();
// }
#![no_main]